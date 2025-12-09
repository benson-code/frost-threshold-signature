//! # FROST CLI 主程式
//!
//! 這是 FROST 門檻簽章的命令列工具前端。
//! 支援在多個終端視窗模擬不同角色進行離線簽章。

use anyhow::{Context, Result};
use clap::Parser;
use frost_secp256k1 as frost;
use frost_threshold_signature::cli::{commands::*, file_store::*, nonce_store::*};
use frost_threshold_signature::transport::{
    LoRaTransportState, MessageMetadata, MessageType, SimulatedLoRaTransport, StdoutTransport,
    Transport,
};
use frost_threshold_signature::{api::*, frost, Coordinator, Signer};
use rand::thread_rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ============================================================================
// 主程式入口
// ============================================================================

fn main() -> Result<()> {
    // 解析命令列參數
    let cli = Cli::parse_args();

    // 根據命令執行對應的處理函式
    match &cli.command {
        Commands::Keygen {
            output_dir,
            max_signers,
            min_signers,
        } => cmd_keygen(output_dir, *max_signers, *min_signers, cli.verbose),

        Commands::Round1 {
            share_file,
            message_file,
            output,
            session_id,
        } => cmd_round1(share_file, message_file, output.as_deref(), session_id.as_deref(), cli.verbose),

        Commands::CreatePackage {
            commitment_files,
            message_file,
            output,
        } => cmd_create_package(commitment_files, message_file, output, cli.verbose),

        Commands::Round2 {
            share_file,
            package_file,
            output,
            session_id,
        } => cmd_round2(share_file, package_file, output.as_deref(), session_id, cli.verbose),

        Commands::Aggregate {
            package_file,
            share_files,
            pubkey_file,
            output,
        } => cmd_aggregate(package_file, share_files, pubkey_file, output, cli.verbose),

        Commands::Verify {
            signature_file,
            message_file,
            pubkey_file,
        } => cmd_verify(signature_file, message_file, pubkey_file, cli.verbose),

        Commands::DemoBasic {
            message,
            signers,
            full_payload,
        } => {
            // DemoBasic 需要異步 runtime（用於 HTTP Server）
            tokio::runtime::Runtime::new()
                .context("無法創建 Tokio runtime")?
                .block_on(cmd_demo_basic(message, signers, *full_payload))
        }
    }
}

// ============================================================================
// 命令處理函式
// ============================================================================

/// 【Dealer】生成金鑰分片
fn cmd_keygen(
    output_dir: &std::path::Path,
    max_signers: u16,
    min_signers: u16,
    verbose: bool,
) -> Result<()> {
    println!("🔑 生成 FROST 金鑰分片...\n");

    // 驗證參數
    if min_signers > max_signers {
        anyhow::bail!("門檻值 ({}) 不能大於總簽署者數 ({})", min_signers, max_signers);
    }

    // 確保輸出目錄存在
    FileStore::ensure_dir(output_dir)?;

    // 生成金鑰
    let mut rng = thread_rng();
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )
    .context("金鑰生成失敗")?;

    println!("✓ 已生成 {} 個金鑰分片（門檻值：{}）\n", max_signers, min_signers);

    // 儲存每個金鑰分片
    for (identifier, key_package) in shares {
        let signer_id = u16::from(identifier);
        let share_path = output_dir.join(format!("share_{}.json", signer_id));

        FileStore::save_key_share(&share_path, signer_id, &key_package, min_signers, max_signers)?;

        println!("  📄 簽署者 {} → {}", signer_id, share_path.display());
    }

    // 儲存群組公鑰
    let pubkey_path = output_dir.join("pubkey.json");
    FileStore::save_public_key(&pubkey_path, &pubkey_package, min_signers, max_signers)?;

    let group_pubkey = pubkey_package.verifying_key();
    println!("\n  🔓 群組公鑰 → {}", pubkey_path.display());
    println!("     {}", hex::encode(group_pubkey.serialize()));

    println!("\n✅ 金鑰生成完成！");
    println!("\n💡 下一步：");
    println!("   1. 將金鑰分片分發給各個簽署者");
    println!("   2. 每個簽署者執行 'frost-cli round1' 開始簽章流程");

    Ok(())
}

