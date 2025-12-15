# 🌐 遠端訪問 Mac mini 設定指南

> 從咖啡廳訪問家裡的 Mac mini

---

## 🎯 情況說明

你現在：
- Surface Go 4 在咖啡廳（iPhone 熱點）
- Mac mini 在家裡（家裡 WiFi）
- 兩台設備不在同一網路，無法直接連接

---

## 📋 解決方案選項

### 方案 A：使用 SSH + 本地端口轉發（推薦）⭐

透過 SSH 隧道，將 Mac mini 的服務映射到 Surface Go 4 的本地端口。

#### 前提條件：
1. Mac mini 已啟用「遠端登入」（SSH）
2. 知道家裡網路的公網 IP 或使用動態 DNS
3. 路由器支援 Port Forwarding（端口轉發）

#### 設定步驟：

**1. 在家裡路由器設定 Port Forwarding**

登入路由器管理介面，添加規則：
```
外部端口 22 → Mac mini IP (192.168.68.51) 端口 22
```

**2. 查詢家裡的公網 IP**

在 Mac mini 上執行：
```bash
curl ifconfig.me
# 或
curl ipinfo.io/ip
```

記下這個 IP，例如：`123.45.67.89`

**3. 從 Surface Go 4 建立 SSH 隧道**

在 Surface Go 4 的 PowerShell 執行：
```powershell
# 建立 SSH 隧道，將 Mac mini 的服務映射到本地
ssh -L 3000:localhost:3000 -L 8000:localhost:8000 mac@123.45.67.89

# 輸入 Mac mini 的密碼
```

**4. 在 Surface Go 4 瀏覽器訪問**

```
http://localhost:8000
http://localhost:8000/dashboard.html?api=http://localhost:3000
```

這樣就可以像在本地一樣訪問了！

---

### 方案 B：使用 ngrok（最簡單，無需路由器設定）⭐⭐⭐

ngrok 可以將 Mac mini 的服務暴露到公網，無需配置路由器。

#### 在 Mac mini 上：

**1. 安裝 ngrok**
```bash
# 使用 Homebrew 安裝
brew install ngrok

# 或直接下載
curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
```

**2. 註冊 ngrok 帳號**
- 前往 https://ngrok.com/
- 註冊免費帳號
- 複製 authtoken

**3. 配置 ngrok**
```bash
ngrok config add-authtoken <your-authtoken>
```

**4. 啟動 ngrok 隧道**

在 Mac mini 上開兩個終端：

**終端 1：Dashboard (port 8000)**
```bash
ngrok http 8000
```

**終端 2：API (port 3000)**
```bash
ngrok http 3000
```

**5. 記下公網 URL**

ngrok 會顯示類似：
```
Forwarding    https://abcd-1234-5678.ngrok.io -> http://localhost:8000
```

**6. 在 Surface Go 4 瀏覽器訪問**
```
https://abcd-1234-5678.ngrok.io
https://abcd-1234-5678.ngrok.io/dashboard.html?api=https://efgh-9876.ngrok.io
```

---

### 方案 C：使用 Tailscale（最安全）⭐⭐

Tailscale 建立虛擬私有網路，讓兩台設備像在同一網路。

#### 設定步驟：

**1. 在兩台設備安裝 Tailscale**

Mac mini:
```bash
brew install tailscale
```

Surface Go 4:
下載安裝：https://tailscale.com/download/windows

**2. 登入同一帳號**

兩台設備都執行 Tailscale 並登入相同的帳號。

**3. 查看 Mac mini 的 Tailscale IP**

在 Mac mini 上：
```bash
tailscale ip -4
# 輸出例如：100.64.1.2
```

**4. 在 Surface Go 4 訪問**
```
http://100.64.1.2:8000
http://100.64.1.2:8000/dashboard.html?api=http://100.64.1.2:3000
```

---

### 方案 D：等回家後測試（最穩定）⭐

如果不急，最簡單的方式：
1. 把 Surface Go 4 帶回家
2. 連接家裡的 WiFi
3. 直接訪問 `http://192.168.68.51:8000`

這樣所有功能都能正常測試，而且是展示時的真實環境。

---

## 🆚 方案比較

| 方案 | 難度 | 速度 | 安全性 | 適合場景 |
|------|------|------|--------|---------|
| SSH 隧道 | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 臨時測試，懂 SSH |
| ngrok | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | 快速展示，不改路由器 |
| Tailscale | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 長期使用，安全第一 |
| 回家測試 | ⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 最穩定，真實環境 |

---

## 💡 我的建議

### 如果你現在就想測試：
使用 **ngrok**（最簡單）

### 如果可以等：
**回家後測試**（最穩定，最接近展示環境）

### 如果需要長期遠端訪問：
使用 **Tailscale**（最安全，最方便）

---

## 🚀 快速開始 - ngrok 方案

在 Mac mini 上執行：

```bash
# 安裝 ngrok
brew install ngrok

# 啟動服務（如果還沒啟動）
./demo-hackathon-all.sh

# 新終端：Dashboard 隧道
ngrok http 8000

# 再開新終端：API 隧道
ngrok http 3000
```

記下兩個 ngrok URL，然後在 Surface Go 4 瀏覽器訪問！

---

**選擇哪個方案？告訴我，我幫你設定！** 🎯
