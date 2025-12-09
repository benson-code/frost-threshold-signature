# FROST-T Demo - Transport 抽象層展示

> 為 bitcoin++ Taipei 2025 黑客松準備的 FROST 門檻簽章展示專案

## 🎯 專案目標

展示一個完整的 FROST 3-of-5 門檻簽章流程，並透過 **Transport 抽象層** 視覺化訊息傳遞的過程。

## 🏗️ 專案架構

```
frost-threshold-signature/
├── src/
│   ├── lib.rs                 # 主入口，定義模組結構
│   ├── coordinator.rs         # 協調者邏輯（編排流程，不持有私鑰）
│   ├── signer.rs             # 簽署者邏輯（管理金鑰分片和 Nonce）
│   ├── api.rs                # API 資料結構
│   ├── transport/            # 🆕 Transport 抽象層
│   │   └── mod.rs            # Transport trait + StdoutTransport
│   ├── cli/                  # CLI 工具
│   │   ├── commands.rs       # 命令定義（包含 demo-basic）
│   │   ├── file_store.rs     # 檔案序列化
│   │   └── nonce_store.rs    # Nonce 儲存
│   ├── bin/
│   │   └── frost-cli.rs      # CLI 主程式（包含 cmd_demo_basic）
│   └── main.rs               # HTTP API 服務器
├── Cargo.toml                # 依賴管理
└── demo-basic.bat            # 🆕 快速執行 Demo 腳本
```

## ✨ 新增功能：Transport 抽象層

### 設計理念

Transport 抽象層將「訊息傳遞」與「FROST 協議邏輯」分離，提供：

1. **解耦設計**：協議不需要關心底層如何傳輸
2. **可視覺化**：每個傳輸事件都可以被記錄和展示
3. **易於擴展**：未來可以輕鬆添加不同的傳輸實作

### Transport Trait 定義

```rust
pub trait Transport {
    fn send(&mut self, metadata: MessageMetadata, payload: &str);
    fn get_stats(&self) -> Option<TransportStats>;
    fn reset(&mut self);
}
```

### 目前實作

- ✅ **StdoutTransport**：將訊息印到終端機（用於 Demo）

### 未來擴展

- ⏳ **SimulatedLoRaTransport**：模擬低頻寬、延遲、掉包
- ⏳ **FileTransport**：透過檔案系統傳遞
- ⏳ **HttpDashboardTransport**：提供 `/status` API 給前端查詢

## 🚀 快速開始

### 執行 Demo

最簡單的方式：

```bash
# Windows
demo-basic.bat

# 或使用 cargo
cargo run --bin frost-cli -- demo-basic
```

### 自訂參數

```bash
# 自訂訊息
cargo run --bin frost-cli -- demo-basic -m "Hello, bitcoin++"

# 選擇不同的簽署者（例如：1, 3, 5）
cargo run --bin frost-cli -- demo-basic --signers 1,3,5

# 顯示完整的 payload（hex 數據）
cargo run --bin frost-cli -- demo-basic --full-payload
```

### 查看幫助

```bash
cargo run --bin frost-cli -- demo-basic --help
```

## 📺 Demo 流程展示

執行 `demo-basic` 命令時，你會看到以下流程：

```
╔════════════════════════════════════════════════════════════════╗
║   FROST 3-of-5 門檻簽章 - 完整流程展示                        ║
║   Demo for bitcoin++ Taipei 2025                              ║
╚════════════════════════════════════════════════════════════════╝

階段 1: Setup - Trusted Dealer 金鑰生成
  ✓ 已生成 5 個金鑰分片（門檻值：3）
  ✓ 群組公鑰: 03a1b2c3d4e5...

階段 2: Round 1 - 生成 Nonce 承諾
  📝 為什麼需要 Round 1？
     FROST 使用 Commitment-Reveal 模式防止惡意簽署者操縱 nonce

  📡 [Round1Commitment] signer_1 → coordinator
     📦 Payload: deadbeef123456...

階段 3: 建立簽章套件
  📦 協調者正在建立簽章套件...

  📡 [SigningPackage] coordinator → signer_1
  📡 [SigningPackage] coordinator → signer_2
  📡 [SigningPackage] coordinator → signer_3

階段 4: Round 2 - 生成簽章分片
  📝 Round 2 做什麼？
     每個簽署者使用金鑰分片 + 秘密 nonce + 簽章套件生成簽章分片

  📡 [Round2SignatureShare] signer_1 → coordinator

階段 5: 聚合簽章
  ✓ 簽章聚合成功！

  📡 [FinalSignature] coordinator → broadcast

階段 6: 驗證簽章
  ✓ 簽章驗證通過！

╔════════════════════════════════════════════════════════════════╗
║  傳輸統計                                                     ║
╚════════════════════════════════════════════════════════════════╝

📊 總訊息數: 10
📊 總位元組數: 4567

訊息類型分布:
   - Round1Commitment: 3 個
   - SigningPackage: 3 個
   - Round2SignatureShare: 3 個
   - FinalSignature: 1 個
```

## 🔧 完整的 CLI 工具

除了 `demo-basic`，專案還提供完整的 CLI 工具鏈：

