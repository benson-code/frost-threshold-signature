//! # CLI 命令定義
//!
//! 使用 clap derive API 定義所有 CLI 命令和參數。
//!
//! ## 命令結構
//!
//! ```text
//! frost-cli
//! ├── keygen       - 生成金鑰分片（Dealer 角色）
//! ├── round1       - Round 1: 生成承諾（Signer 角色）
//! ├── create-pkg   - 建立簽章套件（Coordinator 角色）
//! ├── round2       - Round 2: 生成簽章分片（Signer 角色）
//! ├── aggregate    - 聚合最終簽章（Coordinator 角色）
//! └── verify       - 驗證簽章（任何人）
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;

// ============================================================================
// 主 CLI 結構
// ============================================================================

/// FROST 3-of-5 門檻簽章 CLI 工具
///
/// 這是一個離線可用的 FROST 門檻簽章工具，支援在多個終端視窗模擬不同角色。
/// 所有中間結果通過 JSON 檔案傳遞，適合演示和測試。
#[derive(Parser, Debug)]
#[command(
    name = "frost-cli",
    version,
    about = "FROST 3-of-5 Threshold Signature CLI Tool",
    long_about = "A command-line tool for FROST threshold signatures. \
                  Supports offline operation through file-based communication. \
                  Perfect for demos and testing on a single machine."
)]
pub struct Cli {
    /// 全域選項：詳細輸出
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

// ============================================================================
// 命令定義
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 【Dealer】生成金鑰分片
    ///
    /// 使用 Trusted Dealer 方法生成 5 個金鑰分片和群組公鑰。
    /// 這一步只需執行一次（Setup 階段）。
    ///
    /// 輸出檔案：
    /// - {output-dir}/share_{1..5}.json - 5 個金鑰分片
    /// - {output-dir}/pubkey.json - 群組公鑰套件
    Keygen {
        /// 輸出目錄（預設：./frost-data）
        #[arg(short, long, default_value = "frost-data")]
        output_dir: PathBuf,

        /// 總簽署者數量（預設：5）
        #[arg(long, default_value = "5")]
        max_signers: u16,

        /// 門檻值 - 最少需要的簽署者數量（預設：3）
        #[arg(long, default_value = "3")]
        min_signers: u16,
    },

    /// 【Signer】Round 1: 生成承諾
    ///
    /// 每個參與的簽署者運行此命令，生成 Nonce 承諾。
    /// 秘密 Nonce 會儲存在本地，公開承諾寫入檔案。
    ///
    /// 輸入檔案：
    /// - {share-file} - 此簽署者的金鑰分片（來自 keygen）
    /// - {message-file} - 要簽署的訊息
    ///
    /// 輸出檔案：
    /// - {output} - 公開承諾（JSON）
    Round1 {
        /// 此簽署者的金鑰分片檔案
        #[arg(short, long)]
        share_file: PathBuf,

        /// 要簽署的訊息檔案（文字或二進位）
        #[arg(short, long)]
        message_file: PathBuf,

        /// 輸出的承諾檔案（預設：commitment_{id}.json）
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Session ID（可選，預設自動生成）
        #[arg(long)]
        session_id: Option<String>,
    },

    /// 【Coordinator】建立簽章套件
    ///
    /// 協調者收集所有 Round 1 的承諾，建立簽章套件。
    /// 這個套件將分發給所有參與的簽署者用於 Round 2。
    ///
    /// 輸入檔案：
    /// - {commitment-files...} - 所有參與者的承諾檔案（至少 3 個）
    /// - {message-file} - 要簽署的訊息（必須與 Round 1 相同）
    ///
    /// 輸出檔案：
    /// - {output} - 簽章套件（JSON）
    CreatePackage {
        /// 承諾檔案清單（支援 glob pattern）
        #[arg(short, long, num_args = 1..)]
        commitment_files: Vec<PathBuf>,

        /// 要簽署的訊息檔案
        #[arg(short, long)]
        message_file: PathBuf,

        /// 輸出的簽章套件檔案
        #[arg(short, long, default_value = "signing_package.json")]
        output: PathBuf,
    },

