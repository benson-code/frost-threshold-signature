//! # 檔案儲存 - JSON 序列化與反序列化
//!
//! 此模組處理所有中間結果的檔案操作：
//! - 金鑰分片
//! - 承諾
//! - 簽章套件
//! - 簽章分片
//! - 最終簽章
//!
//! ## 設計原則
//! 1. 所有資料使用 JSON 格式（人類可讀 + 機器可解析）
//! 2. 二進位資料（金鑰、簽章等）使用 hex 編碼
//! 3. 提供友善的錯誤訊息

use crate::api::{CommitmentData, SigningPackageData};
use anyhow::{Context, Result};
use frost_secp256k1 as frost;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// ============================================================================
// 檔案格式定義（JSON 可序列化）
// ============================================================================

/// 金鑰分片檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyShareFile {
    /// 簽署者 ID
    pub signer_id: u16,

    /// 序列化的金鑰分片（hex 編碼）
    pub key_package_hex: String,

    /// 元資訊
    pub metadata: KeyShareMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyShareMetadata {
    pub created_at: String,
    pub threshold: u16,
    pub max_signers: u16,
}

/// 群組公鑰檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyFile {
    /// 序列化的公鑰套件（hex 編碼）
    pub pubkey_package_hex: String,

    /// 群組公鑰（hex 編碼，方便驗證）
    pub group_pubkey_hex: String,

    /// 元資訊
    pub metadata: KeyShareMetadata,
}

/// 承諾檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentFile {
    /// Session ID
    pub session_id: String,

    /// 簽署者 ID
    pub signer_id: u16,

    /// 承諾資料（hex 編碼）
    pub commitment_hex: String,

    /// 訊息雜湊（用於驗證）
    pub message_hash: String,
}

/// 簽章套件檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningPackageFile {
    /// Session ID
    pub session_id: String,

    /// 所有承諾
    pub commitments: Vec<CommitmentData>,

    /// 訊息（hex 編碼）
    pub message_hex: String,

    /// 參與的簽署者 ID
    pub signer_ids: Vec<u16>,
}

/// 簽章分片檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureShareFile {
    /// Session ID
    pub session_id: String,

    /// 簽署者 ID
    pub signer_id: u16,

    /// 簽章分片（hex 編碼）
    pub signature_share_hex: String,
}

/// 最終簽章檔案格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureFile {
    /// Session ID
    pub session_id: String,

    /// 最終簽章（hex 編碼）
    pub signature_hex: String,

    /// 訊息（hex 編碼）
    pub message_hex: String,

    /// 參與的簽署者 ID
    pub signer_ids: Vec<u16>,
}

// ============================================================================
// FileStore - 檔案操作的統一介面
// ============================================================================

pub struct FileStore;

impl FileStore {
    // ========================================================================
    // 金鑰分片相關
    // ========================================================================

    /// 儲存金鑰分片
    pub fn save_key_share(
        path: &Path,
        signer_id: u16,
        key_package: &frost::keys::KeyPackage,
        threshold: u16,
        max_signers: u16,
    ) -> Result<()> {
        let key_share_file = KeyShareFile {
            signer_id,
            key_package_hex: hex::encode(key_package.serialize()?),
            metadata: KeyShareMetadata {
                created_at: chrono::Utc::now().to_rfc3339(),
                threshold,
                max_signers,
            },
        };

        let json = serde_json::to_string_pretty(&key_share_file)?;
        fs::write(path, json).context("Failed to write key share file")?;

        Ok(())
    }

    /// 載入金鑰分片
    pub fn load_key_share(path: &Path) -> Result<frost::keys::KeyPackage> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read key share file: {}", path.display()))?;

        let key_share_file: KeyShareFile = serde_json::from_str(&json)
            .context("Failed to parse key share JSON")?;

        let key_package_bytes = hex::decode(&key_share_file.key_package_hex)
            .context("Failed to decode key package hex")?;

        frost::keys::KeyPackage::deserialize(&key_package_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize key package: {:?}", e))
    }

    /// 儲存群組公鑰
    pub fn save_public_key(
        path: &Path,
        pubkey_package: &frost::keys::PublicKeyPackage,
        threshold: u16,
        max_signers: u16,
    ) -> Result<()> {
        let pubkey_file = PublicKeyFile {
            pubkey_package_hex: hex::encode(pubkey_package.serialize()?),
            group_pubkey_hex: hex::encode(pubkey_package.verifying_key().serialize()?),
            metadata: KeyShareMetadata {
                created_at: chrono::Utc::now().to_rfc3339(),
                threshold,
                max_signers,
            },
        };

        let json = serde_json::to_string_pretty(&pubkey_file)?;
        fs::write(path, json).context("Failed to write public key file")?;

        Ok(())
    }

    /// 載入群組公鑰
    pub fn load_public_key(path: &Path) -> Result<frost::keys::PublicKeyPackage> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read public key file: {}", path.display()))?;

        let pubkey_file: PublicKeyFile = serde_json::from_str(&json)
            .context("Failed to parse public key JSON")?;

        let pubkey_package_bytes = hex::decode(&pubkey_file.pubkey_package_hex)
            .context("Failed to decode public key hex")?;

        frost::keys::PublicKeyPackage::deserialize(&pubkey_package_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize public key package: {:?}", e))
    }

    // ========================================================================
    // 承諾相關
    // ========================================================================

    /// 儲存承諾
    pub fn save_commitment(
        path: &Path,
        session_id: &str,
        signer_id: u16,
        commitment: &frost::round1::SigningCommitments,
        message: &[u8],
    ) -> Result<()> {
        let commitment_file = CommitmentFile {
            session_id: session_id.to_string(),
            signer_id,
            commitment_hex: hex::encode(commitment.serialize()?),
            message_hash: hex::encode(&message[..32.min(message.len())]),
        };

        let json = serde_json::to_string_pretty(&commitment_file)?;
        fs::write(path, json).context("Failed to write commitment file")?;

        Ok(())
    }

