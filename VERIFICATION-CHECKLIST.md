# 🔍 FROST-T Demo 手動驗證清單

> **用途**: 在 bitcoin++ Taipei 2025 展示前，確保所有功能正常運作

---

## 📋 展示前檢查清單

### ✅ 環境準備

- [ ] **Rust 編譯環境**
  ```bash
  cargo --version
  # 應顯示 cargo 1.70 或更高版本
  ```

- [ ] **編譯成功**
  ```bash
  cargo build --release --bin frost-cli
  # 應無錯誤訊息
  ```

- [ ] **Python 環境**（如使用自動化測試）
  ```bash
  python --version
  pip install requests
  # 或使用 python3
  ```

- [ ] **Port 3000 可用**
  ```bash
  netstat -ano | findstr :3000
  # 應無任何輸出（表示 port 未被佔用）
  ```

- [ ] **瀏覽器準備**
  - 使用 Chrome, Firefox, 或 Edge
  - 確認可開啟本地 HTML 檔案

---

## 🚀 快速測試流程

### 1️⃣ 一鍵啟動測試

```bash
demo-basic.bat
```

**預期結果:**
- [ ] Dashboard 在瀏覽器中自動開啟
- [ ] CLI 顯示完整的 FROST 簽章流程
- [ ] HTTP Server 啟動訊息顯示 `http://127.0.0.1:3000`
- [ ] 無錯誤訊息或 panic

---

## 📺 CLI 輸出驗證

### 2️⃣ 檢查 CLI 顯示

執行 `demo-basic.bat` 後，檢查以下內容：

#### ✓ Unicode 邊框正常顯示

- [ ] 看到完整的 `╔════╗` 邊框（不是亂碼）
- [ ] 中文字正常顯示（不是 `???` 或方塊）
- [ ] Emoji 正常顯示（✓, ✗, 📡, 🔐 等）

#### ✓ 階段標題清楚

- [ ] 階段 1: 金鑰生成（Setup - Key Generation）
- [ ] 階段 2: Round 1 承諾（Round 1 - Commitments）
- [ ] 階段 3: 建立簽章套件（Create Signing Package）
- [ ] 階段 4: Round 2 簽章分片（Round 2 - Signature Shares）
- [ ] 階段 5: 聚合簽章（Aggregate Signature）
- [ ] 階段 6: 驗證簽章（Verify Signature）

#### ✓ LoRa 傳輸細節

每次訊息傳輸時，應看到：

- [ ] **傳輸開始訊息**
  ```
  📡 LoRa 傳輸開始
     類型: Round1Commitment
     從: signer_1 → 到: coordinator
     Payload 大小: XXX bytes
     預計片段數: X
  ```

- [ ] **片段傳輸進度**
  ```
  📡 Fragment 1/3 (64 bytes)... ✓
  📡 Fragment 2/3 (64 bytes)... ✗ (掉包)
     🔄 重傳 1/3...
  📡 Fragment 2/3 (64 bytes)... ✓
  ```

- [ ] **至少看到 1 次掉包與重傳**（10% 機率）
  - 如果沒看到，再執行一次 demo

#### ✓ 最終結果

- [ ] 顯示群組公鑰 (hex)
- [ ] 顯示最終簽章 (hex)
- [ ] 顯示 `✓ 簽章驗證通過！`
- [ ] 顯示傳輸統計（總訊息數、總位元組數、訊息類型分布）
- [ ] Server 持續運行訊息: `🌐 HTTP Server 仍在運行...`

---

## 🎨 Dashboard 驗證

### 3️⃣ 檢查 Dashboard 視覺效果

開啟 `dashboard.html`，檢查以下項目：

#### ✓ 基本連線狀態

- [ ] 頁面標題: `FROST-T DASHBOARD`
- [ ] 右上角連線指示燈為 **綠色** (● CONNECTED)
- [ ] 如果是紅色，表示無法連線到 `http://127.0.0.1:3000`

#### ✓ Phase Indicator (頂部)

- [ ] 顯示當前階段名稱（如 `ROUND1COMMITMENT`）
- [ ] 後面有閃爍的游標 `█`

