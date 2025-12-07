//! # Coordinator - FROST 協議編排器
//!
//! 協調者負責編排整個 FROST 簽章流程：
//! 1. 收集簽署者的 Round 1 承諾
//! 2. 建立並分發 SigningPackage
//! 3. 收集簽章分片
//! 4. 聚合最終簽章
//! 5. 驗證簽章
//!
//! ## 安全性原則
//! - 協調者**永不持有**私鑰分片
//! - 協調者**永不接觸**秘密 nonces
//! - 協調者可以是不受信任的（它無法偽造簽章）

use crate::api::{CommitmentData, SessionId, SignatureShareData, SigningPackageData};
use crate::signer::Signer;
use dashmap::DashMap;
use frost_secp256k1 as frost;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

// ============================================================================
// 錯誤定義
// ============================================================================

#[derive(Debug, Error)]
pub enum CoordinatorError {
    #[error("Session {0} not found")]
    SessionNotFound(SessionId),

    #[error("Insufficient commitments: expected {expected}, got {actual}")]
    InsufficientCommitments { expected: usize, actual: usize },

    #[error("Insufficient signature shares: expected {expected}, got {actual}")]
    InsufficientShares { expected: usize, actual: usize },

    #[error("Failed to aggregate signature: {0}")]
    AggregationFailed(String),

    #[error("Signature verification failed: {0}")]
    VerificationFailed(String),

    #[error("Signer error: {0}")]
    SignerError(String),

    #[error("Invalid public key package")]
    InvalidPublicKeyPackage,

    #[error("Commitment deserialization failed: {0}")]
    CommitmentDeserializationFailed(String),

    #[error("Signature share deserialization failed: {0}")]
    ShareDeserializationFailed(String),
}

// ============================================================================
// Session 狀態管理
// ============================================================================

/// 簽章會話的狀態
#[derive(Debug, Clone)]
pub struct SessionState {
    /// Session ID
    pub session_id: SessionId,

    /// 要簽署的訊息
    pub message: Vec<u8>,

    /// Round 1 收集的承諾
    pub commitments: Vec<CommitmentData>,

