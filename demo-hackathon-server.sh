#!/bin/bash
# ============================================================================
# FROST-T Hackathon Demo - Mac mini Server 啟動腳本
# ============================================================================
#
# 用途：在 Mac mini 上啟動 FROST API 服務器
# 功能：
#   - 啟動 HTTP API 服務（綁定到所有網路介面）
#   - 自動獲取本機 IP 地址
#   - 顯示連接資訊供其他設備使用
#
# 使用方式：
#   chmod +x demo-hackathon-server.sh
#   ./demo-hackathon-server.sh
#
# ============================================================================

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║   FROST-T Hackathon Demo - Server Mode                        ║"
echo "║   Mac mini as Backend Server                                  ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 獲取本機 IP 地址（排除 localhost）
echo "🔍 Detecting network configuration..."
MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

if [ -z "$MAC_IP" ]; then
    echo "⚠️  Warning: Could not detect IP address. Using localhost only."
    MAC_IP="127.0.0.1"
else
    echo "✓ Detected Mac mini IP: $MAC_IP"
fi

echo ""
echo "📋 Server Configuration:"
echo "   • Host: 0.0.0.0 (all network interfaces)"
echo "   • Port: 3000"
echo "   • Access URL: http://$MAC_IP:3000"
echo ""
echo "📱 Client Connection Info (for Surface Go 4):"
echo "   ┌──────────────────────────────────────────────────────┐"
echo "   │  API Server: http://$MAC_IP:3000              │"
echo "   │  Dashboard:  http://$MAC_IP:8000              │"
echo "   └──────────────────────────────────────────────────────┘"
echo ""
echo "💡 On Surface Go 4, open dashboard with:"
echo "   http://$MAC_IP:8000?api=http://$MAC_IP:3000"
echo ""

# 檢查是否已編譯
if [ ! -f "target/release/frost-threshold-signature" ]; then
    echo "⚙️  Building project (this may take a few minutes)..."
    cargo build --release
    echo "✓ Build complete"
    echo ""
fi

# 啟動 API 服務器（背景執行）
echo "🚀 Starting FROST API Server..."
echo "   (Press Ctrl+C to stop)"
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo ""

# 設定環境變數並啟動
HOST=0.0.0.0 PORT=3000 cargo run --bin frost-threshold-signature --release

# 注意：服務器會持續運行直到手動停止
