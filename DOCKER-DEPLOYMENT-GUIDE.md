# 🐳 FROST-T Docker 部署指南

> 一鍵部署，Surface Go 4 零安裝展示方案

---

## 🎯 方案概述

**Docker 容器化部署 = 最簡單的展示方案！**

### 架構優勢

```
┌─────────────────────────────────┐         ┌──────────────────────┐
│   Mac mini                      │◄────────┤   Surface Go 4       │
│   (Docker 容器化)               │  WiFi   │   (零安裝！)         │
├─────────────────────────────────┤         ├──────────────────────┤
│ ✅ Docker Desktop (僅一次安裝)  │         │ ✅ Chrome 瀏覽器     │
│ ✅ docker-compose up (一鍵啟動) │         │ ✅ SSH (Windows內建) │
│                                 │         │                      │
│ Container 1: API Server :3000   │         │ ❌ 不需要 Rust      │
│ Container 2: Dashboard :8000    │         │ ❌ 不需要 Docker    │
│                                 │         │ ❌ 不需要任何開發工具│
└─────────────────────────────────┘         └──────────────────────┘
```

### ✅ 優點對比

| 特性 | Docker 方案 | 傳統方案 |
|------|------------|---------|
| Mac mini 安裝 | ✅ 僅 Docker Desktop | ⚠️ Rust + 編譯依賴 |
| Surface Go 4 安裝 | ✅ 無需安裝 | ❌ 可能需要 Rust |
| 啟動速度 | ✅ 1 分鐘 | ⚠️ 2-5 分鐘 |
| 環境隔離 | ✅ 完全隔離 | ❌ 系統環境 |
| 可移植性 | ✅ 極佳 | ⚠️ 依賴系統 |
| 清理乾淨度 | ✅ 一鍵刪除 | ⚠️ 手動清理 |

---

## 📋 前置準備

### Mac mini（僅一次設定）

#### 1. 安裝 Docker Desktop

**下載安裝：**
```bash
# 方式 1: 官網下載（推薦）
# https://www.docker.com/products/docker-desktop

# 方式 2: Homebrew
brew install --cask docker
```

**啟動 Docker Desktop：**
1. 打開 Applications → Docker
2. 等待 Docker 啟動（狀態列顯示綠色）
3. 驗證安裝：
   ```bash
   docker --version
   docker-compose --version
   ```

#### 2. 克隆專案（如果尚未完成）

```bash
cd ~/Documents/Prj
git clone <your-repo-url> frost-threshold-signature
cd frost-threshold-signature
```

### Surface Go 4（無需安裝任何東西）

**已內建工具：**
- ✅ Windows 10/11 內建 SSH 客戶端
- ✅ Microsoft Edge / Chrome 瀏覽器
- ✅ 完成！就這麼簡單

**驗證 SSH（可選）：**
```powershell
# Windows PowerShell
ssh -V
# 輸出：OpenSSH_for_Windows_8.x
```

---

## 🚀 快速開始（3 步驟）

### 步驟 1：Mac mini 啟動服務

```bash
cd ~/Documents/Prj/frost-threshold-signature

# 一鍵啟動所有服務
./demo-docker.sh start
```

**首次運行預期時間：**
- 構建 Docker 鏡像：5-10 分鐘（僅首次）
- 啟動容器：30 秒

**預期輸出：**
```
╔════════════════════════════════════════════════════════════════╗
║   FROST-T Hackathon Demo - Docker Mode                        ║
║   One-Click Deployment with Docker Compose                    ║
╚════════════════════════════════════════════════════════════════╝

🔍 Checking Docker installation...
✓ Docker is installed and running
✓ docker-compose is available

🚀 Starting FROST-T services with Docker...

Building Docker images (first time may take 5-10 minutes)...
Starting containers...

✅ Services started successfully!

═══════════════════════════════════════════════════════════════
🎉 FROST-T is now running!

📋 Access Information:
   ┌────────────────────────────────────────────────────┐
   │  API Server:       http://192.168.68.51:3000       │
   │  Dashboard Server: http://192.168.68.51:8000       │
   └────────────────────────────────────────────────────┘

📱 From Surface Go 4 (or any device):
   Dashboard: http://192.168.68.51:8000/dashboard.html?api=http://192.168.68.51:3000
```

記下顯示的 IP 地址！

### 步驟 2：Surface Go 4 執行 CLI Demo

**Linux / macOS / WSL：**
```bash
./demo-docker-client.sh 192.168.68.51 demo
```

**Windows (PowerShell / CMD)：**
```cmd
demo-docker-client.bat 192.168.68.51 demo
```

### 步驟 3：開啟 Dashboard

瀏覽器訪問（替換為實際 IP）：
```
http://192.168.68.51:8000/dashboard.html?api=http://192.168.68.51:3000
```

按 `F11` 進入全螢幕模式

---

## 💻 詳細使用指南

### Mac mini 服務器管理

#### 啟動服務
```bash
./demo-docker.sh start
```

