#!/bin/bash
# ============================================================================
# FROST-T Hackathon Demo - 完整啟動腳本（Mac mini）
# ============================================================================
#
# 用途：一鍵啟動所有服務（API + Dashboard）
# 功能：
#   - 同時啟動 FROST API 服務器（port 3000）
#   - 同時啟動 Dashboard HTTP 服務器（port 8000）
#   - 自動顯示連接資訊
#
# 使用方式：
#   chmod +x demo-hackathon-all.sh
#   ./demo-hackathon-all.sh
#
# ============================================================================

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║   FROST-T Hackathon Demo - Full Stack Mode                    ║"
echo "║   Mac mini: API Server + Dashboard Server                     ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 獲取本機 IP 地址
echo "🔍 Detecting network configuration..."
MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

if [ -z "$MAC_IP" ]; then
    echo "⚠️  Warning: Could not detect IP address. Using localhost only."
    MAC_IP="127.0.0.1"
else
    echo "✓ Mac mini IP: $MAC_IP"
fi

echo ""
echo "📋 Services Configuration:"
echo "   ┌────────────────────────────────────────────────┐"
echo "   │  API Server:       http://$MAC_IP:3000    │"
echo "   │  Dashboard Server: http://$MAC_IP:8000    │"
echo "   └────────────────────────────────────────────────┘"
echo ""
echo "📱 Access from Surface Go 4 or other devices:"
echo "   Dashboard: http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""

# 檢查是否已編譯
if [ ! -f "target/release/frost-threshold-signature" ]; then
    echo "⚙️  Building project..."
    cargo build --release
    echo "✓ Build complete"
    echo ""
fi

# 創建臨時目錄用於 PID 文件
mkdir -p /tmp/frost-demo

# 清理函數（確保退出時停止所有服務）
cleanup() {
    echo ""
    echo "🛑 Stopping all services..."

    if [ -f /tmp/frost-demo/api.pid ]; then
        API_PID=$(cat /tmp/frost-demo/api.pid)
        kill $API_PID 2>/dev/null || true
        rm /tmp/frost-demo/api.pid
    fi

    if [ -f /tmp/frost-demo/dashboard.pid ]; then
        DASH_PID=$(cat /tmp/frost-demo/dashboard.pid)
        kill $DASH_PID 2>/dev/null || true
        rm /tmp/frost-demo/dashboard.pid
    fi

    echo "✓ All services stopped"
    exit 0
}

trap cleanup INT TERM

echo "🚀 Starting services..."
echo ""

# 啟動 API 服務器（背景執行）
echo "   [1/2] Starting FROST API Server on port 3000..."
HOST=0.0.0.0 PORT=3000 cargo run --bin frost-threshold-signature --release > /tmp/frost-demo/api.log 2>&1 &
API_PID=$!
echo $API_PID > /tmp/frost-demo/api.pid
echo "   ✓ API Server started (PID: $API_PID)"

# 等待 API 服務器啟動
sleep 3

# 啟動 Dashboard 服務器（背景執行）
echo "   [2/2] Starting Dashboard Server on port 8000..."
python3 -m http.server 8000 > /tmp/frost-demo/dashboard.log 2>&1 &
DASH_PID=$!
echo $DASH_PID > /tmp/frost-demo/dashboard.pid
echo "   ✓ Dashboard Server started (PID: $DASH_PID)"

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✅ All services are running!"
echo ""
echo "📊 Quick Test:"
echo "   curl http://$MAC_IP:3000/health"
echo ""
echo "📱 Open Dashboard:"
echo "   open http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
echo "📝 Logs:"
echo "   • API:       tail -f /tmp/frost-demo/api.log"
echo "   • Dashboard: tail -f /tmp/frost-demo/dashboard.log"
echo ""
echo "Press Ctrl+C to stop all services"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# 保持腳本運行
wait
