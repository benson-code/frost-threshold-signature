# FROST-T 🚀

> **Bitcoin-Compatible 3-of-5 Threshold Signature with Simulated LoRa Transport**

<div align="center">

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Bitcoin++](https://img.shields.io/badge/bitcoin++-Taipei%202025-orange?style=for-the-badge)](https://btcplusplus.dev/)

![Status](https://img.shields.io/badge/Status-Phase%202%20Complete-success?style=for-the-badge)
![Demo](https://img.shields.io/badge/Demo-Ready-brightgreen?style=for-the-badge)

**[English](#english)** | **[中文](#中文)**

</div>

---

<a name="english"></a>

## 🎯 What is FROST-T?

**FROST-T** (FROST Terminal) is a complete implementation of the **FROST (Flexible Round-Optimized Schnorr Threshold)** signature protocol with two major innovations:

1. **Simulated LoRa Transport**: Realistic wireless transmission simulation with latency, packet loss, and fragmentation
2. **Cyberpunk Dashboard**: Real-time visualization with retro terminal aesthetics

Perfect for demonstrating offline multi-party wallets and long-range communication scenarios.

### ⚡ Quick Demo

```bash
# One command to see everything!
demo-basic.bat
```

This launches:
- ✅ Full 3-of-5 FROST signing flow
- ✅ Simulated LoRa transmission (500ms latency, 10% packet loss, 64-byte chunks)
- ✅ HTTP API server on port 3000
- ✅ Cyberpunk dashboard in your browser

---

## ✨ Features

### 🔐 Core FROST Implementation

- **3-of-5 Threshold**: Any 3 out of 5 signers can create a valid signature
- **Bitcoin Compatible**: Uses secp256k1 curve (Taproot/Schnorr compatible)
- **Two-Round Protocol**: Efficient 2-round communication
- **Privacy Preserving**: Threshold signatures look identical to single-key signatures
- **Nonce Safety**: Automatic one-time nonce enforcement

### 📡 Simulated LoRa Transport

```rust
✓ Latency:        500ms per packet
✓ Packet Loss:    10% drop rate with auto-retry (max 3 attempts)
✓ Fragmentation:  64-byte chunks (LoRa SF7 typical)
✓ Event Tracking: Real-time logging for dashboard
✓ Shared State:   Thread-safe Arc<Mutex> for monitoring
```

### 🎨 Cyberpunk Dashboard

<div align="center">
  <img src="https://via.placeholder.com/800x400/000000/00ff00?text=FROST-T+Dashboard+%7C+Real-time+Visualization" alt="Dashboard Preview" width="80%">
</div>

**Features:**
- 📈 **Progress Bar**: Live transmission progress with shimmer
- 📶 **RSSI Meter**: Signal strength (-120 to -30 dBm)
- 🌊 **Spectrum Analyzer**: 50-bar animated visualization
- 📋 **Event Log**: Last 20 events with timestamps
- 📊 **Statistics**: Messages, bytes, retries, success rate

**Theme:**
- Black background + phosphor green (#0f0)
- CRT scanline effects
- Neon glow animations
- Blinking cursor █

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows
# Download from: https://rustup.rs/
```

### Installation

```bash
git clone https://github.com/benson-code/frost-threshold-signature.git
cd frost-threshold-signature
cargo build --release
```

### Run the Demo

**Windows (One-Click):**
```bash
demo-basic.bat
```

**Manual (Cross-platform):**
```bash
# Terminal 1: Open dashboard
start dashboard.html   # Windows
open dashboard.html    # macOS
xdg-open dashboard.html  # Linux

# Terminal 2: Run demo
cargo run --bin frost-cli -- demo-basic
```

**Custom Parameters:**
```bash
# Custom message
cargo run --bin frost-cli -- demo-basic -m "Hello bitcoin++"

# Different signers (e.g., 2, 4, 5)
cargo run --bin frost-cli -- demo-basic --signers 2,4,5

# Show full hex payloads
cargo run --bin frost-cli -- demo-basic --full-payload
```

---

## 📺 What You'll See

### CLI Output

```
╔════════════════════════════════════════════════════════════════╗
║   FROST 3-of-5 門檻簽章 - 完整流程展示                        ║
║   Demo for bitcoin++ Taipei 2025                              ║
╚════════════════════════════════════════════════════════════════╝

🔧 初始化 Transport 抽象層...
   ✓ 使用 SimulatedLoRaTransport
   ✓ 延遲: 500ms per packet
   ✓ 掉包率: 10%
   ✓ 分片大小: 64 bytes

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📡 LoRa 傳輸開始
   類型: Round1Commitment
   從: signer_1 → 到: coordinator
   Payload 大小: 132 bytes
   預計片段數: 3

  📡 Fragment 1/3 (64 bytes)... ✓
  📡 Fragment 2/3 (64 bytes)... ✗ (掉包)
     🔄 重傳 1/3...
  📡 Fragment 2/3 (64 bytes)... ✓
  📡 Fragment 3/3 (4 bytes)... ✓
```

### Dashboard (Live Updates)

- **Phase Indicator**: `ROUND1COMMITMENT█`
- **Progress**: 60% complete with animated bar
- **RSSI**: -75 dBm with 4/5 signal bars
- **Spectrum**: Real-time animated frequency bars
- **Event Log**:
  ```
  [10:30:45] [START] signer_1 → coordinator | Round1Commitment
  [10:30:46] [FRAGMENT] 1/3 | 64 bytes
  [10:30:47] [LOST] Fragment 2 | Retry 1
  [10:30:48] [COMPLETE] 2150ms | 1 retries
  ```

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────┐
│  CLI / User Interface                       │
│  (demo-basic, HTTP server)                  │
└──────────────┬──────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────┐
│  FROST Protocol Layer                       │
│  • Coordinator (orchestration)              │
│  • Signers (key shares + nonces)            │
└──────────────┬──────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────┐
│  Transport Abstraction (trait)              │
│  • SimulatedLoRaTransport ✓                 │
│  • StdoutTransport ✓                        │
│  • RealLoRaTransport (future)               │
└──────────────┬──────────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────────┐
│  HTTP API (Axum + Tokio)                    │
│  GET /status → Dashboard                    │
└─────────────────────────────────────────────┘
```

### Directory Structure

```
frost-threshold-signature/
├── src/
│   ├── lib.rs                  # Library root
│   ├── coordinator.rs          # FROST coordinator
│   ├── signer.rs               # FROST signer
│   ├── api.rs                  # Data structures
│   ├── transport/
│   │   ├── mod.rs              # Transport trait
│   │   └── simulated_lora.rs   # LoRa simulation
│   ├── cli/                    # CLI tools
│   └── bin/
│       └── frost-cli.rs        # Main CLI + HTTP server
├── dashboard.html              # Cyberpunk dashboard
├── demo-basic.bat              # Quick launcher
└── README.md                   # This file
```

---

## 🔧 Technical Stack

- **Language**: Rust 2021 Edition
- **FROST**: `frost-secp256k1` (threshold signatures)
- **Async**: `tokio` (async runtime)
- **Web**: `axum` (HTTP framework)
- **CLI**: `clap` (argument parsing)
- **Serialization**: `serde` + `serde_json`
- **Crypto**: secp256k1 curve (Bitcoin compatible)

---

## 🧪 Testing & Verification

FROST-T includes comprehensive testing tools to ensure demo reliability:

### Quick Test (Recommended)

**Windows:**
```bash
quick-test.bat
```

**Linux/Mac:**
```bash
chmod +x quick-test.sh
./quick-test.sh
```

Checks: Rust environment, compilation, port availability, API endpoints, dashboard

### Full Automated Test

```bash
# Terminal 1: Start server
demo-basic.bat

# Terminal 2: Run tests
python verify_demo.py
```

Performs complete workflow testing with health checks, status monitoring, signing requests, and validation.

### Testing Resources

- 🧪 [TESTING-GUIDE.md](TESTING-GUIDE.md) - Complete testing documentation
- ✅ [VERIFICATION-CHECKLIST.md](VERIFICATION-CHECKLIST.md) - Manual verification checklist
- 🔧 `verify_demo.py` - Python automated test suite
- ⚡ `quick-test.bat/sh` - One-click environment verification

---

## 📚 Documentation

- 📘 [PHASE2-README.md](PHASE2-README.md) - Complete Phase 2 documentation
- 📗 [DEMO-README.md](DEMO-README.md) - Demo usage guide
- 📙 [CLI-README.md](CLI-README.md) - CLI tools reference
- 🧪 [TESTING-GUIDE.md](TESTING-GUIDE.md) - Testing & verification guide

### API Endpoints

#### GET /health
Health check endpoint
```bash
curl http://127.0.0.1:3000/health
```

**Response:**
```json
{
  "status": "ok",
  "service": "frost-threshold-signature",
  "version": "0.1.0"
}
```

#### GET /status
Current LoRa transmission state
```bash
curl http://127.0.0.1:3000/status
```

**Response:**
```json
{
  "current_phase": "Round1Commitment",
  "total_messages": 5,
  "total_bytes": 1234,
  "progress": 0.6,
  "rssi": -75,
  "recent_events": [...],
  "total_retries": 2
}
```

#### POST /sign
Execute threshold signature
```bash
curl -X POST http://127.0.0.1:3000/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "test", "signer_ids": [1, 2, 3]}'
```

**Response:**
```json
{
  "signature": "a1b2c3d4...",
  "verified": true,
  "message": "test",
  "signer_ids": [1, 2, 3]
}
```

For complete API documentation, see [TESTING-GUIDE.md](TESTING-GUIDE.md#api-端點說明).

---

## 🎯 bitcoin++ Taipei 2025

This project was built for **bitcoin++ Taipei 2025** hackathon, demonstrating:

- **Sovereignty**: Distributed key management
- **Privacy**: Indistinguishable threshold signatures
- **Censorship Resistance**: Offline + long-range LoRa communication

### Demo Script

**30 seconds:**
```bash
demo-basic.bat
```
*"Watch FROST signatures over simulated LoRa with packet loss and retry!"*

**2 minutes:**
1. Explain multi-party wallet problem
2. Show live demo (CLI + Dashboard)
3. Highlight: fragmentation, packet loss, RSSI changes

**5 minutes:**
1. Problem background (60s)
2. Architecture + Transport abstraction (90s)
3. Live demo with explanation (120s)
4. Future: Real LoRa hardware integration (30s)

---

## 🔮 Roadmap

### Phase 3: Hardware Integration
- [ ] Real LoRa module (SX1276/SX1278)
- [ ] ESP32/Arduino firmware
- [ ] Field testing

### Phase 4: Alternative Transports
- [ ] QR Code (air-gapped)
- [ ] NFC (near-field)
- [ ] Bluetooth LE

### Phase 5: Production
- [ ] Distributed Key Generation (DKG)
- [ ] HSM integration
- [ ] WebSocket real-time updates
- [ ] Mobile app

---

## 🤝 Contributing

Contributions welcome! Areas of interest:

- 🔬 Cryptography review
- 🛠️ Hardware integration
- 🎨 UI/UX improvements
- 📝 Documentation
- 🐛 Bug reports

```bash
# Fork, clone, create branch
git checkout -b feature/amazing-feature

# Make changes, test
cargo test && cargo clippy

# Commit and push
git commit -m "feat: add amazing feature"
git push origin feature/amazing-feature
```

---

## 📄 License

MIT License - see [LICENSE](LICENSE)

---

## 🙏 Acknowledgments

- **ZcashFoundation**: `frost-core` library
- **bitcoin++ Community**: Inspiration and support
- **Rust Community**: Amazing ecosystem

---

<a name="中文"></a>

## 🎯 什麼是 FROST-T？

**FROST-T** (FROST Terminal) 是 **FROST (Flexible Round-Optimized Schnorr Threshold)** 協議的完整實作，具有兩大創新：

1. **虛擬 LoRa 傳輸**：真實的無線傳輸模擬（延遲、掉包、分片）
2. **Cyberpunk Dashboard**：復古終端機風格的即時視覺化

非常適合展示離線多方錢包和遠距通訊場景。

### ⚡ 快速展示

```bash
# 一個命令看到所有功能！
demo-basic.bat
```

這會啟動：
- ✅ 完整的 3-of-5 FROST 簽章流程
- ✅ 模擬 LoRa 傳輸（500ms 延遲、10% 掉包、64 bytes 分片）
- ✅ HTTP API 服務器（port 3000）
- ✅ 瀏覽器中的 Cyberpunk dashboard

---

## ✨ 功能特性

### 🔐 核心 FROST 實作

- **3-of-5 門檻**：5 個簽署者中任意 3 個可創建有效簽章
- **比特幣相容**：使用 secp256k1 曲線（Taproot/Schnorr 相容）
- **兩輪協議**：高效的 2 輪通訊
- **隱私保護**：門檻簽章與單一金鑰簽章無法區分
- **Nonce 安全**：自動強制 nonce 一次性使用

### 📡 模擬 LoRa 傳輸

```rust
✓ 延遲：         每個封包 500ms
✓ 掉包率：       10% 機率掉包，自動重傳（最多 3 次）
✓ 封包分片：     64 bytes 片段（LoRa SF7 典型值）
✓ 事件追蹤：     即時記錄供 Dashboard 查詢
✓ 共享狀態：     執行緒安全的 Arc<Mutex>
```

### 🎨 Cyberpunk Dashboard

**功能：**
- 📈 **進度條**：即時傳輸進度與 shimmer 效果
- 📶 **RSSI 訊號計**：訊號強度（-120 到 -30 dBm）
- 🌊 **頻譜分析器**：50 個動態柱狀圖
- 📋 **事件日誌**：最近 20 條事件與時間戳
- 📊 **統計資訊**：訊息數、位元組數、重傳次數、成功率

**主題：**
- 黑底 + 螢光綠（#0f0）
- CRT 掃描線效果
- 霓虹發光動畫
- 閃爍游標 █

---

## 🚀 快速開始

### 前置需求

```bash
# 安裝 Rust（如果尚未安裝）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows 用戶
# 從這裡下載：https://rustup.rs/
```

### 安裝

```bash
git clone https://github.com/benson-code/frost-threshold-signature.git
cd frost-threshold-signature
cargo build --release
```

### 執行 Demo

**Windows（一鍵啟動）：**
```bash
demo-basic.bat
```

**手動（跨平台）：**
```bash
# 終端 1：開啟 dashboard
start dashboard.html        # Windows
open dashboard.html         # macOS
xdg-open dashboard.html     # Linux

# 終端 2：執行 demo
cargo run --bin frost-cli -- demo-basic
```

**自訂參數：**
```bash
# 自訂訊息
cargo run --bin frost-cli -- demo-basic -m "Hello bitcoin++"

# 不同的簽署者（例如：2, 4, 5）
cargo run --bin frost-cli -- demo-basic --signers 2,4,5

# 顯示完整的 hex payload
cargo run --bin frost-cli -- demo-basic --full-payload
```

---

## 📞 聯絡方式

- **GitHub**: [@benson-code](https://github.com/benson-code)
- **Project**: [frost-threshold-signature](https://github.com/benson-code/frost-threshold-signature)

---

<div align="center">

**Built with ❤️ for bitcoin++ Taipei 2025**

Rust • FROST • Axum • Tokio • secp256k1

[⬆ Back to Top](#frost-t-)

</div>
