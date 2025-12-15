#!/bin/bash
# ============================================================================
# FROST-T Hackathon Demo - Client 展示腳本（Surface Go 4）
# ============================================================================
#
# 用途：在 Surface Go 4 上透過 SSH 執行 FROST CLI 命令
# 前提：
#   1. Mac mini 已啟動 API 服務器（執行 demo-hackathon-all.sh）
#   2. 已設定 SSH 免密碼登入到 Mac mini
#
# 使用方式：
#   chmod +x demo-hackathon-client.sh
#   ./demo-hackathon-client.sh [MAC_IP]
#
# 範例：
#   ./demo-hackathon-client.sh 192.168.1.100
#
# ============================================================================

# 檢查參數
if [ $# -eq 0 ]; then
    echo "Usage: $0 <MAC_IP>"
    echo ""
    echo "Example:"
    echo "  $0 192.168.1.100"
    echo ""
    echo "Make sure Mac mini is running the API server first!"
    exit 1
fi

MAC_IP=$1
MAC_USER=${MAC_USER:-mac}  # 預設使用者名稱，可透過環境變數覆蓋
PROJECT_PATH=${PROJECT_PATH:-"~/Documents/Prj/frost-threshold-signature"}

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║   FROST-T Hackathon Demo - Client Mode                        ║"
echo "║   Surface Go 4 → Mac mini SSH Remote Execution                ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📋 Configuration:"
echo "   • Mac mini IP:  $MAC_IP"
echo "   • SSH User:     $MAC_USER"
echo "   • Project Path: $PROJECT_PATH"
echo ""

# 測試連接
echo "🔍 Testing connection to Mac mini..."
if ! ssh -o ConnectTimeout=5 $MAC_USER@$MAC_IP "echo '✓ SSH connection successful'" 2>/dev/null; then
    echo "❌ Cannot connect to Mac mini"
    echo ""
    echo "Troubleshooting:"
    echo "  1. Check Mac mini IP address: ping $MAC_IP"
    echo "  2. Check SSH service: ssh $MAC_USER@$MAC_IP"
    echo "  3. Setup SSH key: ssh-copy-id $MAC_USER@$MAC_IP"
    exit 1
fi

echo ""
echo "🚀 Starting FROST Demo..."
echo "═══════════════════════════════════════════════════════════════"
echo ""

# 執行 CLI demo
echo "📡 Executing FROST CLI demo on Mac mini..."
echo ""

ssh -t $MAC_USER@$MAC_IP "cd $PROJECT_PATH && cargo run --bin frost-cli -- demo-basic -m 'Hello Bitcoin++ Taipei 2025!'"

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✅ Demo completed!"
echo ""
echo "📊 View Dashboard:"
echo "   http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
