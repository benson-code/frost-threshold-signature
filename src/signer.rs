//! # Signer Actor - 簽署者邏輯與狀態管理
//!
//! 此模組實作了獨立的簽署者邏輯，每個簽署者：
//! - 持有自己的金鑰分片 (KeyPackage)
//! - 管理 Nonce 狀態（防止重用）
//! - 提供 Round 1 (commit) 和 Round 2 (sign) 的介面
//!
//! ## 安全性考量
//! 1. **Nonce 一次性使用**: 每個 SessionId 的 Nonce 在 sign() 後會被立即銷毀
//! 2. **並發安全**: 使用 DashMap 允許多個並發的簽章會話
//! 3. **無狀態洩漏**: SecretNonces 永遠不會離開此模組

use crate::api::{SessionId, SigningPackageData};
use dashmap::DashMap;
use frost_secp256k1 as frost;
use rand::thread_rng;
use std::sync::Arc;
use thiserror::Error;

// ============================================================================
// 錯誤定義
// ============================================================================

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Session ID {0} not found - nonce may have been used or never generated")]
    SessionNotFound(SessionId),

    #[error("Failed to generate nonce commitments: {0}")]
    CommitmentGenerationFailed(String),

    #[error("Failed to generate signature share: {0}")]
    SignatureGenerationFailed(String),

    #[error("Invalid commitment format: {0}")]
    InvalidCommitment(String),

    #[error("Invalid signing package: {0}")]
    InvalidSigningPackage(String),

    #[error("FROST library error: {0}")]
    FrostError(String),
}

// ============================================================================
// Signer Actor 結構
// ============================================================================

/// 簽署者 Actor - 代表 FROST 協議中的一個參與者
///
/// ## 狀態管理
/// - `key_package`: 此簽署者的密鑰分片（不可變）
/// - `nonce_store`: SessionId -> SecretNonces 的映射（可變狀態）
///
/// ## 生命週期
/// ```text
/// 1. 初始化: Signer::new(key_package)
/// 2. Round 1: commit(session_id) -> 儲存 SecretNonces, 返回 Commitments
/// 3. Round 2: sign(session_id, signing_package) -> 消費 SecretNonces, 返回 SignatureShare
/// ```
pub struct Signer {
    /// 簽署者的唯一識別碼
    signer_id: frost::Identifier,

    /// 此簽署者的金鑰分片（包含私鑰分片）
    secret_share: frost::keys::SecretShare,

    /// Nonce 儲存: SessionId -> SecretNonces
    /// 使用 DashMap 提供並發安全且高效能的存取
    ///
    /// 為什麼使用 DashMap 而不是 Arc<Mutex<HashMap>>？
    /// - DashMap 使用分片鎖 (sharded locking)，降低鎖競爭
    /// - 對於讀多寫少的場景更高效
    /// - API 更簡潔，不需要手動 lock()/unlock()
    nonce_store: Arc<DashMap<SessionId, frost::round1::SigningNonces>>,
}

impl Signer {
    // ========================================================================
    // 建構函數
    // ========================================================================

    /// 建立新的簽署者實例
    ///
    /// # 參數
    /// - `key_package`: 此簽署者的金鑰分片（由 Setup 階段分發）
    pub fn new(secret_share: frost::keys::SecretShare) -> Self {
        let signer_id = *secret_share.identifier();

        Self {
            signer_id,
            secret_share,
            nonce_store: Arc::new(DashMap::new()),
        }
    }

    /// 獲取簽署者 ID
    pub fn id(&self) -> frost::Identifier {
        self.signer_id
    }

    // ========================================================================
    // Round 1: Commitment 生成
    // ========================================================================