    /// 【Signer】Round 2: 生成簽章分片
    ///
    /// 每個簽署者使用簽章套件生成自己的簽章分片。
    /// 此步驟會消費（刪除）Round 1 產生的秘密 Nonce。
    ///
    /// 輸入檔案：
    /// - {share-file} - 此簽署者的金鑰分片
    /// - {package-file} - 簽章套件（來自 create-package）
    ///
    /// 輸出檔案：
    /// - {output} - 簽章分片（JSON）
    Round2 {
        /// 此簽署者的金鑰分片檔案
        #[arg(short, long)]
        share_file: PathBuf,

        /// 簽章套件檔案
        #[arg(short, long)]
        package_file: PathBuf,

        /// 輸出的簽章分片檔案（預設：sig_share_{id}.json）
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Session ID（必須與 Round 1 相同）
        #[arg(long)]
        session_id: String,
    },

    /// 【Coordinator】聚合簽章
    ///
    /// 協調者收集所有簽章分片，聚合成最終的 Schnorr 簽章。
    ///
    /// 輸入檔案：
    /// - {package-file} - 簽章套件
    /// - {share-files...} - 所有簽章分片（至少 3 個）
    /// - {pubkey-file} - 群組公鑰套件（來自 keygen）
    ///
    /// 輸出檔案：
    /// - {output} - 最終簽章（JSON）
    Aggregate {
        /// 簽章套件檔案
        #[arg(short, long)]
        package_file: PathBuf,

        /// 簽章分片檔案清單
        #[arg(short = 's', long, num_args = 1..)]
        share_files: Vec<PathBuf>,

        /// 群組公鑰檔案
        #[arg(short = 'k', long)]
        pubkey_file: PathBuf,

        /// 輸出的簽章檔案
        #[arg(short, long, default_value = "signature.json")]
        output: PathBuf,
    },

    /// 【Anyone】驗證簽章
    ///
    /// 使用群組公鑰驗證簽章的有效性。
    /// 任何人都可以執行此命令，無需持有私鑰。
    ///
    /// 輸入檔案：
    /// - {signature-file} - 簽章檔案（來自 aggregate）
    /// - {message-file} - 原始訊息
    /// - {pubkey-file} - 群組公鑰
    Verify {
        /// 簽章檔案
        #[arg(short, long)]
        signature_file: PathBuf,

        /// 訊息檔案
        #[arg(short, long)]
        message_file: PathBuf,

        /// 群組公鑰檔案
        #[arg(short = 'k', long)]
        pubkey_file: PathBuf,
    },

    /// 【Demo】完整流程展示
    ///
    /// 在單一 process 內模擬完整的 3-of-5 FROST 簽章流程。
    /// 使用 Transport 抽象層展示訊息傳遞的過程。
    ///
    /// 這個命令會：
    /// 1. 生成 5 個金鑰分片（3-of-5 門檻）
    /// 2. 選擇 3 個簽署者參與簽章
    /// 3. 執行 Round 1（生成承諾）
    /// 4. 建立簽章套件
    /// 5. 執行 Round 2（生成簽章分片）
    /// 6. 聚合並驗證最終簽章
    ///
    /// 適合用於：
    /// - 快速驗證 FROST 流程是否正常運作
    /// - 展示給觀眾看（bitcoin++ hackathon demo）
    /// - 理解 FROST 協議的完整流程
    DemoBasic {
        /// 要簽署的訊息（預設："Hello, FROST!"）
        #[arg(short, long, default_value = "Hello, FROST!")]
        message: String,

        /// 參與簽署的簽署者 ID（預設：1,2,3）
        #[arg(long, value_delimiter = ',', default_values_t = vec![1, 2, 3])]
        signers: Vec<u16>,

        /// 是否顯示完整的 payload（預設：false）
        #[arg(long)]
        full_payload: bool,
    },
}

// ============================================================================
// 輔助方法
// ============================================================================

impl Cli {
    /// 解析命令列參數
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

impl Commands {
    /// 獲取命令名稱（用於日誌）
    pub fn name(&self) -> &str {
        match self {
            Commands::Keygen { .. } => "keygen",
            Commands::Round1 { .. } => "round1",
            Commands::CreatePackage { .. } => "create-package",
            Commands::Round2 { .. } => "round2",
            Commands::Aggregate { .. } => "aggregate",
            Commands::Verify { .. } => "verify",
            Commands::DemoBasic { .. } => "demo-basic",
        }
    }
}