/// 【Signer】Round 1: 生成承諾
fn cmd_round1(
    share_file: &std::path::Path,
    message_file: &std::path::Path,
    output: Option<&std::path::Path>,
    session_id: Option<&str>,
    verbose: bool,
) -> Result<()> {
    println!("🎲 Round 1: 生成 Nonce 承諾...\n");

    // 載入金鑰分片
    let key_package = FileStore::load_key_share(share_file)
        .context("無法載入金鑰分片")?;

    let signer_id = u16::from(key_package.identifier());
    println!("✓ 已載入簽署者 {} 的金鑰分片", signer_id);

    // 讀取訊息
    let message = FileStore::read_message(message_file)
        .context("無法讀取訊息檔案")?;

    println!("✓ 訊息: {} bytes", message.len());
    if verbose {
        println!("  內容預覽: {:?}", String::from_utf8_lossy(&message[..64.min(message.len())]));
    }

    // 生成 Session ID
    let session_id = session_id
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    println!("✓ Session ID: {}", session_id);

    // 生成 Nonce 和承諾（直接使用 FROST API）
    let mut rng = thread_rng();
    let (nonces, commitments) = frost::round1::commit(
        key_package.signing_share(),
        &mut rng,
    );

    println!("\n✓ 已生成 Nonce 承諾");
    if verbose {
        println!("  承諾 (hex): {}...", &hex::encode(commitments.serialize())[..32]);
    }

    // ⚠️ Demo Only: 持久化秘密 Nonce
    let nonce_path = NonceStore::save_nonce(&session_id, signer_id, &nonces)?;
    println!("  ⚠️  秘密 Nonce 已儲存到: {} (僅供 Demo!)", nonce_path.display());

    // 儲存承諾
    let output_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(format!("commitment_{}.json", signer_id)));

    FileStore::save_commitment(&output_path, &session_id, signer_id, &commitments, &message)?;

    println!("\n📄 承諾已儲存 → {}", output_path.display());
    println!("\n✅ Round 1 完成！");
    println!("\n💡 下一步：");
    println!("   將此承諾檔案交給協調者");

    Ok(())
}

/// 【Coordinator】建立簽章套件
fn cmd_create_package(
    commitment_files: &[std::path::PathBuf],
    message_file: &std::path::Path,
    output: &std::path::Path,
    verbose: bool,
) -> Result<()> {
    println!("📦 建立簽章套件...\n");

    // 驗證至少有 3 個承諾
    if commitment_files.len() < 3 {
        anyhow::bail!("至少需要 3 個承諾檔案，目前只有 {}", commitment_files.len());
    }

    // 讀取訊息
    let message = FileStore::read_message(message_file)
        .context("無法讀取訊息檔案")?;

    println!("✓ 訊息: {} bytes", message.len());

    // 載入所有承諾
    println!("\n收集承諾:");
    let mut commitments = Vec::new();
    let mut signer_ids = Vec::new();
    let mut session_id = None;

    for (i, commitment_file_path) in commitment_files.iter().enumerate() {
        let commitment_file = FileStore::load_commitment(commitment_file_path)
            .context(format!("無法載入承諾檔案 {}", commitment_file_path.display()))?;

        // 驗證 Session ID 一致
        if let Some(ref existing_session) = session_id {
            if existing_session != &commitment_file.session_id {
                anyhow::bail!("承諾檔案的 Session ID 不一致");
            }
        } else {
            session_id = Some(commitment_file.session_id.clone());
        }

        commitments.push(CommitmentData {
            signer_id: commitment_file.signer_id,
            commitment: commitment_file.commitment_hex.clone(),
        });

        signer_ids.push(commitment_file.signer_id);

        println!("  {} ✓ 簽署者 {} → {}", i + 1, commitment_file.signer_id, commitment_file_path.display());
    }

    let session_id = session_id.unwrap();
    println!("\n✓ 已收集 {} 個承諾", commitments.len());
    println!("✓ Session ID: {}", session_id);

    // 建立簽章套件
    let package_data = SigningPackageData {
        commitments,
        message: message.clone(),
    };

    // 儲存簽章套件
    FileStore::save_signing_package(output, &session_id, &package_data, signer_ids)?;

    println!("\n📄 簽章套件已儲存 → {}", output.display());
    println!("\n✅ 簽章套件建立完成！");
    println!("\n💡 下一步：");
    println!("   將簽章套件分發給所有參與的簽署者");
    println!("   每個簽署者執行 'frost-cli round2' 生成簽章分片");

    Ok(())
}

