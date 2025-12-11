//! # FROST 門檻簽章服務 - Level 2: HTTP API 架構
//!
//! 這是一個企業級的 FROST (Flexible Round-Optimized Schnorr Threshold) 門檻簽章服務。
//!
//! ## 架構
//! - **Coordinator**: 編排簽章流程，但不持有私鑰
//! - **Signers**: 獨立的簽署者 Actor，管理自己的金鑰分片和 Nonce 狀態
//! - **HTTP API**: RESTful API 提供簽章服務
//!
//! ## API 端點
//! - `GET  /health` - 健康檢查
//! - `GET  /pubkey` - 獲取群組公鑰
//! - `POST /signer/:id/round1` - Round 1: 生成承諾
//! - `POST /signer/:id/round2` - Round 2: 生成簽章分片
//! - `POST /sign` - 完整簽章流程（示範用）
//!
//! ## 運行方式
//! ```bash
//! cargo run --release
//! ```
//!
//! 服務將在 http://127.0.0.1:3000 啟動

// ============================================================================
// 模組聲明
// ============================================================================

mod api;
mod coordinator;
mod handlers;
mod signer;

// ============================================================================
// 導入
// ============================================================================

use axum::{
    routing::{get, post},
    Router,
};
use frost_secp256k1 as frost;
use handlers::AppState;
use rand::thread_rng;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::Level;

// ============================================================================
// 主程式
// ============================================================================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ========================================================================
    // 初始化日誌系統
    // ========================================================================
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .init();

    print_banner();

    // ========================================================================
    // Setup: Trusted Dealer 金鑰生成
    // ========================================================================
    tracing::info!("🔑 Initializing FROST setup (Trusted Dealer)");

    let max_signers = 5;
    let min_signers = 3;

    let mut rng = thread_rng();

    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )?;

    tracing::info!(
        "✓ Generated {} key shares with threshold {}",
        max_signers,
        min_signers
    );

    let group_pubkey = pubkey_package.verifying_key();
    tracing::info!(
        "✓ Group public key: {}...",
        &hex::encode(group_pubkey.serialize().unwrap())[..32]
    );

    // ========================================================================
    // 建立 Coordinator 和 Signers
    // ========================================================================
    let coordinator = coordinator::Coordinator::new(pubkey_package, min_signers);

    let app_state = AppState::new(coordinator);

    // 為每個金鑰分片建立 Signer
    for (identifier, key_package) in shares {
        let signer = signer::Signer::new(key_package);

        // Convert Identifier to u16 by serializing
        let id_bytes = identifier.serialize();
        let signer_id = u16::from_le_bytes([id_bytes[0], id_bytes[1]]);

        app_state.add_signer(signer_id, signer);

        tracing::info!("✓ Created Signer {}", signer_id);
    }

    // ========================================================================
    // 建立 HTTP 路由
    // ========================================================================
    let app = Router::new()
        // 健康檢查與資訊端點
        .route("/health", get(handlers::health))
        .route("/pubkey", get(handlers::get_pubkey))
        // Round 1: Commitment 生成
        .route(
            "/signer/:signer_id/round1",
            post(handlers::signer_round1),
        )
        // Round 2: Signature Share 生成
        .route(
            "/signer/:signer_id/round2",
            post(handlers::signer_round2),
        )
        // 完整簽章流程（示範用）
        .route("/sign", post(handlers::sign))
        // 添加共享狀態
        .with_state(app_state)
        // 添加日誌中間件
        .layer(TraceLayer::new_for_http());

    // ========================================================================
    // 啟動 HTTP 服務
    // ========================================================================
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("🚀 FROST API Server starting on http://{}", addr);
    tracing::info!("📚 API Documentation:");
    tracing::info!("   GET  /health                    - Health check");
    tracing::info!("   GET  /pubkey                    - Get group public key");
    tracing::info!("   POST /signer/:id/round1         - Round 1: Generate commitment");
    tracing::info!("   POST /signer/:id/round2         - Round 2: Generate signature share");
    tracing::info!("   POST /sign                      - Complete signing flow");
    tracing::info!("");
    tracing::info!("💡 Try the demo client:");
    tracing::info!("   cargo run --example demo_client");
    tracing::info!("");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// 輔助函數
// ============================================================================

fn print_banner() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║   FROST 3-of-5 門檻簽章服務                                    ║");
    println!("║   Level 2: HTTP API Architecture                              ║");
    println!("║                                                                ║");
    println!("║   Bitcoin-Compatible Schnorr Threshold Signatures             ║");
    println!("║   Using secp256k1 curve (Taproot compatible)                  ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
}