    /// Round 1: 生成 Nonce 並建立 Commitment
    ///
    /// 此方法執行以下操作：
    /// 1. 生成隨機的 SigningNonces（包含秘密和公開部分）
    /// 2. 將秘密 nonce 儲存在內部（以 session_id 為索引）
    /// 3. 返回公開承諾（SigningCommitments）
    ///
    /// ## 密碼學原理
    /// FROST 使用 Commitment-Reveal 模式來防止惡意簽署者操縱 nonce：
    /// - 所有簽署者先提交承諾（無法修改）
    /// - 然後在 Round 2 才揭露如何使用 nonce
    /// - 這確保了簽章的不可偽造性
    ///
    /// # 參數
    /// - `session_id`: 此次簽章會話的唯一識別碼
    ///
    /// # 返回
    /// - `Ok(SigningCommitments)`: 公開承諾（可以安全地傳輸給協調者）
    /// - `Err(SignerError)`: Nonce 生成失敗
    ///
    /// # 安全性
    /// - 秘密 nonce 永遠不會離開此方法
    /// - 每個 session_id 只能生成一次 nonce（重複呼叫會覆蓋，但在正常流程中不應發生）
    pub fn commit(
        &self,
        session_id: SessionId,
    ) -> Result<frost::round1::SigningCommitments, SignerError> {
        tracing::info!(
            signer_id = ?self.signer_id,
            session_id = %session_id,
            "Generating nonce commitments for Round 1"
        );

        let mut rng = thread_rng();

        // 生成簽章 nonces
        // 這會返回兩個部分：
        // - SigningNonces: 秘密部分（必須保密）
        // - SigningCommitments: 公開承諾（可以傳輸）
        let (nonces, commitments) = frost::round1::commit(
            self.secret_share.signing_share(),
            &mut rng,
        );

        // 檢查是否已存在此 session 的 nonce（除錯用）
        if self.nonce_store.contains_key(&session_id) {
            tracing::warn!(
                session_id = %session_id,
                "Overwriting existing nonce for this session - this should not happen in normal operation"
            );
        }

        // 儲存秘密 nonce（將在 Round 2 使用）
        self.nonce_store.insert(session_id, nonces);

        tracing::debug!(
            signer_id = ?self.signer_id,
            session_id = %session_id,
            commitment_hex = %hex::encode(commitments.serialize().unwrap()),
            "Nonce stored, commitment generated"
        );

        Ok(commitments)
    }

    // ========================================================================
    // Round 2: 簽章分片生成
    // ========================================================================

    /// Round 2: 生成簽章分片
    ///
    /// 此方法執行以下操作：
    /// 1. 驗證 session_id 有效
    /// 2. 從儲存中檢索秘密 nonce
    /// 3. **立即刪除** nonce（防止重用）
    /// 4. 使用 key_share + nonce + signing_package 生成簽章分片
    ///
    /// ## 為什麼要立即刪除 Nonce？
    /// Nonce 重用是 Schnorr/ECDSA 簽章中的災難性錯誤：
    /// - 如果使用相同的 nonce 簽署兩個不同的訊息
    /// - 攻擊者可以從兩個簽章中推導出私鑰！
    /// - 因此，每個 nonce 必須且只能使用一次
    ///
    /// # 參數
    /// - `session_id`: Round 1 中使用的相同 Session ID
    /// - `signing_package_data`: 協調者提供的簽章套件（包含所有承諾和訊息）
    ///
    /// # 返回
    /// - `Ok(SignatureShare)`: 此簽署者的簽章分片
    /// - `Err(SignerError::SessionNotFound)`: Session ID 無效或 nonce 已被使用
    /// - `Err(SignerError::SignatureGenerationFailed)`: FROST 簽章生成失敗
    ///
    /// # 錯誤情況
    /// - Session ID 不存在 → 可能原因：
    ///   1. 從未呼叫過 commit()
    ///   2. Nonce 已經在先前的 sign() 中被消費
    ///   3. Session ID 輸入錯誤
    pub fn sign(
        &self,
        session_id: SessionId,
        signing_package_data: &SigningPackageData,
    ) -> Result<frost::round2::SignatureShare, SignerError> {
        tracing::info!(
            signer_id = ?self.signer_id,
            session_id = %session_id,
            "Generating signature share for Round 2"
        );

        // 步驟 1: 檢索並**消費**（刪除）秘密 nonce
        // 使用 remove() 而不是 get()，確保 nonce 只能使用一次
        let (_session_id, nonces) = self
            .nonce_store
            .remove(&session_id)
            .ok_or(SignerError::SessionNotFound(session_id))?;

        tracing::debug!(
            session_id = %session_id,
            "Secret nonce retrieved and removed from storage"
        );

        // 步驟 2: 反序列化簽章套件
        let signing_package = self
            .deserialize_signing_package(signing_package_data)
            .map_err(|e| SignerError::InvalidSigningPackage(e.to_string()))?;

        // 步驟 3: 生成簽章分片
        // 使用：金鑰分片 + 秘密 nonce + 簽章套件
        // Create a KeyPackage from the secret_share components
        let key_package = frost::keys::KeyPackage::try_from(self.secret_share.clone())
            .map_err(|e| SignerError::SignatureGenerationFailed(format!("KeyPackage conversion failed: {:?}", e)))?;

        let signature_share = frost::round2::sign(&signing_package, &nonces, &key_package)
            .map_err(|e| SignerError::SignatureGenerationFailed(format!("{:?}", e)))?;

        tracing::info!(
            signer_id = ?self.signer_id,
            session_id = %session_id,
            share_hex = %hex::encode(signature_share.serialize()),
            "Signature share generated successfully"
        );

        // 注意：nonces 在此處被 drop，記憶體中的秘密被清除
        Ok(signature_share)
    }