    /// 載入承諾
    pub fn load_commitment(path: &Path) -> Result<CommitmentFile> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read commitment file: {}", path.display()))?;

        serde_json::from_str(&json).context("Failed to parse commitment JSON")
    }

    /// 載入多個承諾並轉換為 FROST 格式
    pub fn load_commitments_map(
        paths: &[impl AsRef<Path>],
    ) -> Result<BTreeMap<frost::Identifier, frost::round1::SigningCommitments>> {
        let mut commitments_map = BTreeMap::new();

        for path in paths {
            let commitment_file = Self::load_commitment(path.as_ref())?;

            let identifier = frost::Identifier::try_from(commitment_file.signer_id)
                .map_err(|e| anyhow::anyhow!("Invalid signer ID: {:?}", e))?;

            let commitment_bytes = hex::decode(&commitment_file.commitment_hex)
                .context("Failed to decode commitment hex")?;

            let commitment = frost::round1::SigningCommitments::deserialize(&commitment_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize commitment: {:?}", e))?;

            commitments_map.insert(identifier, commitment);
        }

        Ok(commitments_map)
    }

    // ========================================================================
    // 簽章套件相關
    // ========================================================================

    /// 儲存簽章套件
    pub fn save_signing_package(
        path: &Path,
        session_id: &str,
        package_data: &SigningPackageData,
        signer_ids: Vec<u16>,
    ) -> Result<()> {
        let package_file = SigningPackageFile {
            session_id: session_id.to_string(),
            commitments: package_data.commitments.clone(),
            message_hex: hex::encode(&package_data.message),
            signer_ids,
        };

        let json = serde_json::to_string_pretty(&package_file)?;
        fs::write(path, json).context("Failed to write signing package file")?;

        Ok(())
    }

    /// 載入簽章套件
    pub fn load_signing_package(path: &Path) -> Result<SigningPackageFile> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read signing package file: {}", path.display()))?;

        serde_json::from_str(&json).context("Failed to parse signing package JSON")
    }

    // ========================================================================
    // 簽章分片相關
    // ========================================================================

    /// 儲存簽章分片
    pub fn save_signature_share(
        path: &Path,
        session_id: &str,
        signer_id: u16,
        signature_share: &frost::round2::SignatureShare,
    ) -> Result<()> {
        let share_file = SignatureShareFile {
            session_id: session_id.to_string(),
            signer_id,
            signature_share_hex: hex::encode(signature_share.serialize()),
        };

        let json = serde_json::to_string_pretty(&share_file)?;
        fs::write(path, json).context("Failed to write signature share file")?;

        Ok(())
    }

    /// 載入簽章分片
    pub fn load_signature_share(path: &Path) -> Result<SignatureShareFile> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read signature share file: {}", path.display()))?;

        serde_json::from_str(&json).context("Failed to parse signature share JSON")
    }

    /// 載入多個簽章分片並轉換為 FROST 格式
    pub fn load_signature_shares_map(
        paths: &[impl AsRef<Path>],
    ) -> Result<BTreeMap<frost::Identifier, frost::round2::SignatureShare>> {
        let mut shares_map = BTreeMap::new();

        for path in paths {
            let share_file = Self::load_signature_share(path.as_ref())?;

            let identifier = frost::Identifier::try_from(share_file.signer_id)
                .map_err(|e| anyhow::anyhow!("Invalid signer ID: {:?}", e))?;

            let share_bytes = hex::decode(&share_file.signature_share_hex)
                .context("Failed to decode signature share hex")?;

            let signature_share = frost::round2::SignatureShare::deserialize(&share_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize signature share: {:?}", e))?;

            shares_map.insert(identifier, signature_share);
        }

        Ok(shares_map)
    }

    // ========================================================================
    // 最終簽章相關
    // ========================================================================

    /// 儲存最終簽章
    pub fn save_signature(
        path: &Path,
        session_id: &str,
        signature: &frost::Signature,
        message: &[u8],
        signer_ids: Vec<u16>,
    ) -> Result<()> {
        let signature_file = SignatureFile {
            session_id: session_id.to_string(),
            signature_hex: hex::encode(signature.serialize()?),
            message_hex: hex::encode(message),
            signer_ids,
        };

        let json = serde_json::to_string_pretty(&signature_file)?;
        fs::write(path, json).context("Failed to write signature file")?;

        Ok(())
    }

    /// 載入最終簽章
    pub fn load_signature(path: &Path) -> Result<SignatureFile> {
        let json = fs::read_to_string(path)
            .context(format!("Failed to read signature file: {}", path.display()))?;

        serde_json::from_str(&json).context("Failed to parse signature JSON")
    }

    /// 反序列化簽章
    pub fn deserialize_signature(signature_hex: &str) -> Result<frost::Signature> {
        let signature_bytes = hex::decode(signature_hex)
            .context("Failed to decode signature hex")?;

        frost::Signature::deserialize(&signature_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize signature: {:?}", e))
    }

    // ========================================================================
    // 訊息讀取
    // ========================================================================

    /// 讀取訊息檔案
    pub fn read_message(path: &Path) -> Result<Vec<u8>> {
        fs::read(path).context(format!("Failed to read message file: {}", path.display()))
    }

    // ========================================================================
    // 輔助方法
    // ========================================================================

    /// 確保目錄存在
    pub fn ensure_dir(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)
                .context(format!("Failed to create directory: {}", path.display()))?;
        }
        Ok(())
    }
}