/// 【Signer】Round 2: 生成簽章分片
fn cmd_round2(
    share_file: &std::path::Path,
    package_file: &std::path::Path,
    output: Option<&std::path::Path>,
    session_id: &str,
    verbose: bool,
) -> Result<()> {
    println!("✍️  Round 2: 生成簽章分片...\n");

    // 載入金鑰分片
    let key_package = FileStore::load_key_share(share_file)
        .context("無法載入金鑰分片")?;

    let signer_id = u16::from(key_package.identifier());
    println!("✓ 已載入簽署者 {} 的金鑰分片", signer_id);

    // 載入簽章套件
    let package_file_data = FileStore::load_signing_package(package_file)
        .context("無法載入簽章套件")?;

    println!("✓ 已載入簽章套件");
    println!("  Session ID: {}", package_file_data.session_id);
    println!("  參與簽署者: {:?}", package_file_data.signer_ids);

    // 驗證 Session ID
    if package_file_data.session_id != session_id {
        anyhow::bail!(
            "Session ID 不匹配：預期 {}，實際 {}",
            session_id,
            package_file_data.session_id
        );
    }

    // 載入秘密 Nonce（從 Round 1 儲存的）
    let nonces = NonceStore::load_and_delete_nonce(session_id, signer_id)
        .context("無法載入秘密 Nonce。請確保已先執行 Round 1 並使用相同的 Session ID")?;

    println!("✓ 已載入並刪除秘密 Nonce（一次性使用）");

    // 重建 SigningPackage
    let message = hex::decode(&package_file_data.message_hex)
        .context("無法解碼訊息")?;

    let mut commitments_map = HashMap::new();
    for commitment_data in &package_file_data.commitments {
        let identifier = frost::Identifier::try_from(commitment_data.signer_id)
            .map_err(|e| anyhow::anyhow!("無效的簽署者 ID: {:?}", e))?;

        let commitment_bytes = hex::decode(&commitment_data.commitment)
            .context("無法解碼承諾 hex")?;

        let commitment = frost::round1::SigningCommitments::deserialize(&commitment_bytes)
            .map_err(|e| anyhow::anyhow!("無法反序列化承諾: {:?}", e))?;

        commitments_map.insert(identifier, commitment);
    }

    let signing_package = frost::SigningPackage::new(commitments_map, &message);

    // 生成簽章分片
    let signature_share = frost::round2::sign(&signing_package, &nonces, &key_package)
        .map_err(|e| anyhow::anyhow!("生成簽章分片失敗: {:?}", e))?;

    println!("\n✓ 已生成簽章分片");
    if verbose {
        println!("  分片 (hex): {}...", &hex::encode(signature_share.serialize())[..32]);
    }

    // 儲存簽章分片
    let output_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(format!("sig_share_{}.json", signer_id)));

    FileStore::save_signature_share(&output_path, session_id, signer_id, &signature_share)?;

    println!("\n📄 簽章分片已儲存 → {}", output_path.display());
    println!("\n✅ Round 2 完成！");
    println!("\n💡 下一步：");
    println!("   將簽章分片交給協調者進行聚合");

    Ok(())
}

