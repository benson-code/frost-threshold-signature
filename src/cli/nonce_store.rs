//! # Nonce 持久化儲存（僅供 Demo 使用）
//!
//! ⚠️ **安全警告**：
//! 在生產環境中，**絕對不應該**將秘密 Nonce 寫入磁碟！
//! 這個模組僅用於 CLI Demo，讓我們可以在多個終端視窗模擬完整流程。
//!
//! 在真實場景中，Nonce 應該：
//! 1. 僅存在於記憶體中
//! 2. 使用後立即銷毀
//! 3. 如需持久化，必須使用 HSM 或加密儲存

use anyhow::{Context, Result};
use frost_secp256k1 as frost;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// Nonce 檔案格式
// ============================================================================

/// Nonce 儲存檔案格式
///
/// ⚠️ 僅供 Demo 使用！生產環境不應持久化秘密 Nonce！
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceFile {
    /// Session ID
    pub session_id: String,

    /// 簽署者 ID
    pub signer_id: u16,

    /// 序列化的秘密 Nonce（hex 編碼）
    ///
    /// ⚠️ 極度敏感！洩漏會導致私鑰洩漏！
    pub nonce_hex: String,

    /// 警告訊息
    pub warning: String,
}

// ============================================================================
// NonceStore - Nonce 持久化介面
// ============================================================================

pub struct NonceStore;

impl NonceStore {
    /// 預設的 Nonce 儲存目錄
    const NONCE_DIR: &'static str = ".frost-nonces";

    /// 取得 Nonce 檔案路徑
    fn get_nonce_path(session_id: &str, signer_id: u16) -> PathBuf {
        PathBuf::from(Self::NONCE_DIR).join(format!("nonce_{}_{}.json", session_id, signer_id))
    }

    /// 儲存秘密 Nonce
    ///
    /// ⚠️ 僅供 Demo 使用！
    pub fn save_nonce(
        session_id: &str,
        signer_id: u16,
        nonce: &frost::round1::SigningNonces,
    ) -> Result<PathBuf> {
        // 確保目錄存在
        let nonce_dir = PathBuf::from(Self::NONCE_DIR);
        if !nonce_dir.exists() {
            fs::create_dir_all(&nonce_dir)
                .context("無法建立 Nonce 儲存目錄")?;
        }

        let nonce_file = NonceFile {
            session_id: session_id.to_string(),
            signer_id,
            nonce_hex: hex::encode(nonce.serialize()),
            warning: "⚠️ DEMO ONLY! Never persist secret nonces in production!".to_string(),
        };

        let path = Self::get_nonce_path(session_id, signer_id);
        let json = serde_json::to_string_pretty(&nonce_file)?;
        fs::write(&path, json)
            .context("無法寫入 Nonce 檔案")?;

        // 設置檔案權限為 600 (僅擁有者可讀寫) - Unix only
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&path, permissions)?;
        }

        Ok(path)
    }

    /// 載入並刪除秘密 Nonce（一次性使用）
    ///
    /// ⚠️ 僅供 Demo 使用！
    pub fn load_and_delete_nonce(
        session_id: &str,
        signer_id: u16,
    ) -> Result<frost::round1::SigningNonces> {
        let path = Self::get_nonce_path(session_id, signer_id);

        // 讀取檔案
        let json = fs::read_to_string(&path)
            .context(format!(
                "無法讀取 Nonce 檔案: {}\n提示：請確保已先執行 Round 1",
                path.display()
            ))?;

        let nonce_file: NonceFile = serde_json::from_str(&json)
            .context("無法解析 Nonce JSON")?;

        // 驗證 Session ID 和 Signer ID
        if nonce_file.session_id != session_id {
            anyhow::bail!("Session ID 不匹配");
        }
        if nonce_file.signer_id != signer_id {
            anyhow::bail!("Signer ID 不匹配");
        }

        // 反序列化 Nonce
        let nonce_bytes = hex::decode(&nonce_file.nonce_hex)
            .context("無法解碼 Nonce hex")?;

        let nonce = frost::round1::SigningNonces::deserialize(&nonce_bytes)
            .map_err(|e| anyhow::anyhow!("無法反序列化 Nonce: {:?}", e))?;

        // 立即刪除檔案（一次性使用）
        fs::remove_file(&path)
            .context("無法刪除 Nonce 檔案")?;

        Ok(nonce)
    }

    /// 清除所有 Nonce 檔案（清理用）
    pub fn clear_all_nonces() -> Result<usize> {
        let nonce_dir = PathBuf::from(Self::NONCE_DIR);
        if !nonce_dir.exists() {
            return Ok(0);
        }

        let mut count = 0;
        for entry in fs::read_dir(nonce_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                fs::remove_file(entry.path())?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// 列出所有儲存的 Nonce
    pub fn list_nonces() -> Result<Vec<(String, u16)>> {
        let nonce_dir = PathBuf::from(Self::NONCE_DIR);
        if !nonce_dir.exists() {
            return Ok(Vec::new());
        }

        let mut nonces = Vec::new();
        for entry in fs::read_dir(nonce_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let json = fs::read_to_string(entry.path())?;
                let nonce_file: NonceFile = serde_json::from_str(&json)?;
                nonces.push((nonce_file.session_id, nonce_file.signer_id));
            }
        }

        Ok(nonces)
    }
}
