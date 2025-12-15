#!/bin/bash
# ============================================================================
# FROST-T Dashboard 靜態服務器
# ============================================================================
#
# 用途：提供 Dashboard 的 HTTP 靜態文件服務
# 功能：
#   - 在 port 8000 啟動簡單的 HTTP 服務器
#   - 允許從其他設備訪問 Dashboard
#
# 使用方式：
#   chmod +x serve-dashboard.sh
#   ./serve-dashboard.sh
#
# ============================================================================

set -e

# 獲取本機 IP
MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   FROST-T Dashboard Server                                     ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "🌐 Starting HTTP server on port 8000..."
echo ""
echo "📱 Access Dashboard from:"
echo "   • Local:   http://localhost:8000/dashboard.html"
echo "   • Network: http://$MAC_IP:8000/dashboard.html"
echo ""
echo "💡 With API parameter:"
echo "   http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
echo "Press Ctrl+C to stop"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# 使用 Python 3 的簡單 HTTP 服務器
python3 -m http.server 8000
