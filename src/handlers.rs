//! # HTTP 處理器
//!
//! 此模組實作了所有 HTTP 端點的處理邏輯。
//! 使用 Axum 框架提供 RESTful API。

use crate::api::*;
use crate::coordinator::Coordinator;
use crate::signer::Signer;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

// ============================================================================
// 共享應用狀態
// ============================================================================

/// 應用的共享狀態
///
/// 使用 Arc 包裝，可以在多個請求處理器之間安全共享。
#[derive(Clone)]
pub struct AppState {
    /// 協調者實例
    pub coordinator: Arc<Coordinator>,

    /// 所有簽署者的列表
    /// Key: Signer ID (u16)
    /// Value: Signer 實例
    pub signers: Arc<dashmap::DashMap<u16, Arc<Signer>>>,
}

impl AppState {
    pub fn new(coordinator: Coordinator) -> Self {
        Self {
            coordinator: Arc::new(coordinator),
            signers: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub fn add_signer(&self, signer_id: u16, signer: Signer) {
        self.signers.insert(signer_id, Arc::new(signer));
    }

    pub fn get_signer(&self, signer_id: u16) -> Option<Arc<Signer>> {
        self.signers.get(&signer_id).map(|s| Arc::clone(&s))
    }
}

// ============================================================================
// 錯誤處理
// ============================================================================

/// 統一的錯誤類型
pub enum ApiError {
    SignerNotFound(u16),
    SignerError(crate::signer::SignerError),
    CoordinatorError(crate::coordinator::CoordinatorError),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::SignerNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse::new(
                    "SIGNER_NOT_FOUND",
                    format!("Signer {} not found", id),
                ),
            ),
            ApiError::SignerError(e) => {
                let (code, message) = match e {
                    crate::signer::SignerError::SessionNotFound(session_id) => (
                        "SESSION_NOT_FOUND",
                        format!(
                            "Session {} not found - nonce may have been used or never generated",
                            session_id
                        ),
                    ),
                    _ => ("SIGNER_ERROR", e.to_string()),
                };
                (StatusCode::BAD_REQUEST, ErrorResponse::new(code, message))
            }
            ApiError::CoordinatorError(e) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new("COORDINATOR_ERROR", e.to_string()),
            ),
            ApiError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("INTERNAL_ERROR", msg),
            ),
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<crate::signer::SignerError> for ApiError {
    fn from(e: crate::signer::SignerError) -> Self {
        ApiError::SignerError(e)
    }
}

impl From<crate::coordinator::CoordinatorError> for ApiError {
    fn from(e: crate::coordinator::CoordinatorError) -> Self {
        ApiError::CoordinatorError(e)
    }
}

// ============================================================================
// Handler: Round 1 - Commitment
// ============================================================================

/// POST /signer/:signer_id/round1
///
/// 簽署者生成並返回 Round 1 承諾
pub async fn signer_round1(
    State(state): State<AppState>,
    Path(signer_id): Path<u16>,
    Json(request): Json<Round1Request>,
) -> Result<Json<Round1Response>, ApiError> {
    tracing::info!(
        signer_id = signer_id,
        session_id = %request.session_id,
        "Received Round 1 request"
    );

    // 獲取簽署者
    let signer = state
        .get_signer(signer_id)
        .ok_or(ApiError::SignerNotFound(signer_id))?;

    // 生成承諾
    let commitment = signer.commit(request.session_id)?;

    // 建立回應
    let response = Round1Response {
        signer_id,
        session_id: request.session_id,
        commitment: hex::encode(commitment.serialize()),
        timestamp: chrono::Utc::now(),
    };

    tracing::info!(
        signer_id = signer_id,
        session_id = %request.session_id,
        "Round 1 commitment generated"
    );

    Ok(Json(response))
}

// ============================================================================
// Handler: Round 2 - Signature Share
// ============================================================================