    // ========================================================================
    // 輔助方法 - 序列化/反序列化
    // ========================================================================

    /// 將 API 的 SigningPackageData 轉換為 FROST 的 SigningPackage
    fn deserialize_signing_package(
        &self,
        data: &SigningPackageData,
    ) -> Result<frost::SigningPackage, SignerError> {
        use std::collections::BTreeMap;

        // 反序列化所有承諾
        let mut commitments_map = BTreeMap::new();

        for commitment_data in &data.commitments {
            // 將簽署者 ID 轉換為 FROST Identifier
            let identifier = frost::Identifier::try_from(commitment_data.signer_id)
                .map_err(|e| SignerError::InvalidCommitment(format!("Invalid signer ID: {:?}", e)))?;

            // 反序列化承諾
            let commitment_bytes = hex::decode(&commitment_data.commitment)
                .map_err(|e| SignerError::InvalidCommitment(format!("Hex decode error: {}", e)))?;

            let commitment = frost::round1::SigningCommitments::deserialize(&commitment_bytes)
                .map_err(|e| SignerError::InvalidCommitment(format!("Deserialize error: {:?}", e)))?;

            commitments_map.insert(identifier, commitment);
        }

        // 建立 SigningPackage
        Ok(frost::SigningPackage::new(
            commitments_map,
            &data.message,
        ))
    }

    // ========================================================================
    // 管理與除錯方法
    // ========================================================================

    /// 獲取當前儲存的 Session 數量（用於監控）
    pub fn active_sessions_count(&self) -> usize {
        self.nonce_store.len()
    }

    /// 清除指定的 Session（用於錯誤恢復或超時處理）
    pub fn clear_session(&self, session_id: &SessionId) -> bool {
        self.nonce_store.remove(session_id).is_some()
    }

    /// 清除所有過期的 Sessions（可以定期呼叫）
    ///
    /// 注意：目前的簡單實作不包含時間戳，
    /// 在生產環境中應該將 (SessionId, Timestamp, Nonces) 一起儲存
    pub fn clear_all_sessions(&self) {
        self.nonce_store.clear();
        tracing::info!("Cleared all nonce sessions");
    }
}

// ============================================================================
// 執行緒安全性
// ============================================================================

// Signer 自動實現 Send + Sync，因為：
// - DashMap 實現了 Send + Sync
// - frost::keys::KeyPackage 實現了 Send + Sync
// - frost::Identifier 是 Copy 類型
// 因此不需要手動實現，編譯器會自動派生

// ============================================================================
// 測試
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // 這裡可以加入單元測試，例如：
    // - 測試 Nonce 一次性使用
    // - 測試並發 Sessions
    // - 測試錯誤情況
}
