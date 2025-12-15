# 🌐 FROST-T Tailscale 遠端訪問快速指南

> 在咖啡廳就能測試！透過 Tailscale VPN 連接家裡的 Mac mini

---

## ✅ 你的當前架構

```
┌─────────────────────────────┐         ┌──────────────────────────┐
│   Mac mini                  │         │   Surface Go 4           │
│   🏠 在家裡                  │         │   ☕ 在咖啡廳             │
├─────────────────────────────┤         ├──────────────────────────┤
│ • 家裡 WiFi                  │         │ • iPhone 4G 熱點         │
│ • 區域網路 IP:              │         │ • 行動網路               │
│   192.168.68.51             │    🔒    │                          │
│ • Tailscale IP:             │◄────────┤ • Tailscale VPN          │
│   100.110.164.70            │  VPN    │                          │
└─────────────────────────────┘         └──────────────────────────┘
```

**關鍵資訊**:
- ✅ Mac mini Tailscale IP: `100.110.164.70`
- ✅ 兩台設備已透過 Tailscale 連接
- ✅ 可以在任何地方訪問！

---

## 🚀 立即測試（3 步驟）

### 步驟 1：在 Surface Go 4 測試連接

**PowerShell / CMD:**
```powershell
# 測試 Tailscale 連接
ping 100.110.164.70

# 測試 API
curl http://100.110.164.70:3000/health
```

**預期結果：**
```json
{"status":"ok","signers_count":1,"active_sessions":0}
```

### 步驟 2：開啟瀏覽器

**在瀏覽器輸入任一網址：**

**選項 1：首頁（推薦）**
```
http://100.110.164.70:8000
```

**選項 2：Tailscale 專用首頁**
```
http://100.110.164.70:8000/index-tailscale.html
```

**選項 3：直接開啟 Dashboard**
```
http://100.110.164.70:8000/dashboard.html?api=http://100.110.164.70:3000
```

### 步驟 3：執行 CLI Demo

**下載並執行批次檔：**

在瀏覽器下載：
```
http://100.110.164.70:8000/demo-tailscale-client.bat
```

或在瀏覽器下載：
```
http://100.110.164.70:8000/open-dashboard-tailscale.bat
```

雙擊執行即可！

---

## 📥 快速下載連結

| 文件 | 下載網址 | 用途 |
|------|---------|------|
| Tailscale 客戶端 | `http://100.110.164.70:8000/demo-tailscale-client.bat` | 執行 CLI demo |
| Dashboard 快捷方式 | `http://100.110.164.70:8000/open-dashboard-tailscale.bat` | 一鍵開啟 Dashboard |
| 連接測試 | `http://100.110.164.70:8000/test-from-surface.bat` | 診斷連接問題 |

---

## 🎯 所有可用網址

### 網頁介面
- **首頁**: http://100.110.164.70:8000
- **Tailscale 首頁**: http://100.110.164.70:8000/index-tailscale.html
- **Dashboard**: http://100.110.164.70:8000/dashboard.html?api=http://100.110.164.70:3000
- **下載中心**: http://100.110.164.70:8000/download.html
- **連線測試**: http://100.110.164.70:8000/test.html

### API 端點
- **健康檢查**: http://100.110.164.70:3000/health
- **群組公鑰**: http://100.110.164.70:3000/pubkey
- **傳輸狀態**: http://100.110.164.70:3000/status

---

## 💻 SSH 遠端執行

### 基本用法

**在 Surface Go 4 的 PowerShell / CMD:**

```powershell
# SSH 連接到 Mac mini
ssh mac@100.110.164.70

# 執行 demo
cd ~/Documents/Prj/frost-threshold-signature
cargo run --bin frost-cli -- demo-basic -m "Hello from coffee shop!"
```

### 一行命令執行

```powershell
ssh mac@100.110.164.70 "cd ~/Documents/Prj/frost-threshold-signature && cargo run --bin frost-cli -- demo-basic"
```

---

## 🎨 完整展示流程（在咖啡廳）

### 1. 開啟服務（如果還沒啟動）

**SSH 到 Mac mini:**
```powershell
ssh mac@100.110.164.70
```

**啟動服務:**
```bash
cd ~/Documents/Prj/frost-threshold-signature
./demo-hackathon-all.sh
```

按 Ctrl+D 或輸入 `exit` 離開 SSH。

### 2. 開啟 Dashboard

**在 Surface Go 4 瀏覽器：**
```
http://100.110.164.70:8000/dashboard.html?api=http://100.110.164.70:3000
```

按 **F11** 進入全螢幕模式。

### 3. 執行 CLI Demo

**下載並執行:**
```
http://100.110.164.70:8000/demo-tailscale-client.bat
```