    /// 建立時間（用於超時處理）
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SessionState {
    pub fn new(session_id: SessionId, message: Vec<u8>) -> Self {
        Self {
            session_id,
            message,
            commitments: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }
}

// ============================================================================
// Coordinator 結構
// ============================================================================

/// FROST 協調者
///
/// 協調者管理簽章流程的狀態，但不持有任何私鑰。
/// 它的主要職責是：
/// - 協調多個簽署者之間的通訊
/// - 聚合簽章分片
/// - 驗證最終簽章
pub struct Coordinator {
    /// 群組公鑰（用於驗證簽章）
    pubkey_package: frost::keys::PublicKeyPackage,

    /// 門檻值（最少需要的簽署者數量）
    threshold: u16,

    /// 當前活動的會話狀態
    sessions: Arc<DashMap<SessionId, SessionState>>,
}

impl Coordinator {
    // ========================================================================
    // 建構函數
    // ========================================================================

    /// 建立新的協調者實例
    ///
    /// # 參數
    /// - `pubkey_package`: 群組的公鑰套件（包含群組公鑰和所有簽署者的公鑰）
    /// - `threshold`: 生成簽章所需的最少簽署者數量
    pub fn new(pubkey_package: frost::keys::PublicKeyPackage, threshold: u16) -> Self {
        Self {
            pubkey_package,
            threshold,
            sessions: Arc::new(DashMap::new()),
        }
    }

    /// 獲取群組公鑰
    pub fn group_public_key(&self) -> &frost::VerifyingKey {
        self.pubkey_package.verifying_key()
    }

    // ========================================================================
    // Session 管理
    // ========================================================================

    /// 建立新的簽章會話
    pub fn create_session(&self, message: Vec<u8>) -> SessionId {
        let session_id = SessionId::new();
        let state = SessionState::new(session_id, message);

        self.sessions.insert(session_id, state);

        tracing::info!(
            session_id = %session_id,
            "Created new signing session"
        );

        session_id
    }

    /// 添加承諾到會話
    pub fn add_commitment(
        &self,
        session_id: SessionId,
        commitment: CommitmentData,
    ) -> Result<usize, CoordinatorError> {
        let mut session = self
            .sessions
            .get_mut(&session_id)
            .ok_or(CoordinatorError::SessionNotFound(session_id))?;

        session.commitments.push(commitment);
        let count = session.commitments.len();

        tracing::debug!(
            session_id = %session_id,
            commitments_count = count,
            "Added commitment to session"
        );

        Ok(count)
    }

    /// 獲取會話的簽章套件（用於 Round 2）
    pub fn get_signing_package(
        &self,
        session_id: SessionId,
    ) -> Result<SigningPackageData, CoordinatorError> {
        let session = self
            .sessions
            .get(&session_id)
            .ok_or(CoordinatorError::SessionNotFound(session_id))?;

        // 檢查是否收集了足夠的承諾
        if session.commitments.len() < self.threshold as usize {
            return Err(CoordinatorError::InsufficientCommitments {
                expected: self.threshold as usize,
                actual: session.commitments.len(),
            });
        }

        Ok(SigningPackageData {
            commitments: session.commitments.clone(),
            message: session.message.clone(),
        })
    }

    // ========================================================================
    // 完整簽章流程（高階 API）
    // ========================================================================

    /// 執行完整的簽章流程
    ///
    /// 這是一個高階方法，展示了如何使用協調者編排整個 FROST 流程。
    /// 在實際的 HTTP API 中，每個步驟會是獨立的端點。
    ///
    /// # 參數
    /// - `signers`: 參與簽署的簽署者列表（必須 >= threshold）
    /// - `message`: 要簽署的訊息
    ///
    /// # 返回
    /// - `Ok(Signature)`: 最終的群組簽章
    /// - `Err(CoordinatorError)`: 流程中的任何錯誤
    pub async fn orchestrate_signing(
        &self,
        signers: &[Arc<Signer>],
        message: &[u8],
    ) -> Result<frost::Signature, CoordinatorError> {
        tracing::info!(
            signer_count = signers.len(),
            threshold = self.threshold,
            "Starting FROST signing orchestration"
        );

        // 驗證簽署者數量
        if signers.len() < self.threshold as usize {
            return Err(CoordinatorError::InsufficientCommitments {
                expected: self.threshold as usize,
                actual: signers.len(),
            });
        }

        // 建立新的簽章會話
        let session_id = self.create_session(message.to_vec());

        // ====================================================================
        // Round 1: 並行收集所有簽署者的承諾
        // ====================================================================
        tracing::info!(session_id = %session_id, "Round 1: Collecting commitments");

        let round1_futures = signers.iter().map(|signer| {
            let signer = Arc::clone(signer);
            let session_id = session_id;

            // 為每個簽署者 spawn 一個異步任務
            tokio::spawn(async move {
                let commitment = signer
                    .commit(session_id)
                    .map_err(|e| CoordinatorError::SignerError(e.to_string()))?;

                Ok::<_, CoordinatorError>((signer.id(), commitment))
            })
        });

        // 等待所有承諾
        let round1_results = futures::future::join_all(round1_futures).await;

        // 收集承諾
        let mut commitments_map = HashMap::new();
        for result in round1_results {
            let (identifier, commitment) = result
                .map_err(|e| CoordinatorError::SignerError(e.to_string()))??;

            let commitment_data = CommitmentData {
                signer_id: u16::from(identifier),
                commitment: hex::encode(commitment.serialize()),
            };

            self.add_commitment(session_id, commitment_data)?;
            commitments_map.insert(identifier, commitment);
        }

        tracing::info!(
            session_id = %session_id,
            commitments_count = commitments_map.len(),
            "Round 1 complete: All commitments collected"
        );

        // ====================================================================
        // 建立簽章套件
        // ====================================================================
        let signing_package_data = self.get_signing_package(session_id)?;
        let signing_package = frost::SigningPackage::new(commitments_map, message);

        // ====================================================================
        // Round 2: 並行收集所有簽署者的簽章分片
        // ====================================================================
        tracing::info!(session_id = %session_id, "Round 2: Collecting signature shares");

        let round2_futures = signers.iter().map(|signer| {
            let signer = Arc::clone(signer);
            let session_id = session_id;
            let signing_package_data = signing_package_data.clone();

            tokio::spawn(async move {
                let signature_share = signer
                    .sign(session_id, &signing_package_data)
                    .map_err(|e| CoordinatorError::SignerError(e.to_string()))?;

                Ok::<_, CoordinatorError>((signer.id(), signature_share))
            })
        });

        // 等待所有簽章分片
        let round2_results = futures::future::join_all(round2_futures).await;

        // 收集簽章分片
        let mut signature_shares = HashMap::new();
        for result in round2_results {
            let (identifier, share) = result
                .map_err(|e| CoordinatorError::SignerError(e.to_string()))??;

            signature_shares.insert(identifier, share);
        }

        tracing::info!(
            session_id = %session_id,
            shares_count = signature_shares.len(),
            "Round 2 complete: All signature shares collected"
        );

        // ====================================================================
        // 聚合簽章
        // ====================================================================
        let group_signature = self.aggregate_signature(&signing_package, &signature_shares)?;

        tracing::info!(
            session_id = %session_id,
            signature = %hex::encode(group_signature.serialize()),
            "Signature aggregated successfully"
        );

        // ====================================================================
        // 驗證簽章
        // ====================================================================
        self.verify_signature(message, &group_signature)?;

        tracing::info!(
            session_id = %session_id,
            "Signature verified successfully"
        );

        // 清理會話
        self.sessions.remove(&session_id);

        Ok(group_signature)
    }

    // ========================================================================
    // 低階聚合與驗證方法
    // ========================================================================

    /// 聚合簽章分片為最終簽章
    ///
    /// # 參數
    /// - `signing_package`: 簽章套件（包含承諾和訊息）
    /// - `signature_shares`: 所有簽署者的簽章分片
    pub fn aggregate_signature(
        &self,
        signing_package: &frost::SigningPackage,
        signature_shares: &HashMap<frost::Identifier, frost::round2::SignatureShare>,
    ) -> Result<frost::Signature, CoordinatorError> {
        // 檢查分片數量
        if signature_shares.len() < self.threshold as usize {
            return Err(CoordinatorError::InsufficientShares {
                expected: self.threshold as usize,
                actual: signature_shares.len(),
            });
        }

        // 聚合簽章
        frost::aggregate(signing_package, signature_shares, &self.pubkey_package)
            .map_err(|e| CoordinatorError::AggregationFailed(format!("{:?}", e)))
    }

    /// 驗證簽章
    ///
    /// # 參數
    /// - `message`: 原始訊息
    /// - `signature`: 要驗證的簽章
    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &frost::Signature,
    ) -> Result<(), CoordinatorError> {
        self.pubkey_package
            .verifying_key()
            .verify(message, signature)
            .map_err(|e| CoordinatorError::VerificationFailed(format!("{:?}", e)))
    }

    // ========================================================================
    // 管理方法
    // ========================================================================

    /// 獲取活動會話數量
    pub fn active_sessions_count(&self) -> usize {
        self.sessions.len()
    }

    /// 清除指定會話
    pub fn clear_session(&self, session_id: &SessionId) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    /// 清除所有會話
    pub fn clear_all_sessions(&self) {
        self.sessions.clear();
    }
}

// ============================================================================
// 執行緒安全性
// ============================================================================

// Coordinator 自動實現 Send + Sync，因為：
// - frost::keys::PublicKeyPackage 實現了 Send + Sync
// - DashMap 實現了 Send + Sync
// - 所有基本類型（u16）都實現了 Send + Sync
// 因此不需要手動實現，編譯器會自動派生

// ============================================================================
// 測試
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // 可以加入單元測試，例如：
    // - 測試並發的多個會話
    // - 測試錯誤情況（不足的簽署者、無效的簽章等）
}
