//! # API 合約定義
//!
//! 此模組定義了 FROST 門檻簽章服務的所有 HTTP API 請求和回應結構。
//!
//! ## 安全考量
//! - 所有操作都使用 `SessionId` 來防止重放攻擊和混淆攻擊
//! - SigningNonces 永遠不會透過 API 傳輸（僅傳輸 Commitments）
//! - 每個 Session 的 Nonce 在使用後會被立即銷毀（一次性使用）

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// 核心類型別名
// ============================================================================

/// Session ID - 用於識別一個獨立的簽章流程
///
/// 每個簽章請求都會獲得一個唯一的 Session ID，用於：
/// 1. 防止重放攻擊（replay attacks）
/// 2. 關聯 Round 1 和 Round 2 的狀態
/// 3. 支援並發的多個簽章流程
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    /// 生成新的隨機 Session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Setup API - 初始化簽章群組
// ============================================================================

/// POST /setup - 初始化一個新的 FROST 簽章群組
///
/// 在生產環境中，這應該使用分散式金鑰生成 (DKG)。
/// 目前使用 Trusted Dealer 方法進行演示。
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupRequest {
    /// 簽署者總數（例如：5）
    pub max_signers: u16,

    /// 門檻值 - 生成簽章所需的最小簽署者數（例如：3）
    pub min_signers: u16,
}

/// Setup 成功回應
#[derive(Debug, Serialize, Deserialize)]
pub struct SetupResponse {
    /// 群組公鑰（用於驗證簽章）
    pub group_public_key: String, // hex-encoded

    /// 簽署者 ID 列表
    pub signer_ids: Vec<u16>,

    /// 成功訊息
    pub message: String,
}

// ============================================================================
// Round 1 API - Commitment 階段
// ============================================================================

/// POST /signer/{signer_id}/round1 - 簽署者生成並提交 Commitment
///
/// 此端點觸發簽署者：
/// 1. 生成隨機 nonce (秘密)
/// 2. 計算 nonce 的公開承諾
/// 3. 內部儲存秘密 nonce（以 session_id 為索引）
/// 4. 返回公開承諾給協調者
#[derive(Debug, Serialize, Deserialize)]
pub struct Round1Request {
    /// Session ID - 關聯此次簽章流程
    pub session_id: SessionId,

    /// 要簽署的訊息（通常是交易雜湊）
    #[serde(with = "hex_serde")]
    pub message: Vec<u8>,
}

/// Round 1 成功回應
#[derive(Debug, Serialize, Deserialize)]
pub struct Round1Response {
    /// 簽署者 ID
    pub signer_id: u16,

    /// Session ID（回傳確認）
    pub session_id: SessionId,

    /// 公開承諾（hex 編碼）
    /// 注意：秘密 nonce 永遠不會離開簽署者
    pub commitment: String, // hex-encoded SigningCommitments

    /// 時間戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Round 2 API - Signature Share 階段
// ============================================================================

/// POST /signer/{signer_id}/round2 - 簽署者生成簽章分片
///
/// 此端點觸發簽署者：
/// 1. 驗證 session_id 有效
/// 2. 檢索並**消費**（刪除）對應的秘密 nonce
/// 3. 使用 key_share + nonce + signing_package 生成簽章分片
/// 4. 返回簽章分片
///
/// ## 安全性
/// - 如果 session_id 無效或 nonce 已被使用，返回錯誤
/// - Nonce 只能使用一次（防止 nonce 重用攻擊）
#[derive(Debug, Serialize, Deserialize)]
pub struct Round2Request {
    /// Session ID（必須與 Round 1 相同）
    pub session_id: SessionId,

    /// 簽章套件（由協調者建立）
    /// 包含：所有參與者的承諾 + 訊息
    pub signing_package: SigningPackageData,
}

/// 簽章套件資料（可序列化的版本）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningPackageData {
    /// 所有參與簽署者的承諾
    pub commitments: Vec<CommitmentData>,

    /// 要簽署的訊息
    #[serde(with = "hex_serde")]
    pub message: Vec<u8>,
}

/// 單個簽署者的承諾資料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentData {
    /// 簽署者 ID
    pub signer_id: u16,

    /// 承諾（hex 編碼）
    pub commitment: String, // hex-encoded
}

/// Round 2 成功回應
#[derive(Debug, Serialize, Deserialize)]
pub struct Round2Response {
    /// 簽署者 ID
    pub signer_id: u16,

    /// Session ID（回傳確認）
    pub session_id: SessionId,

    /// 簽章分片（hex 編碼）
    pub signature_share: String, // hex-encoded

    /// 時間戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Coordinator API - 聚合簽章
// ============================================================================

/// POST /coordinator/aggregate - 協調者聚合簽章分片
///
/// 協調者收集所有簽章分片後，聚合成最終的 Schnorr 簽章。
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregateRequest {
    /// Session ID
    pub session_id: SessionId,

    /// 所有簽署者的簽章分片
    pub signature_shares: Vec<SignatureShareData>,
}

/// 單個簽署者的簽章分片資料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureShareData {
    /// 簽署者 ID
    pub signer_id: u16,

    /// 簽章分片（hex 編碼）
    pub signature_share: String, // hex-encoded
}

/// 聚合簽章成功回應
#[derive(Debug, Serialize, Deserialize)]
pub struct AggregateResponse {
    /// Session ID
    pub session_id: SessionId,

    /// 最終的群組簽章（hex 編碼的 Schnorr 簽章）
    pub signature: String,

    /// 驗證狀態
    pub verified: bool,

    /// 成功訊息
    pub message: String,
}

// ============================================================================
// 通用錯誤回應
// ============================================================================

/// 統一的錯誤回應格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// 錯誤代碼（例如：NONCE_NOT_FOUND, INVALID_SESSION）
    pub error_code: String,

    /// 人類可讀的錯誤訊息
    pub message: String,

    /// 可選的詳細資訊（用於除錯）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error_code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_code: error_code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

// ============================================================================
// 輔助模組 - Hex 序列化
// ============================================================================

/// 自訂的 Serde 模組，用於將 Vec<u8> 序列化為 hex 字串
mod hex_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        hex::decode(&s).map_err(serde::de::Error::custom)
    }
}