```bash
# 1. 生成金鑰分片
cargo run --bin frost-cli -- keygen

# 2. Round 1（每個簽署者）
cargo run --bin frost-cli -- round1 -s share_1.json -m message.txt

# 3. 建立簽章套件（協調者）
cargo run --bin frost-cli -- create-pkg -c commitment_*.json -m message.txt

# 4. Round 2（每個簽署者）
cargo run --bin frost-cli -- round2 -s share_1.json -p signing_package.json --session-id xxx

# 5. 聚合簽章（協調者）
cargo run --bin frost-cli -- aggregate -p signing_package.json -s sig_share_*.json -k pubkey.json

# 6. 驗證簽章（任何人）
cargo run --bin frost-cli -- verify -s signature.json -m message.txt -k pubkey.json
```

## 🌟 教學重點（給潛在雇主看）

### 1. 密碼學理解

- ✅ 理解 FROST 協議的兩輪次流程
- ✅ 理解為什麼需要 Commitment-Reveal 模式
- ✅ 理解 Nonce 重用的災難性後果
- ✅ 理解協調者為什麼不需要持有私鑰

### 2. Rust 工程能力

- ✅ 使用 trait 設計抽象介面
- ✅ 清晰的模組化架構
- ✅ 完整的錯誤處理（thiserror, anyhow）
- ✅ 良好的註解和文檔

### 3. 系統設計

- ✅ **Transport 抽象層**：為未來擴展預留空間
- ✅ **同步與異步混合**：CLI 用同步，HTTP API 用異步
- ✅ **狀態管理**：使用 DashMap 實現並發安全

### 4. 實務經驗

- ✅ 使用真實的密碼學函式庫（`frost-secp256k1`）
- ✅ 考慮實際場景（離線簽章、低頻寬傳輸）
- ✅ 提供完整的 Demo 和文檔

## 🎬 bitcoin++ Taipei 2025 Demo 腳本

### 30 秒版本

```bash
# 直接執行
cargo run --bin frost-cli -- demo-basic
```

說明：
1. 展示完整流程（Setup → Round 1 → Round 2 → Aggregate → Verify）
2. 透過 Transport 抽象層視覺化訊息傳遞
3. 展示統計資訊

### 2 分鐘版本

```bash
# 1. 展示基本流程
cargo run --bin frost-cli -- demo-basic

# 2. 展示不同的簽署者組合
cargo run --bin frost-cli -- demo-basic --signers 2,4,5

# 3. 展示完整的 payload（技術細節）
cargo run --bin frost-cli -- demo-basic --full-payload
```

### 5 分鐘版本（包含解說）

1. **說明場景**：多方簽名錢包、離線簽章
2. **執行 demo-basic**：展示完整流程
3. **解釋 Transport 抽象層**：未來可以接 LoRa、NFC、QR Code
4. **展示 CLI 工具鏈**：展示如何在多個終端模擬離線簽章
5. **未來規劃**：SimulatedLoRaTransport + HTTP Dashboard

## 📚 技術細節

### FROST 協議簡介

FROST（Flexible Round-Optimized Schnorr Threshold）是一個兩輪次的門檻簽章協議：

- **Setup**：Trusted Dealer 生成 n 個金鑰分片
- **Round 1**：參與者生成 Nonce 承諾（防止惡意操縱）
- **Round 2**：參與者使用承諾生成簽章分片
- **Aggregate**：協調者聚合簽章分片
- **Verify**：使用群組公鑰驗證簽章

### 為什麼選擇 FROST？

1. **比特幣相容**：使用 secp256k1 曲線，與比特幣 Schnorr 簽章（Taproot）相容
2. **隱私保護**：門檻簽章與單一金鑰簽章無法區分
3. **靈活性**：支援任意的 t-of-n 配置
4. **高效性**：只需要兩輪通訊

## 🔮 未來擴展

### Phase 2: 虛擬 LoRa 傳輸

```rust
// src/transport/simulated_lora.rs
pub struct SimulatedLoRaTransport {
    latency_ms: u64,          // 延遲
    packet_loss_rate: f64,     // 掉包率
    bandwidth_limit: usize,    // 頻寬限制
    fragmentation_size: usize, // 分片大小
}
```

特性：
- 模擬低頻寬環境（例如：250 bytes/s）
- 模擬封包延遲和掉包
- 模擬大型訊息的分片傳輸

### Phase 3: HTTP Dashboard

```
GET /status
{
  "current_phase": "Round1",
  "progress": {
    "round1_commitments": 2,
    "expected": 3
  },
  "transport_events": [
    {"from": "signer_1", "to": "coordinator", "type": "Round1Commitment", "timestamp": "..."}
  ]
}
```

前端 Dashboard：
- 即時顯示傳輸事件
- 視覺化協議流程
- 展示統計圖表

## 📞 聯絡方式

如果你對這個專案有興趣，或者想討論比特幣相關的工作機會：

- GitHub: [benson-code]
- 專案: [frost-threshold-signature](https://github.com/benson-code/frost-threshold-signature)

## 📄 授權

MIT License - 開源專案，歡迎使用和貢獻！