#### ✓ Transmission Progress (左上)

- [ ] 顯示進度條
- [ ] 進度條有 **shimmer 動畫效果**（閃亮滑動）
- [ ] 百分比數字即時更新

#### ✓ Signal Strength RSSI (右上)

- [ ] 顯示 5 格訊號強度圖示
- [ ] 根據 RSSI 數值，格數會變化
- [ ] dBm 數值在 -120 到 -30 之間

#### ✓ Spectrum Analyzer (中間)

- [ ] 顯示 50 個頻譜柱狀圖
- [ ] 柱狀圖有 **動態上下浮動動畫**
- [ ] 綠色霓虹發光效果

#### ✓ Statistics Panel (左下)

- [ ] 顯示統計資訊：
  - Total Messages: XX
  - Total Bytes: XXXX
  - Retries: X
  - Success Rate: XX%
- [ ] 數字即時更新

#### ✓ Event Log (右下)

- [ ] 顯示最近的事件
- [ ] 格式: `[HH:MM:SS] [TYPE] description`
- [ ] 顏色區分：
  - `[START]` - 綠色
  - `[FRAGMENT]` - 青色
  - `[LOST]` - 紅色（重要！）
  - `[COMPLETE]` - 綠色
- [ ] 自動滾動顯示新事件

#### ✓ 整體風格