/// 【Coordinator】聚合簽章
fn cmd_aggregate(
    package_file: &std::path::Path,
    share_files: &[std::path::PathBuf],
    pubkey_file: &std::path::Path,
    output: &std::path::Path,
    verbose: bool,
) -> Result<()> {
    println!("🔗 聚合簽章分片...\n");

    // 驗證至少有 3 個簽章分片
    if share_files.len() < 3 {
        anyhow::bail!("至少需要 3 個簽章分片，目前只有 {}", share_files.len());
    }

    // 載入簽章套件
    let package_file_data = FileStore::load_signing_package(package_file)
        .context("無法載入簽章套件")?;

    println!("✓ 已載入簽章套件");
    println!("  Session ID: {}", package_file_data.session_id);

    // 載入群組公鑰
    let pubkey_package = FileStore::load_public_key(pubkey_file)
        .context("無法載入群組公鑰")?;

    println!("✓ 已載入群組公鑰");

    // 載入簽章分片
    println!("\n收集簽章分片:");
    let signature_shares_map = FileStore::load_signature_shares_map(share_files)
        .context("載入簽章分片失敗")?;

    let signer_ids: Vec<u16> = signature_shares_map.keys().map(|id| u16::from(*id)).collect();
    for (i, id) in signer_ids.iter().enumerate() {
        println!("  {} ✓ 簽署者 {}", i + 1, id);
    }

    println!("\n✓ 已收集 {} 個簽章分片", signature_shares_map.len());

    // 重建 SigningPackage
    let message = hex::decode(&package_file_data.message_hex)
        .context("無法解碼訊息")?;

    let mut commitments_map = HashMap::new();
    for commitment_data in &package_file_data.commitments {
        let identifier = frost::Identifier::try_from(commitment_data.signer_id)
            .map_err(|e| anyhow::anyhow!("無效的簽署者 ID: {:?}", e))?;

        let commitment_bytes = hex::decode(&commitment_data.commitment)
            .context("無法解碼承諾 hex")?;

        let commitment = frost::round1::SigningCommitments::deserialize(&commitment_bytes)
            .map_err(|e| anyhow::anyhow!("無法反序列化承諾: {:?}", e))?;

        commitments_map.insert(identifier, commitment);
    }

    let signing_package = frost::SigningPackage::new(commitments_map, &message);

    // 建立 Coordinator 並聚合簽章
    let coordinator = Coordinator::new(pubkey_package, 3);

    let group_signature = coordinator.aggregate_signature(&signing_package, &signature_shares_map)
        .map_err(|e| anyhow::anyhow!("聚合簽章失敗: {}", e))?;

    println!("\n✓ 簽章聚合成功");
    println!("  簽章 (hex): {}", hex::encode(group_signature.serialize()));

    // 驗證簽章
    coordinator.verify_signature(&message, &group_signature)
        .map_err(|e| anyhow::anyhow!("簽章驗證失敗: {}", e))?;

    println!("✓ 簽章驗證通過");

    // 儲存簽章
    FileStore::save_signature(
        output,
        &package_file_data.session_id,
        &group_signature,
        &message,
        signer_ids,
    )?;

    println!("\n📄 最終簽章已儲存 → {}", output.display());
    println!("\n🎉 簽章聚合完成！");
    println!("\n💡 下一步：");
    println!("   使用 'frost-cli verify' 驗證簽章");

    Ok(())
}

/// 【Anyone】驗證簽章
fn cmd_verify(
    signature_file: &std::path::Path,
    message_file: &std::path::Path,
    pubkey_file: &std::path::Path,
    verbose: bool,
) -> Result<()> {
    println!("✅ 驗證簽章...\n");

    // 載入簽章
    let signature_data = FileStore::load_signature(signature_file)
        .context("無法載入簽章檔案")?;

    println!("✓ 已載入簽章");
    println!("  Session ID: {}", signature_data.session_id);
    println!("  參與簽署者: {:?}", signature_data.signer_ids);

    let signature = FileStore::deserialize_signature(&signature_data.signature_hex)
        .context("無法反序列化簽章")?;

    // 載入訊息
    let message = FileStore::read_message(message_file)
        .context("無法讀取訊息檔案")?;

    let stored_message = hex::decode(&signature_data.message_hex)
        .context("無法解碼儲存的訊息")?;

    if message != stored_message {
        anyhow::bail!("訊息不匹配！簽章檔案中的訊息與提供的訊息檔案不同");
    }

    println!("✓ 訊息: {} bytes", message.len());

    // 載入群組公鑰
    let pubkey_package = FileStore::load_public_key(pubkey_file)
        .context("無法載入群組公鑰")?;

    let group_pubkey = pubkey_package.verifying_key();
    println!("✓ 群組公鑰: {}...", &hex::encode(group_pubkey.serialize())[..32]);

    // 驗證簽章
    println!("\n開始驗證...");

    match group_pubkey.verify(&message, &signature) {
        Ok(_) => {
            println!("\n🎊 簽章驗證成功！");
            println!("\n✓ 此訊息確實由至少 3 個簽署者共同簽署");
            println!("✓ 簽章有效且未被篡改");
            println!("✓ 參與簽署者: {:?}", signature_data.signer_ids);
            Ok(())
        }
        Err(e) => {
            println!("\n❌ 簽章驗證失敗");
            Err(anyhow::anyhow!("驗證失敗: {:?}", e))
        }
    }
}

