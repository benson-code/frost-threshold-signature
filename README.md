# FROST Threshold Signature Service | FROST 門檻簽章服務

<div align="center">

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

**[English](#english)** | **[中文](#中文)**

</div>

---

<a name="english"></a>
# 🔐 FROST Threshold Signature Service

> Enterprise-grade Bitcoin-compatible Schnorr threshold signature service using the FROST protocol

## 📋 Overview

This project implements a **3-of-5 threshold signature service** using the **FROST (Flexible Round-Optimized Schnorr Threshold)** protocol. It supports a configuration where any 3 out of 5 signers can collaboratively generate a valid Schnorr signature.

### Core Features

- ✅ **Bitcoin Compatible**: Uses secp256k1 curve, fully compatible with Bitcoin Taproot
- ✅ **Efficient Protocol**: Only 2 communication rounds (vs. 6-9 rounds in traditional TSS)
- ✅ **Privacy-Preserving**: Final signature is indistinguishable from single-signer signatures
- ✅ **Enterprise Architecture**: Modular design supporting horizontal scaling
- ✅ **Concurrency-Safe**: Supports multiple concurrent signing sessions
- ✅ **Nonce Security**: Strict one-time nonce usage mechanism

## 🏗️ Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                      HTTP API Layer                         │
│              (Axum + Tokio - RESTful)                       │
└───────────────────┬─────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        │                       │
┌───────▼────────┐      ┌──────▼───────────────────────┐
│  Coordinator   │      │    Signer Actors (1-5)       │
│                │      │                              │
│ • Orchestrates │◄────►│ • Holds KeyPackage          │
│ • Aggregates   │      │ • Manages Nonce State       │
│ • Verifies     │      │ • Round 1: commit()         │
│ • No Keys!     │      │ • Round 2: sign()           │
└────────────────┘      └─────────────────────────────┘
```

### Module Structure

```
src/
├── main.rs           # Main entry point - Initialize & start service
├── api.rs            # API contracts - Request/Response structures
├── coordinator.rs    # Coordinator logic - Orchestrate signing flow
├── signer.rs         # Signer actor - Nonce state management
└── handlers.rs       # HTTP handlers - Axum routes

examples/
├── level1_mvp.rs     # Level 1 monolithic demo
└── demo_client.rs    # HTTP API client demo
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (install via `rustup`)
- (Optional) `curl` or `httpie` for manual API testing

### Installation & Running

```bash
# 1. Clone the repository
git clone https://github.com/benson-code/frost-threshold-signature.git
cd frost-threshold-signature

# 2. Build the project
cargo build --release

# 3. Run the service
cargo run --release
```

The service will start at `http://127.0.0.1:3000`.

### Run Demo Client

In another terminal:

```bash
cargo run --example demo_client
```

### Run Level 1 MVP

```bash
cargo run --example level1_mvp
```

## 📡 API Documentation

### 1. Health Check

```bash
GET /health
```

**Response Example:**
```json
{
  "status": "ok",
  "signers_count": 5,
  "active_sessions": 0
}
```

### 2. Get Group Public Key

```bash
GET /pubkey
```

**Response Example:**
```json
{
  "group_public_key": "02a1b2c3d4..."
}
```

### 3. Round 1: Generate Commitment

```bash
POST /signer/:signer_id/round1
Content-Type: application/json

{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "deadbeef..."
}
```

### 4. Round 2: Generate Signature Share

```bash
POST /signer/:signer_id/round2
Content-Type: application/json

{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "signing_package": {
    "commitments": [...],
    "message": "deadbeef..."
  }
}
```

### 5. Complete Signing Flow (High-level API)

```bash
POST /sign
Content-Type: application/json

{
  "signer_ids": [1, 2, 3],
  "message": "5472616e73666572..."
}
```

## 🔐 Security Considerations

### Implemented Security Measures

1. **One-Time Nonce Usage**
   - Each session's nonce is immediately destroyed after use
   - Prevents private key leakage from nonce reuse

2. **Concurrency Safety**
   - Uses `DashMap` for lock-free concurrent access
   - Supports multiple independent signing sessions

3. **Unprivileged Coordinator**
   - Coordinator never holds private key shares
   - Coordinator never touches secret nonces
   - Even if compromised, cannot forge signatures

4. **Session ID Isolation**
   - Each signing request uses a unique UUID
   - Prevents replay and confusion attacks

### Production Recommendations

⚠️ **Current implementation uses Trusted Dealer method for demonstration only!**

For production, implement:

- [ ] **Distributed Key Generation (DKG)** - Eliminate single point of trust
- [ ] **HSM Integration** - Hardware protection for key shares
- [ ] **TLS/mTLS** - Encrypted communication
- [ ] **Session Expiration** - Prevent resource leaks
- [ ] **Rate Limiting** - Prevent DoS attacks
- [ ] **Audit Logging** - Record all signing operations
- [ ] **Key Rotation** - Periodic key updates

## 📊 FROST Protocol Flow

```
Setup (One-time):
  Trusted Dealer generates 5 key shares
  ↓
  Distribute to 5 signers

Round 1 (Commitment):
  Signer 1-3: Each generates random nonce
  ↓
  Signer 1-3: Compute and submit public commitments
  ↓
  Coordinator: Collect all commitments

Round 2 (Signing):
  Coordinator: Create SigningPackage
  ↓
  Signer 1-3: Generate share using (key_share + nonce + package)
  ↓
  Coordinator: Aggregate shares → Final signature

Verification:
  Anyone: Verify signature using group public key ✓
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --lib coordinator

# Run benchmarks (future implementation)
cargo bench
```

## 📈 Performance Metrics

| Operation | Latency (Local) | Notes |
|-----------|-----------------|-------|
| Round 1 (commit) | ~1ms | Generate nonce and commitment |
| Round 2 (sign) | ~2ms | Generate signature share |
| Aggregate | ~1ms | Aggregate 3 shares |
| Verify | ~2ms | Verify Schnorr signature |
| **Total (Complete Flow)** | **~10ms** | End-to-end (including network) |

*Test Environment: AMD Ryzen 7 / 16GB RAM / Localhost*

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📚 References

- [FROST Paper](https://eprint.iacr.org/2020/852.pdf) - Komlo & Goldberg, 2020
- [frost-secp256k1 Documentation](https://docs.rs/frost-secp256k1/)
- [Bitcoin Taproot BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)
- [Axum Web Framework](https://docs.rs/axum/)

---

<a name="中文"></a>
# 🔐 FROST 門檻簽章服務

> 企業級的比特幣相容 Schnorr 門檻簽章服務，使用 FROST 協議實作

## 📋 專案簡介

本專案使用 **FROST (Flexible Round-Optimized Schnorr Threshold)** 協議實作了一個 **3-of-5 門檻簽章服務**。支援 5 個簽署者中任意 3 個即可生成有效的 Schnorr 簽章。

### 核心特性

- ✅ **比特幣相容**: 使用 secp256k1 曲線，與 Bitcoin Taproot 完全相容
- ✅ **高效協議**: 僅需 2 輪通訊（相比傳統 TSS 需要 6-9 輪）
- ✅ **隱私保護**: 最終簽章與單一簽署者的簽章無法區分
- ✅ **企業架構**: 模組化設計，支援水平擴展
- ✅ **並發安全**: 支援多個並發簽章會話
- ✅ **Nonce 安全**: 嚴格的 Nonce 一次性使用機制

## 🏗️ 架構設計

### 系統元件

```
┌─────────────────────────────────────────────────────────────┐
│                      HTTP API 層                            │
│              (Axum + Tokio - RESTful)                       │
└───────────────────┬─────────────────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        │                       │
┌───────▼────────┐      ┌──────▼───────────────────────┐
│   協調者       │      │    簽署者 Actors (1-5)       │
│                │      │                              │
│ • 編排流程      │◄────►│ • 持有金鑰分片               │
│ • 聚合簽章      │      │ • 管理 Nonce 狀態           │
│ • 驗證簽章      │      │ • Round 1: commit()         │
│ • 無私鑰！      │      │ • Round 2: sign()           │
└────────────────┘      └─────────────────────────────┘
```

### 模組結構

```
src/
├── main.rs           # 主程式 - 初始化與啟動服務
├── api.rs            # API 合約 - Request/Response 結構
├── coordinator.rs    # 協調者邏輯 - 編排簽章流程
├── signer.rs         # 簽署者 Actor - Nonce 狀態管理
└── handlers.rs       # HTTP 處理器 - Axum 路由

examples/
├── level1_mvp.rs     # Level 1 單體式示範
└── demo_client.rs    # HTTP API 客戶端示範
```

## 🚀 快速開始

### 前置需求

- Rust 1.75+ (使用 `rustup` 安裝)
- (可選) `curl` 或 `httpie` 用於手動測試 API

### 安裝與運行

```bash
# 1. 克隆專案
git clone https://github.com/benson-code/frost-threshold-signature.git
cd frost-threshold-signature

# 2. 建置專案
cargo build --release

# 3. 運行服務
cargo run --release
```

服務將在 `http://127.0.0.1:3000` 啟動。

### 運行示範客戶端

在另一個終端運行：

```bash
cargo run --example demo_client
```

### 運行 Level 1 MVP

```bash
cargo run --example level1_mvp
```

## 📡 API 文檔

### 1. 健康檢查

```bash
GET /health
```

**回應範例:**
```json
{
  "status": "ok",
  "signers_count": 5,
  "active_sessions": 0
}
```

### 2. 獲取群組公鑰

```bash
GET /pubkey
```

**回應範例:**
```json
{
  "group_public_key": "02a1b2c3d4..."
}
```

### 3. Round 1: 生成承諾

```bash
POST /signer/:signer_id/round1
Content-Type: application/json

{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "deadbeef..."
}
```

### 4. Round 2: 生成簽章分片

```bash
POST /signer/:signer_id/round2
Content-Type: application/json

{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "signing_package": {
    "commitments": [...],
    "message": "deadbeef..."
  }
}
```

### 5. 完整簽章流程（高階 API）

```bash
POST /sign
Content-Type: application/json

{
  "signer_ids": [1, 2, 3],
  "message": "5472616e73666572..."
}
```

## 🔐 安全性考量

### 已實作的安全措施

1. **Nonce 一次性使用**
   - 每個 Session 的 Nonce 在使用後立即銷毀
   - 防止 Nonce 重用導致的私鑰洩漏

2. **並發安全**
   - 使用 `DashMap` 提供無鎖並發存取
   - 支援多個獨立的簽章會話

3. **協調者無特權**
   - 協調者永不持有私鑰分片
   - 協調者永不接觸秘密 Nonces
   - 即使協調者被攻破也無法偽造簽章

4. **Session ID 隔離**
   - 每個簽章請求使用唯一的 UUID
   - 防止重放攻擊和混淆攻擊

### 生產環境建議

⚠️ **當前實作使用 Trusted Dealer 方法，僅供演示！**

生產環境應實作：

- [ ] **分散式金鑰生成 (DKG)** - 消除單點信任
- [ ] **HSM 整合** - 硬體保護金鑰分片
- [ ] **TLS/mTLS** - 加密通訊
- [ ] **Session 過期機制** - 防止資源洩漏
- [ ] **速率限制** - 防止 DoS 攻擊
- [ ] **審計日誌** - 記錄所有簽章操作
- [ ] **金鑰輪換** - 定期更換金鑰

## 📊 FROST 協議流程

```
Setup (一次性):
  Trusted Dealer 生成 5 個金鑰分片
  ↓
  分發給 5 個簽署者

Round 1 (Commitment):
  Signer 1-3: 各自生成隨機 nonce
  ↓
  Signer 1-3: 計算並提交公開承諾
  ↓
  Coordinator: 收集所有承諾

Round 2 (Signing):
  Coordinator: 建立 SigningPackage
  ↓
  Signer 1-3: 使用 (key_share + nonce + package) 生成分片
  ↓
  Coordinator: 聚合分片 → 最終簽章

Verification:
  任何人: 使用群組公鑰驗證簽章 ✓
```

## 🧪 測試

```bash
# 運行所有測試
cargo test

# 運行特定模組的測試
cargo test --lib coordinator

# 運行基準測試（未來實作）
cargo bench
```

## 📈 效能指標

| 操作 | 延遲 (本地) | 備註 |
|------|-------------|------|
| Round 1 (commit) | ~1ms | 生成 nonce 和承諾 |
| Round 2 (sign) | ~2ms | 生成簽章分片 |
| Aggregate | ~1ms | 聚合 3 個分片 |
| Verify | ~2ms | 驗證 Schnorr 簽章 |
| **Total (完整流程)** | **~10ms** | 端到端（含網路開銷） |

*測試環境: AMD Ryzen 7 / 16GB RAM / Localhost*

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！

## 📄 授權

本專案採用 MIT 授權 - 詳見 [LICENSE](LICENSE) 文件

## 📚 參考資料

- [FROST 論文](https://eprint.iacr.org/2020/852.pdf) - Komlo & Goldberg, 2020
- [frost-secp256k1 文檔](https://docs.rs/frost-secp256k1/)
- [比特幣 Taproot BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki)
- [Axum Web 框架](https://docs.rs/axum/)

---

<div align="center">

**Built with ❤️ by [benson-code](https://github.com/benson-code)**

**技術棧**: Rust • FROST • Axum • Tokio • secp256k1

</div>
