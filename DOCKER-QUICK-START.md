# 🐳 FROST-T Docker 快速開始

> 3 分鐘啟動，Surface Go 4 零安裝！

---

## ⚡ 超級快速開始

### Mac mini（僅一次設定）

```bash
# 1. 安裝 Docker Desktop
# 下載：https://www.docker.com/products/docker-desktop
# 或：brew install --cask docker

# 2. 啟動服務（首次需 5-10 分鐘構建）
cd ~/Documents/Prj/frost-threshold-signature
./demo-docker.sh start

# 記下顯示的 IP 地址，例如：192.168.68.51
```

### Surface Go 4（無需安裝任何東西）

**終端（執行 CLI）：**
```bash
# Windows
demo-docker-client.bat 192.168.68.51 demo
```

**瀏覽器（Dashboard）：**
```
http://192.168.68.51:8000/dashboard.html?api=http://192.168.68.51:3000
```

---

## 📋 常用命令

### Mac mini 服務器管理

| 操作 | 命令 |
|------|------|
| 啟動 | `./demo-docker.sh start` |
| 停止 | `./demo-docker.sh stop` |
| 重啟 | `./demo-docker.sh restart` |
| 日誌 | `./demo-docker.sh logs` |
| 狀態 | `./demo-docker.sh status` |

### Surface Go 4 客戶端

| 操作 | Windows 命令 |
|------|--------------|
| 基本 Demo | `demo-docker-client.bat <IP> demo` |
| 查看狀態 | `demo-docker-client.bat <IP> status` |
| 自訂命令 | `demo-docker-client.bat <IP> custom` |

---

## ✅ 優勢對比

| 特性 | Docker 方案 | 傳統方案 |
|------|------------|---------|
| Surface Go 4 安裝 | ✅ **零安裝** | ❌ 需要 Rust |
| Mac mini 設定 | ✅ 僅 Docker | ⚠️ Rust + 依賴 |
| 啟動時間 | ✅ **30 秒** | ⚠️ 1-2 分鐘 |
| 環境隔離 | ✅ **完全隔離** | ❌ 系統環境 |
| 一鍵清理 | ✅ **docker-compose down** | ⚠️ 手動清理 |

---

## 🎯 展示流程

### 步驟 1：Mac mini 啟動（1 分鐘）
```bash
./demo-docker.sh start
# 等待顯示 IP 地址
```

### 步驟 2：Surface Go 4 執行 Demo（2 分鐘）
```bash
demo-docker-client.bat 192.168.68.51 demo
```

### 步驟 3：開啟 Dashboard（F11 全螢幕）
```
http://192.168.68.51:8000/dashboard.html?api=http://192.168.68.51:3000
```

---

## 🐛 快速故障排除

### Docker 未啟動
```bash
# 打開 Applications → Docker
# 等待綠色圖標
```

### Port 被佔用
```bash
lsof -i :3000
kill -9 <PID>
./demo-docker.sh restart
```

### 無法連接
```bash
# 檢查服務狀態
./demo-docker.sh status

# 測試 API
curl http://localhost:3000/health
```

---

## 📚 完整文檔

- **完整指南：** [DOCKER-DEPLOYMENT-GUIDE.md](./DOCKER-DEPLOYMENT-GUIDE.md)
- **展示指南：** [HACKATHON-DEMO-GUIDE.md](./HACKATHON-DEMO-GUIDE.md)
- **快速參考：** [QUICK-DEMO-CARD.md](./QUICK-DEMO-CARD.md)

---

**Surface Go 4 不需要安裝任何東西！🎉**

*Bitcoin++ Taipei 2025*