雙擊執行，選擇選項 1 (Run basic FROST demo)。

### 4. 觀察 Dashboard

在 Dashboard 中觀看即時的：
- LoRa 傳輸進度
- RSSI 訊號強度
- 封包掉失和重試
- 事件日誌

---

## 🛠️ 故障排除

### 問題 1：ping 不通 100.110.164.70

**原因：** Tailscale 未運行或未連接

**解決：**

**Surface Go 4:**
```powershell
# 檢查 Tailscale 狀態
tailscale status

# 如果未連接，重新登入
tailscale login
```

**Mac mini:**
```bash
# 檢查 Tailscale 狀態
tailscale status

# 查看 IP
tailscale ip -4
```

### 問題 2：瀏覽器無法載入

**原因：** Mac mini 服務未啟動

**解決：**
```powershell
# SSH 到 Mac mini
ssh mac@100.110.164.70

# 啟動服務
cd ~/Documents/Prj/frost-threshold-signature
./demo-hackathon-all.sh
```

### 問題 3：SSH 連接被拒絕

**原因：** Mac mini 未啟用遠端登入

**解決（在 Mac mini 上）：**
1. 系統偏好設定 → 共享
2. 勾選「遠端登入」
3. 允許「所有使用者」或特定使用者

### 問題 4：Dashboard 顯示 DISCONNECTED

**原因：** API URL 配置錯誤

**解決：**
確保使用正確的 URL 參數：
```
http://100.110.164.70:8000/dashboard.html?api=http://100.110.164.70:3000
                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                                           這部分很重要！
```

---

## ⚡ 性能優化建議

### 降低延遲

Tailscale 使用點對點連接，但如果兩台設備無法直連，會透過 DERP 中繼伺服器。

**檢查連接類型：**
```powershell
tailscale status
```

看 Mac mini 那一行：
- `direct` - 直連（最快）
- `derp` - 中繼（較慢但穩定）

### 使用就近的 DERP 伺服器

**設定偏好的區域：**
```bash
# Mac mini 和 Surface Go 4 都執行
tailscale set --accept-routes
```

---

## 🎯 優勢總結

### 為什麼 Tailscale 是完美方案？

✅ **在任何地方都能訪問**
- 咖啡廳 ☕
- 圖書館 📚
- 共同工作空間 💼
- 家裡 🏠

✅ **安全加密**
- WireGuard 協議
- 點對點加密
- 無需公開端口

✅ **零配置路由器**
- 不用設定 Port Forwarding
- 不用擔心防火牆
- 不用知道公網 IP

✅ **完美的開發體驗**
- 就像在同一網路
- 固定的 IP 地址
- 低延遲連接

---

## 📱 黑客松展示策略

### 方案 A：Tailscale 遠端展示（創新）

**適合場景：**
- 想展示雲端/分散式架構
- Mac mini 放在家裡
- 僅帶 Surface Go 4 到現場

**優點：**
- 展示真實的遠端連接
- 無需擔心會場網路
- 突顯分散式特性

**說詞範例：**
> "這個 Dashboard 正在顯示的數據，實際上來自我家裡的 Mac mini，
> 透過 Tailscale VPN 安全連接。這展示了即使節點在世界各地，
> 也能安全地執行門檻簽章協議。"

### 方案 B：本地熱點展示（穩定）

**適合場景：**
- 兩台設備都帶到現場
- 追求最低延遲
- 不依賴任何外部網路

**做法：**
回到家後，Mac mini 開啟熱點，Surface Go 4 連接。

---

## 🎊 現在就可以做的事

### 1. 立即測試（5 分鐘）

在 Surface Go 4 瀏覽器訪問：
```
http://100.110.164.70:8000
```

### 2. 執行完整 Demo（10 分鐘）

下載並執行：
```
http://100.110.164.70:8000/demo-tailscale-client.bat
```

### 3. 準備展示材料（30 分鐘）

閱讀文檔：
- HACKATHON-DEMO-GUIDE.md
- QUICK-DEMO-CARD.md
- 本文件

### 4. 練習展示流程（15 分鐘）

1. 開啟 Dashboard
2. 執行 CLI demo
3. 解釋技術亮點
4. 準備 Q&A 答案

---

## 📞 快速參考

**Mac mini Tailscale IP:**
```
100.110.164.70
```

**必記的兩個網址：**
```
首頁:      http://100.110.164.70:8000
Dashboard: http://100.110.164.70:8000/dashboard.html?api=http://100.110.164.70:3000
```

**SSH 連接:**
```bash
ssh mac@100.110.164.70
```

---

**你現在就可以在咖啡廳測試了！** ☕🚀

*Bitcoin++ Taipei 2025 - FROST-T Team*