/// POST /signer/:signer_id/round2
///
/// 簽署者生成並返回簽章分片
pub async fn signer_round2(
    State(state): State<AppState>,
    Path(signer_id): Path<u16>,
    Json(request): Json<Round2Request>,
) -> Result<Json<Round2Response>, ApiError> {
    tracing::info!(
        signer_id = signer_id,
        session_id = %request.session_id,
        "Received Round 2 request"
    );

    // 獲取簽署者
    let signer = state
        .get_signer(signer_id)
        .ok_or(ApiError::SignerNotFound(signer_id))?;

    // 生成簽章分片
    let signature_share = signer.sign(request.session_id, &request.signing_package)?;

    // 建立回應
    let response = Round2Response {
        signer_id,
        session_id: request.session_id,
        signature_share: hex::encode(signature_share.serialize()),
        timestamp: chrono::Utc::now(),
    };

    tracing::info!(
        signer_id = signer_id,
        session_id = %request.session_id,
        "Round 2 signature share generated"
    );

    Ok(Json(response))
}

// ============================================================================
// Handler: 完整簽章流程（示範用）
// ============================================================================

/// POST /sign
///
/// 執行完整的簽章流程（示範用高階 API）
///
/// 在生產環境中，客戶端通常會分別呼叫 Round 1 和 Round 2 端點。
/// 這個端點展示了如何使用協調者編排整個流程。
#[derive(serde::Deserialize)]
pub struct SignRequest {
    /// 參與簽署的簽署者 ID 列表
    pub signer_ids: Vec<u16>,

    /// 要簽署的訊息（hex 編碼）
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct SignResponse {
    /// Session ID
    pub session_id: SessionId,

    /// 最終簽章（hex 編碼）
    pub signature: String,

    /// 驗證狀態
    pub verified: bool,

    /// 使用的群組公鑰
    pub group_public_key: String,
}

pub async fn sign(
    State(state): State<AppState>,
    Json(request): Json<SignRequest>,
) -> Result<Json<SignResponse>, ApiError> {
    tracing::info!(
        signer_ids = ?request.signer_ids,
        "Received complete signing request"
    );

    // 解碼訊息
    let message = hex::decode(&request.message)
        .map_err(|e| ApiError::InternalError(format!("Invalid hex message: {}", e)))?;

    // 收集簽署者
    let mut signers = Vec::new();
    for signer_id in &request.signer_ids {
        let signer = state
            .get_signer(*signer_id)
            .ok_or(ApiError::SignerNotFound(*signer_id))?;
        signers.push(signer);
    }

    // 執行完整的簽章流程
    let signature = state
        .coordinator
        .orchestrate_signing(&signers, &message)
        .await?;

    // 建立回應
    let response = SignResponse {
        session_id: SessionId::new(), // 在實際實作中，這應該從協調者返回
        signature: hex::encode(signature.serialize()),
        verified: true,
        group_public_key: hex::encode(state.coordinator.group_public_key().serialize()),
    };

    tracing::info!(
        signature = %response.signature,
        "Complete signing flow finished"
    );

    Ok(Json(response))
}

// ============================================================================
// Handler: 健康檢查
// ============================================================================

/// GET /health
///
/// 健康檢查端點
#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub signers_count: usize,
    pub active_sessions: usize,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        signers_count: state.signers.len(),
        active_sessions: state.coordinator.active_sessions_count(),
    })
}

// ============================================================================
// Handler: 獲取群組公鑰
// ============================================================================

/// GET /pubkey
///
/// 獲取群組公鑰（用於驗證簽章）
#[derive(serde::Serialize)]
pub struct PubkeyResponse {
    pub group_public_key: String,
}

pub async fn get_pubkey(State(state): State<AppState>) -> Json<PubkeyResponse> {
    Json(PubkeyResponse {
        group_public_key: hex::encode(state.coordinator.group_public_key().serialize()),
    })
}