- [ ] **黑色背景 + 螢光綠文字** (#0f0)
- [ ] CRT 掃描線效果（橫條紋）
- [ ] 文字有霓虹發光效果
- [ ] 復古終端機風格

---

## 🔬 API 端點測試

### 4️⃣ 手動測試 HTTP API

#### GET /health

```bash
curl http://127.0.0.1:3000/health
```

**預期輸出:**
```json
{
  "status": "ok",
  "service": "frost-threshold-signature",
  "version": "0.1.0"
}
```

- [ ] HTTP 狀態碼 200
- [ ] JSON 格式正確

#### GET /status

```bash
curl http://127.0.0.1:3000/status
```

**預期輸出:**
```json
{
  "current_phase": "Complete",
  "total_messages": 10,
  "total_bytes": 1234,
  "progress": 1.0,
  "rssi": -75,
  "recent_events": [...],
  "by_type": {...},
  "total_retries": 2
}
```

- [ ] HTTP 狀態碼 200
- [ ] 包含所有必要欄位
- [ ] `progress` 在 0.0 到 1.0 之間
- [ ] `total_retries` > 0（表示有掉包重傳）

#### POST /sign

```bash
curl -X POST http://127.0.0.1:3000/sign \
  -H "Content-Type: application/json" \
  -d '{"message": "test", "signer_ids": [1, 2, 3]}'
```

**預期輸出:**
```json
{
  "signature": "a1b2c3d4...",
  "verified": true,
  "message": "test",
  "signer_ids": [1, 2, 3]
}
```

- [ ] HTTP 狀態碼 200
- [ ] `verified` 為 `true`
- [ ] `signature` 是有效的 hex 字串（不含 "Error:"）

---

## 🤖 自動化測試

### 5️⃣ 執行 Python 驗證腳本

```bash
python verify_demo.py
```

**檢查項目:**

- [ ] 步驟 1: 健康檢查通過 ✓
- [ ] 步驟 2: 狀態監控成功
- [ ] 步驟 3: 簽章請求成功
- [ ] 步驟 4: 監控到 progress 從 0.0 → 1.0
- [ ] 步驟 5: 簽章回應格式正確
- [ ] 步驟 6: 事件日誌正常
- [ ] 最終顯示: `✓✓✓ 所有驗證項目通過！ ✓✓✓`

---

## 🎯 展示演練

### 6️⃣ 完整 Demo 演練

**30 秒快速展示:**

1. [ ] 執行 `demo-basic.bat`
2. [ ] 指出 **Dashboard 視覺化效果**
3. [ ] 指出 **CLI 中的掉包重傳**
4. [ ] 說明 "這模擬了真實的 LoRa 傳輸環境"

**2 分鐘完整展示:**

1. [ ] 開場說明場景（30秒）
   - "離線多方錢包需要遠距通訊"
   - "LoRa: 低功耗、長距離、低頻寬"

2. [ ] 執行 demo（60秒）
   - 同時展示 CLI + Dashboard
   - 指出關鍵時刻：
     - 封包分片
     - 掉包重傳
     - RSSI 變化
     - 最終簽章成功

3. [ ] 技術說明（30秒）
   - Transport 抽象層設計
   - 未來可接真實 LoRa 模組
   - HTTP API 提供監控

**5 分鐘深度展示:**

參考 `PHASE2-README.md` 的詳細腳本

---

## 🐛 常見問題排查

### ❌ Dashboard 無法連線

**症狀**: Dashboard 右上角顯示紅色 `● DISCONNECTED`

**解決方法:**
1. 確認 HTTP Server 正在運行
   ```bash
   netstat -ano | findstr :3000
   ```
2. 檢查 browser console (F12) 是否有 CORS 錯誤
3. 重新執行 `demo-basic.bat`

### ❌ CLI 顯示亂碼

**症狀**: 邊框顯示為 `????` 或方塊

**解決方法:**
1. Windows: 執行 `chcp 65001` 切換到 UTF-8
2. 確認終端機字體支援 Unicode（如 Consolas, Cascadia Code）

### ❌ 編譯錯誤

**症狀**: `cargo build` 失敗

**解決方法:**
1. 檢查 Rust 版本: `rustup update`
2. 清除快取: `cargo clean`
3. 重新編譯: `cargo build --release`

### ❌ Port 3000 被佔用

**症狀**: 啟動失敗，顯示 "Address already in use"

**解決方法:**
1. 找到佔用的 process:
   ```bash
   netstat -ano | findstr :3000
   ```
2. 關閉該 process，或修改 `frost-cli.rs` 中的 port

### ❌ 從未看到掉包重傳

**症狀**: 多次執行 demo，`total_retries` 始終為 0

**解決方法:**
1. 這是正常的機率現象（10% 掉包率）
2. 多執行幾次 demo
3. 如需確保看到，可暫時調整 `simulated_lora.rs` 中的 `packet_loss_rate` 為 0.3（30%）

---

## ✅ 最終確認

### 準備展示前最後檢查:

- [ ] CLI 輸出完整無錯誤
- [ ] Dashboard 視覺效果正常
- [ ] 至少看過 1 次掉包重傳
- [ ] 自動化測試全部通過
- [ ] 熟悉展示腳本
- [ ] 準備好回答技術問題

### 備用方案:

- [ ] 準備好截圖或錄影（以防 live demo 出問題）
- [ ] 準備好 PHASE2-README.md 和 README.md（展示文件）
- [ ] 測試過網路環境（如果需要現場展示）

---

## 🎉 展示建議

### 開場話術範例:

> "這是 FROST-T，一個 Bitcoin 相容的門檻簽章系統。我們模擬了 LoRa 無線傳輸環境，包括延遲、掉包、和重傳。這裡你可以看到，3 個簽署者在低頻寬的環境下，合作產生一個 Schnorr 簽章。"

### 技術亮點:

1. **FROST 協議** - 2 輪高效門檻簽章
2. **Transport 抽象** - 易於整合真實硬體
3. **即時視覺化** - Cyberpunk 風格 Dashboard
4. **真實模擬** - 延遲、掉包、分片

### 結尾話術範例:

> "接下來的 Phase 3，我們計畫整合真實的 LoRa 模組，也會加入 QR Code 和 NFC 等多種離線傳輸方式。這個系統可以用於離線多方錢包、遠距簽章授權等場景。"

---

## 📞 緊急聯絡

如果展示當天遇到問題：

1. **立即重啟**: `demo-basic.bat`
2. **檢查編譯**: `cargo check --bin frost-cli`
3. **清除狀態**: 重新啟動 terminal
4. **最後手段**: 展示預先錄製的影片

---

**祝展示成功！bitcoin++ Taipei 2025 加油！🚀**