#### 停止服務
```bash
./demo-docker.sh stop
```

#### 重啟服務
```bash
./demo-docker.sh restart
```

#### 查看日誌
```bash
./demo-docker.sh logs
# 按 Ctrl+C 退出
```

#### 查看狀態
```bash
./demo-docker.sh status
```

### Surface Go 4 客戶端操作

#### 運行基本 Demo
```bash
# Linux/macOS
./demo-docker-client.sh 192.168.68.51 demo

# Windows
demo-docker-client.bat 192.168.68.51 demo
```

#### 查看服務狀態
```bash
# Linux/macOS
./demo-docker-client.sh 192.168.68.51 status

# Windows
demo-docker-client.bat 192.168.68.51 status
```

#### 自訂 CLI 命令
```bash
# Linux/macOS
./demo-docker-client.sh 192.168.68.51 custom

# Windows
demo-docker-client.bat 192.168.68.51 custom
```

然後輸入你想執行的命令：
```bash
> frost-cli demo-basic --signers 2,4,5
> frost-cli demo-basic --full-payload
> frost-cli verify
```

---

## 🛠️ 進階操作

### 直接進入容器

```bash
# 進入 API 容器的 shell
docker exec -it frost-api sh

# 在容器內執行 CLI
/app/frost-cli demo-basic -m "Hello Docker"

# 退出容器
exit
```

### 查看容器日誌

```bash
# 查看 API 日誌
docker logs frost-api

# 查看 Dashboard 日誌
docker logs frost-dashboard

# 持續追蹤日誌
docker logs -f frost-api
```

### 重新構建鏡像（代碼更新後）

```bash
# 停止服務
./demo-docker.sh stop

# 重新構建
docker-compose build --no-cache

# 啟動
./demo-docker.sh start
```

### 完全清理（釋放空間）

```bash
# 停止並刪除容器
docker-compose down

# 刪除鏡像
docker rmi $(docker images | grep frost | awk '{print $3}')

# 清理未使用的資源
docker system prune -a
```

---

## 🐛 故障排除

### 問題 1：Docker Desktop 未運行

**錯誤訊息：**
```
Cannot connect to the Docker daemon
```

**解決方案：**
1. 打開 Applications → Docker
2. 等待 Docker 圖標變為綠色
3. 重新執行腳本

### 問題 2：Port 已被佔用

**錯誤訊息：**
```
Error starting userland proxy: listen tcp4 0.0.0.0:3000: bind: address already in use
```

**解決方案：**
```bash
# 查找佔用端口的程序
lsof -i :3000

# 停止該程序
kill -9 <PID>

# 或使用不同的端口
# 編輯 docker-compose.yml，修改 ports 配置
```

### 問題 3：構建失敗

**錯誤訊息：**
```
Error building image
```

**解決方案：**
```bash
# 清理 Docker 緩存
docker system prune -a

# 重新構建
docker-compose build --no-cache
```

### 問題 4：容器啟動但無法訪問

**檢查健康狀態：**
```bash
docker-compose ps

# 預期輸出應該顯示 "Up" 和 "healthy"
```

**查看容器日誌：**
```bash
docker logs frost-api
docker logs frost-dashboard
```

**測試 API 端點：**
```bash
curl http://localhost:3000/health
# 預期：{"status":"healthy","version":"0.1.0"}
```

### 問題 5：Surface Go 4 無法連接

**檢查清單：**
- [ ] Mac mini 和 Surface Go 4 在同一 WiFi
- [ ] Mac mini 防火牆允許端口 3000、8000
- [ ] Docker 容器正在運行（`docker ps`）
- [ ] 可以 ping 通 Mac mini IP

**測試連接：**
```bash
# Surface Go 4 上測試
ping 192.168.68.51
curl http://192.168.68.51:3000/health
```

---

## 📊 性能對比

### Docker vs 傳統部署

| 指標 | Docker | 傳統 | 差異 |
|------|--------|------|------|
| 首次設定時間 | 10 分鐘 | 15-30 分鐘 | 快 2-3 倍 |
| 啟動時間 | 30 秒 | 1-2 分鐘 | 快 2-4 倍 |
| 內存佔用 | ~200MB | ~150MB | 多 50MB |
| 可移植性 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | 更佳 |
| 清理乾淨度 | ⭐⭐⭐⭐⭐ | ⭐⭐ | 更佳 |

---

## 🎨 展示最佳實踐

### 1. 提前準備（展示前一天）

```bash
# Mac mini
cd ~/Documents/Prj/frost-threshold-signature

# 拉取最新代碼（如有更新）
git pull

# 構建 Docker 鏡像（避免現場等待）
./demo-docker.sh start
./demo-docker.sh stop
```

### 2. 展示當天（提前 30 分鐘）

```bash
# Mac mini - 啟動服務
./demo-docker.sh start

# 驗證服務
./demo-docker.sh status

# 記錄 IP 地址
ifconfig | grep "inet " | grep -v 127.0.0.1
```

