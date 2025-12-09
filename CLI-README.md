# FROST CLI 工具 - 快速參考

## 🎯 專案目標

這是為黑客松準備的 FROST 3-of-5 門檻簽章 CLI 工具，支援在多個終端視窗模擬分散式簽章流程。

## 📦 專案結構

```
frost-threshold-signature/
├── src/
│   ├── lib.rs              # 核心函式庫入口
│   ├── main.rs             # HTTP API 服務（保留）
│   ├── coordinator.rs      # 協調者邏輯
│   ├── signer.rs          # 簽署者邏輯
│   ├── api.rs             # API 合約
│   ├── handlers.rs        # HTTP 處理器
│   └── cli/               # ✨ 新增：CLI 相關模組
│       ├── mod.rs
│       ├── commands.rs    # 命令列參數定義
│       ├── file_store.rs  # 檔案輸入輸出
│       └── nonce_store.rs # Nonce 持久化（Demo only）
├── src/bin/
│   └── frost-cli.rs       # ✨ 新增：CLI 主程式
├── examples/
│   ├── level1_mvp.rs      # Level 1: 單體示範
│   └── demo_client.rs     # HTTP 客戶端
├── CLI-DEMO.md            # ✨ 新增：詳細使用指南
├── CLI-README.md          # ✨ 新增：快速參考（本檔案）
├── demo-cli.bat           # ✨ 新增：Windows Demo 腳本
└── build-cli.bat          # ✨ 新增：編譯腳本
```

## 🚀 快速開始

### 1. 編譯

```bash
# Windows
build-cli.bat

# Linux/Mac
cargo build --release --bin frost-cli
```

### 2. 查看說明

```bash
cargo run --bin frost-cli -- --help
```

### 3. 完整 Demo

詳見 [`CLI-DEMO.md`](./CLI-DEMO.md)

## 📋 命令速查表

| 命令 | 角色 | 功能 | 範例 |
|------|------|------|------|
| `keygen` | Dealer | 生成金鑰分片 | `frost-cli keygen --output-dir frost-data` |
| `round1` | Signer | 生成承諾 | `frost-cli round1 --share-file share_1.json --message-file msg.txt` |
| `create-package` | Coordinator | 建立簽章套件 | `frost-cli create-package --commitment-files c1.json c2.json c3.json` |
| `round2` | Signer | 生成簽章分片 | `frost-cli round2 --share-file share_1.json --package-file pkg.json --session-id ID` |
| `aggregate` | Coordinator | 聚合簽章 | `frost-cli aggregate --package-file pkg.json --share-files s1.json s2.json s3.json` |
| `verify` | Anyone | 驗證簽章 | `frost-cli verify --signature-file sig.json --message-file msg.txt` |

## 🎤 黑客松 Demo 腳本

### 方案 1：預錄 + 現場執行

**預錄（Demo 前）**：
```bash
# 金鑰生成
cargo run --bin frost-cli -- keygen --output-dir frost-data

# 準備訊息
echo "Transfer 1.5 BTC to bc1q..." > message.txt

# Round 1（3個簽署者）
cargo run --bin frost-cli -- round1 --share-file frost-data/share_1.json --message-file message.txt -o c1.json
cargo run --bin frost-cli -- round1 --share-file frost-data/share_2.json --message-file message.txt -o c2.json
cargo run --bin frost-cli -- round1 --share-file frost-data/share_3.json --message-file message.txt -o c3.json
```

**現場展示（5 分鐘）**：
```bash
# 1. 建立簽章套件
cargo run --bin frost-cli -- create-package \
  --commitment-files c1.json c2.json c3.json \
  --message-file message.txt

# 2. Round 2（展示一個簽署者即可）
cargo run --bin frost-cli -- round2 \
  --share-file frost-data/share_1.json \
  --package-file signing_package.json \
  --session-id [從 c1.json 讀取] \
  -o s1.json

# （其他簽署者已預錄）

# 3. 聚合簽章
cargo run --bin frost-cli -- aggregate \
  --package-file signing_package.json \
  --share-files s1.json s2.json s3.json \
  --pubkey-file frost-data/pubkey.json

# 4. 驗證簽章
cargo run --bin frost-cli -- verify \
  --signature-file signature.json \
  --message-file message.txt \
  --pubkey-file frost-data/pubkey.json
```

### 方案 2：多終端視窗演示

**準備 4 個終端視窗，分別扮演**：
- 視窗 1：Dealer + Coordinator
- 視窗 2：Signer 1
- 視窗 3：Signer 2
- 視窗 4：Signer 3

**演示流程**：
1. 在視窗 1 執行 `keygen`
2. 同時在視窗 2、3、4 執行 `round1`（強調「並行」）
3. 在視窗 1 執行 `create-package`
4. 同時在視窗 2、3、4 執行 `round2`
5. 在視窗 1 執行 `aggregate` 和 `verify`

## 🎯 核心特性（講解重點）

### 1. 僅需 2 輪通訊

> "相比傳統 TSS 需要 6-9 輪，FROST 只需 2 輪！"

```
Round 1: 每個簽署者生成 Nonce 承諾
Round 2: 每個簽署者生成簽章分片
Aggregate: 協調者聚合最終簽章
```

### 2. 協調者無特權

> "協調者永不持有私鑰，即使被攻破也無法偽造簽章！"

```rust
// coordinator.rs
pub struct Coordinator {
    pubkey_package: PublicKeyPackage,  // ✅ 只有公鑰
    threshold: u16,
    // ❌ 沒有私鑰！
}
```

### 3. 隱私保護

> "最終簽章與單一簽署者的 Schnorr 簽章完全相同，無法區分！"

### 4. Bitcoin 相容

> "使用 secp256k1 曲線，與 Bitcoin Taproot 完全相容！"

## ⚠️ 注意事項

### 僅供 Demo

此 CLI 工具將秘密 Nonce 儲存到磁碟（`.frost-nonces/`），
**這在生產環境中是不允許的！**

### 生產環境需要

- [ ] 使用 DKG 取代 Trusted Dealer
- [ ] Nonce 僅存在記憶體，使用後立即銷毀
- [ ] 整合 HSM 保護金鑰分片
- [ ] TLS/mTLS 加密通訊
- [ ] Session 過期機制
- [ ] 審計日誌

## 📚 進階學習

1. **Level 1 (單體示範)**：
   ```bash
   cargo run --example level1_mvp
   ```
   → 理解 FROST 協議的完整流程

2. **Level 2 (HTTP API)**：
   ```bash
   cargo run --release
   cargo run --example demo_client
   ```
   → 理解如何在分散式環境中運作

3. **Level 3 (CLI 工具)**：
   ```bash
   cargo run --bin frost-cli -- keygen
   ```
   → 適合黑客松 Demo 和教學

## 🔗 相關資源

- 主 README：[README.md](./README.md)
- 詳細使用指南：[CLI-DEMO.md](./CLI-DEMO.md)
- 部署指南：[DEPLOY.md](./DEPLOY.md)
- QA 報告：[QA_REPORT.md](./QA_REPORT.md)
- FROST 論文：https://eprint.iacr.org/2020/852.pdf

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！

## 📄 授權

MIT License

---

**祝黑客松順利！🚀**