/// 【Demo】完整流程展示
///
/// 在單一 process 內模擬完整的 3-of-5 FROST 簽章流程。
/// 使用 SimulatedLoRaTransport 模擬真實的無線傳輸環境。
/// 同時啟動 HTTP Server 提供 Dashboard 查詢介面。
///
/// ## 流程說明
///
/// 1. **啟動 HTTP Server**：在背景啟動 API 服務（port 3000）
/// 2. **Setup 階段**：使用 Trusted Dealer 生成 5 個金鑰分片
/// 3. **Round 1**：參與的簽署者生成 Nonce 承諾（透過 LoRa 傳輸）
/// 4. **建立簽章套件**：協調者收集所有承諾
/// 5. **Round 2**：簽署者生成簽章分片
/// 6. **聚合簽章**：協調者聚合所有分片
/// 7. **驗證簽章**：使用群組公鑰驗證
///
/// ## 新功能
///
/// - ✅ SimulatedLoRaTransport：模擬延遲、掉包、分片
/// - ✅ HTTP API：提供 /status 端點給 Dashboard 查詢
/// - ✅ 即時狀態追蹤：記錄所有傳輸事件
async fn cmd_demo_basic(message: &str, signer_ids: &[u16], full_payload: bool) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║   FROST 3-of-5 門檻簽章 - 完整流程展示                        ║");
    println!("║   Demo for bitcoin++ Taipei 2025                              ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // 驗證參數
    if signer_ids.len() < 3 {
        anyhow::bail!("至少需要 3 個簽署者參與，目前只有 {}", signer_ids.len());
    }

    if signer_ids.len() > 5 {
        anyhow::bail!("最多只能有 5 個簽署者參與，目前有 {}", signer_ids.len());
    }

    for &id in signer_ids {
        if id < 1 || id > 5 {
            anyhow::bail!("簽署者 ID 必須在 1-5 之間，收到: {}", id);
        }
    }

    println!("📋 配置:");
    println!("   訊息: \"{}\"", message);
    println!("   參與簽署者: {:?}", signer_ids);
    println!("   門檻配置: 3-of-5");
    println!();

    // ========================================================================
    // 初始化 SimulatedLoRaTransport
    // ========================================================================
    let mut transport = SimulatedLoRaTransport::new();
    let lora_state = transport.get_state();

    println!("🔧 初始化 Transport 抽象層...");
    println!("   ✓ 使用 SimulatedLoRaTransport");
    println!("   ✓ 延遲: 500ms per packet");
    println!("   ✓ 掉包率: 10%");
    println!("   ✓ 分片大小: 64 bytes");
    println!();

    // ========================================================================
    // 啟動 HTTP Server（背景執行）
    // ========================================================================
    println!("🌐 啟動 HTTP API Server...");

    let lora_state_clone = Arc::clone(&lora_state);
    let server_handle = tokio::spawn(async move {
        start_http_server(lora_state_clone).await
    });

    println!("   ✓ Server 運行在 http://127.0.0.1:3000");
    println!("   ✓ Dashboard: 在瀏覽器開啟 dashboard.html");
    println!("   ✓ API 端點: GET /status");
    println!();

    // 等待一下讓 server 啟動
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // ========================================================================
    // Setup: Trusted Dealer 金鑰生成
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 1: Setup - Trusted Dealer 金鑰生成                       ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let max_signers = 5;
    let min_signers = 3;

    let mut rng = thread_rng();

    println!("🔑 生成 FROST 金鑰分片...");
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )
    .context("金鑰生成失敗")?;

    let group_pubkey = pubkey_package.verifying_key();
    println!("✓ 已生成 {} 個金鑰分片（門檻值：{}）", max_signers, min_signers);
    println!("✓ 群組公鑰: {}...", &hex::encode(group_pubkey.serialize())[..32]);
    println!();

    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    // ========================================================================
    // 建立 Coordinator 和 Signers
    // ========================================================================
    println!("🏗️  建立協調者和簽署者...");

    let coordinator = Coordinator::new(pubkey_package, min_signers);

    let mut signers = HashMap::new();
    for (identifier, key_package) in shares {
        let signer_id = u16::from(identifier);
        signers.insert(signer_id, Signer::new(key_package));
        println!("   ✓ 簽署者 {} 已就緒", signer_id);
    }

    println!();
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // ========================================================================
    // Round 1: 生成承諾
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 2: Round 1 - 生成 Nonce 承諾                            ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("📝 為什麼需要 Round 1？");
    println!("   FROST 使用 Commitment-Reveal 模式防止惡意簽署者操縱 nonce：");
    println!("   1. 所有簽署者先提交承諾（無法修改）");
    println!("   2. 然後在 Round 2 才揭露如何使用 nonce");
    println!("   3. 這確保了簽章的不可偽造性");
    println!();

    let session_id = SessionId::new();
    println!("🎲 Session ID: {}", session_id);
    println!();

    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    let mut commitments_map = HashMap::new();
    let mut commitment_data_vec = Vec::new();

    for &signer_id in signer_ids {
        let signer = signers.get(&signer_id).unwrap();

        println!("👤 簽署者 {} 正在生成承諾...", signer_id);
        let commitment = signer
            .commit(session_id)
            .context(format!("簽署者 {} 生成承諾失敗", signer_id))?;

        let commitment_hex = hex::encode(commitment.serialize());

        // 模擬傳輸：Signer -> Coordinator
        transport.send(
            MessageMetadata {
                from: format!("signer_{}", signer_id),
                to: "coordinator".to_string(),
                message_type: MessageType::Round1Commitment,
                timestamp: Some(chrono::Utc::now()),
            },
            &commitment_hex,
        );

        commitments_map.insert(frost::Identifier::try_from(signer_id).unwrap(), commitment);
        commitment_data_vec.push(CommitmentData {
            signer_id,
            commitment: commitment_hex,
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("✓ Round 1 完成！已收集 {} 個承諾\n", commitments_map.len());
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ========================================================================
    // 建立簽章套件
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 3: 建立簽章套件                                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let message_bytes = message.as_bytes();

    println!("📦 協調者正在建立簽章套件...");
    let signing_package = frost::SigningPackage::new(commitments_map.clone(), message_bytes);

    let signing_package_data = SigningPackageData {
        commitments: commitment_data_vec.clone(),
        message: message_bytes.to_vec(),
    };

    // 模擬傳輸：Coordinator -> Signers (廣播)
    let package_json = serde_json::to_string(&signing_package_data)
        .context("無法序列化簽章套件")?;

    for &signer_id in signer_ids {
        transport.send(
            MessageMetadata {
                from: "coordinator".to_string(),
                to: format!("signer_{}", signer_id),
                message_type: MessageType::SigningPackage,
                timestamp: Some(chrono::Utc::now()),
            },
            &package_json[..64.min(package_json.len())],  // 只顯示部分內容
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("✓ 簽章套件已分發給所有參與的簽署者\n");
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ========================================================================
    // Round 2: 生成簽章分片
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 4: Round 2 - 生成簽章分片                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("📝 Round 2 做什麼？");
    println!("   每個簽署者使用：");
    println!("   - 自己的金鑰分片（私密）");
    println!("   - Round 1 的秘密 nonce（一次性使用）");
    println!("   - 簽章套件（包含所有承諾和訊息）");
    println!("   生成一個簽章分片，傳回給協調者。");
    println!();

    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    let mut signature_shares_map = HashMap::new();

    for &signer_id in signer_ids {
        let signer = signers.get(&signer_id).unwrap();

        println!("✍️  簽署者 {} 正在生成簽章分片...", signer_id);
        let signature_share = signer
            .sign(session_id, &signing_package_data)
            .context(format!("簽署者 {} 生成簽章分片失敗", signer_id))?;

        let share_hex = hex::encode(signature_share.serialize());

        // 模擬傳輸：Signer -> Coordinator
        transport.send(
            MessageMetadata {
                from: format!("signer_{}", signer_id),
                to: "coordinator".to_string(),
                message_type: MessageType::Round2SignatureShare,
                timestamp: Some(chrono::Utc::now()),
            },
            &share_hex,
        );

        signature_shares_map.insert(frost::Identifier::try_from(signer_id).unwrap(), signature_share);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("✓ Round 2 完成！已收集 {} 個簽章分片\n", signature_shares_map.len());
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ========================================================================
    // 聚合簽章
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 5: 聚合簽章                                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("🔗 協調者正在聚合簽章分片...");
    let group_signature = coordinator
        .aggregate_signature(&signing_package, &signature_shares_map)
        .context("聚合簽章失敗")?;

    let signature_hex = hex::encode(group_signature.serialize());
    println!("✓ 簽章聚合成功！");
    println!("   簽章 (hex): {}", signature_hex);
    println!();

    // 模擬傳輸：Coordinator -> 廣播
    transport.send(
        MessageMetadata {
            from: "coordinator".to_string(),
            to: "broadcast".to_string(),
            message_type: MessageType::FinalSignature,
            timestamp: Some(chrono::Utc::now()),
        },
        &signature_hex,
    );

    std::thread::sleep(std::time::Duration::from_millis(500));

    // ========================================================================
    // 驗證簽章
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  階段 6: 驗證簽章                                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("🔍 使用群組公鑰驗證簽章...");
    coordinator
        .verify_signature(message_bytes, &group_signature)
        .context("簽章驗證失敗")?;

    println!("✓ 簽章驗證通過！\n");
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ========================================================================
    // 統計資訊
    // ========================================================================
    if let Some(stats) = transport.get_stats() {
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║  傳輸統計                                                     ║");
        println!("╚════════════════════════════════════════════════════════════════╝\n");

        println!("📊 總訊息數: {}", stats.total_messages);
        println!("📊 總位元組數: {}", stats.total_bytes);
        println!("\n訊息類型分布:");
        for (msg_type, count) in &stats.by_type {
            println!("   - {:?}: {} 個", msg_type, count);
        }
        println!();
    }

    // ========================================================================
    // 總結
    // ========================================================================
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║   🎉 FROST 3-of-5 門檻簽章展示完成！                          ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("✅ 成功完成以下步驟:");
    println!("   1. ✓ 生成 5 個金鑰分片（3-of-5 門檻）");
    println!("   2. ✓ {} 個簽署者參與簽章", signer_ids.len());
    println!("   3. ✓ Round 1: 生成並收集承諾");
    println!("   4. ✓ 建立並分發簽章套件");
    println!("   5. ✓ Round 2: 生成並收集簽章分片");
    println!("   6. ✓ 聚合最終簽章");
    println!("   7. ✓ 驗證簽章有效性");
    println!();

    println!("🔐 這就是 FROST 門檻簽章！");
    println!("   - 任意 3 個簽署者可以合作產生合法的 Schnorr 簽章");
    println!("   - 協調者永遠不會接觸到任何私鑰分片");
    println!("   - 簽章與單一金鑰產生的簽章無法區分（隱私保護）");
    println!();

    println!("💡 已實現:");
    println!("   ✅ SimulatedLoRaTransport（模擬低頻寬傳輸）");
    println!("   ✅ HTTP Dashboard（即時視覺化傳輸過程）");
    println!();

    println!("🌐 HTTP Server 仍在運行...");
    println!("   按 Ctrl+C 停止 Server 並結束程式");
    println!();

    // 讓 server 繼續運行，等待用戶按 Ctrl+C
    server_handle.await.ok();

    Ok(())
}

// ============================================================================
// HTTP Server - 提供 Dashboard API
// ============================================================================

/// 啟動 HTTP Server 提供 /status API
async fn start_http_server(
    lora_state: Arc<Mutex<LoRaTransportState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::{
        extract::State,
        http::Method,
        response::Json,
        routing::get,
        Router,
    };
    use tower_http::cors::{Any, CorsLayer};

    // CORS 設定（允許本地 HTML 存取）
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    // 創建 Router
    let app = Router::new()
        .route("/status", get(get_status))
        .layer(cors)
        .with_state(lora_state);

    // 綁定到 127.0.0.1:3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

/// GET /status - 回傳當前 LoRa 傳輸狀態
async fn get_status(
    State(lora_state): State<Arc<Mutex<LoRaTransportState>>>,
) -> Json<LoRaTransportState> {
    let state = lora_state.lock().unwrap();
    Json(state.clone())
}