**Surface Go 4 - 準備瀏覽器：**
1. 開啟 Chrome 瀏覽器
2. 訪問 Dashboard URL
3. 按 F11 全螢幕
4. 準備 SSH 終端（Windows Terminal 或 PowerShell）

### 3. 展示流程

**主螢幕（投影）：** Dashboard F11 全螢幕
**副螢幕（操作）：** Terminal SSH 到 Mac mini

**執行順序：**
1. 介紹專案（30 秒）
2. 執行 CLI demo（2 分鐘）
3. 切換到 Dashboard 觀看即時視覺化（2 分鐘）
4. Q&A（1 分鐘）

---

## 📝 Docker 檔案說明

### 專案新增的 Docker 文件

```
frost-threshold-signature/
├── Dockerfile                  - Docker 鏡像定義（多階段構建）
├── docker-compose.yml          - 服務編排配置
├── nginx.conf                  - Nginx 配置（Dashboard 服務器）
├── .dockerignore               - Docker 構建忽略文件
├── demo-docker.sh              - Mac mini 一鍵啟動腳本
├── demo-docker-client.sh       - Surface Go 4 客戶端（Linux/macOS）
└── demo-docker-client.bat      - Surface Go 4 客戶端（Windows）
```

### Dockerfile 架構

```dockerfile
# Stage 1: Builder（構建階段）
- 使用 rust:1.75-slim 基礎鏡像
- 安裝編譯依賴
- 編譯 Rust 應用（Release 模式）

# Stage 2: Runtime（運行階段）
- 使用 debian:bookworm-slim 輕量鏡像
- 僅複製編譯好的二進制文件
- 最終鏡像大小：~100MB（vs 源碼 + 工具鏈 ~2GB）
```

### docker-compose.yml 服務

```yaml
services:
  frost-api:          # FROST API 服務器
    - Port: 3000
    - Health check: /health
    - Auto-restart

  frost-dashboard:    # Dashboard 靜態服務
    - Port: 8000
    - Nginx Alpine（僅 ~10MB）
    - Auto-restart
```

---

## 🎯 關鍵優勢總結

### 為什麼選擇 Docker 方案？

#### 1. **Surface Go 4 零安裝** ⭐⭐⭐⭐⭐
- 不需要安裝 Rust（~2GB）
- 不需要安裝 Docker（~500MB）
- 不需要任何開發工具
- 僅需瀏覽器 + SSH（Windows 內建）

#### 2. **Mac mini 設定簡單** ⭐⭐⭐⭐⭐
- 僅安裝 Docker Desktop（一次性）
- 一鍵啟動：`./demo-docker.sh start`
- 一鍵停止：`./demo-docker.sh stop`
- 環境完全隔離，不污染系統

#### 3. **展示流暢專業** ⭐⭐⭐⭐⭐
- 啟動速度快（30 秒 vs 2-5 分鐘）
- 穩定可靠（容器隔離，無依賴衝突）
- 可重複性高（環境一致）

#### 4. **易於分享和部署** ⭐⭐⭐⭐⭐
- 提供給評審/觀眾測試：只需 Docker
- 跨平台（Mac / Linux / Windows with WSL2）
- 一鍵部署到雲端（AWS / GCP / Azure）

---

## 🚀 下一步

### 展示前檢查清單

**Mac mini：**
- [ ] 安裝 Docker Desktop
- [ ] 克隆專案代碼
- [ ] 執行 `./demo-docker.sh start`
- [ ] 驗證服務正常（`./demo-docker.sh status`）
- [ ] 記錄 Mac mini IP 地址

**Surface Go 4：**
- [ ] 測試 SSH 連接到 Mac mini
- [ ] 瀏覽器測試訪問 Dashboard
- [ ] 準備展示腳本（`demo-docker-client.bat`）

**網路：**
- [ ] 兩台設備在同一 WiFi
- [ ] Mac mini 防火牆允許端口 3000、8000
- [ ] 測試跨設備訪問

### 緊急備案

如果 Docker 出現問題，可以快速切換回傳統方案：
```bash
# 停止 Docker
./demo-docker.sh stop

# 啟動傳統服務
./demo-hackathon-all.sh
```

---

## 📞 支援資源

**相關文檔：**
- [HACKATHON-DEMO-GUIDE.md](./HACKATHON-DEMO-GUIDE.md) - 完整展示指南
- [DEMO-SETUP-SUMMARY.md](./DEMO-SETUP-SUMMARY.md) - 快速摘要
- [QUICK-DEMO-CARD.md](./QUICK-DEMO-CARD.md) - 現場快速參考

**Docker 官方文檔：**
- [Docker Desktop for Mac](https://docs.docker.com/desktop/install/mac-install/)
- [Docker Compose](https://docs.docker.com/compose/)

---

**祝 Docker 部署順利！🐳**

*Bitcoin++ Taipei 2025 - FROST-T Team*
