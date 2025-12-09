//! # FROST 門檻簽章服務 - 核心函式庫
//!
//! 這個 library crate 提供 FROST 3-of-5 門檻簽章的核心功能，
//! 可被不同的前端使用（HTTP API、CLI 工具等）。
//!
//! ## 模組結構
//!
//! - `coordinator`: 協調者邏輯 - 編排簽章流程，不持有私鑰
//! - `signer`: 簽署者邏輯 - 管理金鑰分片和 Nonce 狀態
//! - `api`: API 合約 - 共用的資料結構（用於序列化）
//! - `cli`: CLI 工具相關模組（條件編譯）
//!
//! ## 使用範例
//!
//! ```no_run
//! use frost_threshold_signature::{coordinator::Coordinator, signer::Signer};
//! use frost_secp256k1 as frost;
//! use rand::thread_rng;
//!
//! # fn main() -> anyhow::Result<()> {
//! // 1. 金鑰生成
//! let (shares, pubkey_package) = frost::keys::generate_with_dealer(
//!     5,  // max_signers
//!     3,  // min_signers
//!     frost::keys::IdentifierList::Default,
//!     &mut thread_rng(),
//! )?;
//!
//! // 2. 建立協調者和簽署者
//! let coordinator = Coordinator::new(pubkey_package, 3);
//! // ... 建立 Signers
//! # Ok(())
//! # }
//! ```

// ============================================================================
// 重新匯出 frost-secp256k1 以便外部使用
// ============================================================================

pub use frost_secp256k1 as frost;

// ============================================================================
// 核心模組 - 始終可用
// ============================================================================

pub mod api;
pub mod coordinator;
pub mod signer;

// ============================================================================
// Transport 抽象層 - 訊息傳遞介面
// ============================================================================

pub mod transport;

// ============================================================================
// CLI 模組
// ============================================================================

pub mod cli;

// ============================================================================
// 常用型別重新匯出 - 簡化外部使用
// ============================================================================

pub use api::{CommitmentData, SessionId, SignatureShareData, SigningPackageData};
pub use coordinator::{Coordinator, CoordinatorError};
pub use signer::{Signer, SignerError};

// ============================================================================
// 版本資訊
// ============================================================================

/// 獲取函式庫版本
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// 獲取函式庫名稱
pub fn name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
