# 🚀 FROST-T Hackathon 快速展示卡

> 5 分鐘快速展示指南 - 現場必備

---

## 📋 展示前檢查（2 分鐘）

```bash
# 1. 測試環境
./test-demo-setup.sh

# 2. 記錄 Mac mini IP
ifconfig | grep "inet " | grep -v 127.0.0.1
# 輸出示例：inet 192.168.68.51
```

**✅ 確認：**
- [ ] 兩台設備同一 WiFi
- [ ] SSH 可連線
- [ ] Port 3000/8000 空閒
- [ ] 專案已編譯

---

## ⚡ 快速啟動（1 分鐘）

### Mac mini
```bash
./demo-hackathon-all.sh
```

### Surface Go 4

**終端（CLI 展示）：**
```bash
# Linux/macOS
./demo-hackathon-client.sh 192.168.68.51

# Windows
demo-hackathon-client.bat 192.168.68.51
```

**瀏覽器（Dashboard）：**
```
http://192.168.68.51:8000/dashboard.html?api=http://192.168.68.51:3000
```

---

## 🎤 展示台詞（5 分鐘）

### 1. 開場（30 秒）
> "FROST-T 是一個基於 FROST 協議的 3-of-5 門檻簽章系統，
> 專為比特幣多重簽章場景設計。
> 今天我將展示分散式密鑰管理和 LoRa 無線傳輸模擬。"

**投影：** 架構圖

### 2. 技術特點（1 分鐘）
> "核心特性包括：
> - ✅ 任意 3 個簽署者即可完成簽章（無需全部 5 個）
> - ✅ 使用比特幣 secp256k1 曲線，與 Taproot 相容
> - ✅ 模擬 LoRa 低頻寬傳輸（500ms 延遲、10% 掉包）
> - ✅ 即時視覺化監控"

**投影：** Dashboard 首頁

### 3. 現場 Demo（2 分鐘）
> "現在我將執行一次完整的門檻簽章流程..."

**執行：**
```bash
cargo run --bin frost-cli -- demo-basic -m "Bitcoin++ Taipei 2025!"
```

**說明：**
- Round 1: 生成 nonce 承諾（5 個簽署者）
- Round 2: 生成簽章分片（使用簽署者 1, 2, 3）
- 聚合：組合簽章分片
- 驗證：驗證最終簽章

**投影：** 切換到 Dashboard 觀看即時傳輸

### 4. Dashboard 展示（1 分鐘）
> "Dashboard 展示了每一筆 LoRa 傳輸的詳細資訊：
> - 傳輸進度和 RSSI 訊號強度
> - 封包掉失和重試機制
> - 頻譜分析器即時視覺化"

**操作：** 滾動事件日誌，指出關鍵事件

### 5. 總結 + Q&A（30 秒）
> "FROST-T 展示了如何結合門檻密碼學和低頻寬通訊，
> 為比特幣冷錢包和多重簽章應用提供安全、實用的解決方案。"

**準備回答：**
- 為什麼選擇 3-of-5？
- LoRa 的實際應用？
- 與傳統多簽的區別？

---

## 🎨 展示技巧

### 雙螢幕設定
- **主螢幕（投影）：** Dashboard F11 全螢幕
- **副螢幕（自己）：** Terminal 操作

### 快捷命令（預先準備）

```bash
# 基本展示
cargo run --bin frost-cli -- demo-basic -m "Bitcoin++"

# 展示不同簽署者組合（證明門檻特性）
cargo run --bin frost-cli -- demo-basic --signers 2,4,5

# 展示技術細節
cargo run --bin frost-cli -- demo-basic --full-payload
```

### 瀏覽器準備
- 開啟 Dashboard 並 F11 全螢幕
- 新分頁準備架構圖（如有）

---

## 🐛 緊急故障處理

### Dashboard 無法連線
```bash
# 快速測試
curl http://192.168.68.51:3000/health

# 重啟服務
pkill -f frost-threshold
./demo-hackathon-all.sh
```

### SSH 連線失敗
```bash
# 改用本地展示（Mac mini）
# 終端 1: 服務器
./demo-hackathon-all.sh

# 終端 2: CLI
cargo run --bin frost-cli -- demo-basic

# 瀏覽器
open http://localhost:8000/dashboard.html
```

### 臨時網路問題
**備案：** 使用錄製好的展示影片

---

## 📊 關鍵數據（觀眾常問）

| 項目 | 數值 |
|------|------|
| 簽章方案 | 3-of-5 (threshold/total) |
| 曲線 | secp256k1 |
| LoRa 延遲 | 500ms |
| 掉包率 | 10% |
| 分片大小 | 64 bytes |
| 協議 | FROST v2.2.0 |

---

## 🎯 展示成功標準

- ✅ 完整執行一次簽章流程
- ✅ Dashboard 成功顯示傳輸過程
- ✅ 清楚解釋技術亮點
- ✅ 回答 2-3 個技術問題
- ✅ 留下好印象和聯繫方式

---

## 📞 備忘

**Mac mini IP：** `___________________`

**WiFi SSID：** `___________________`

**Dashboard URL：**
```
http://____________:8000/dashboard.html?api=http://____________:3000
```

**緊急聯絡：** `___________________`

---

**列印本卡片並隨身攜帶！**

*Bitcoin++ Taipei 2025 - FROST-T Team*
