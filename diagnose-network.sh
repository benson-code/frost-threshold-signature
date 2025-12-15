#!/bin/bash
# ============================================================================
# 網路診斷腳本 - 幫助排查連線問題
# ============================================================================

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   FROST-T Network Diagnostics                                 ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 獲取 Mac mini IP
echo "🔍 Detecting Mac mini IP addresses..."
echo ""
ifconfig | grep "inet " | grep -v 127.0.0.1 | while read -r line; do
    IP=$(echo "$line" | awk '{print $2}')
    INTERFACE=$(echo "$line" | awk '{print $1}')
    echo "   • $IP"
done

MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "📋 Connection Information for Surface Go 4:"
echo ""
echo "   ⚠️  DO NOT USE: http://127.0.0.1:3000"
echo "   ⚠️  DO NOT USE: http://localhost:3000"
echo ""
echo "   ✅ USE THIS INSTEAD:"
echo ""
echo "   Dashboard (推薦):"
echo "   http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
echo "   API Health Check:"
echo "   http://$MAC_IP:3000/health"
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo ""

# 測試本地服務
echo "🧪 Testing local services on Mac mini..."
echo ""

if curl -sf http://localhost:3000/health > /dev/null 2>&1; then
    echo "   ✅ API Server (port 3000) is running"
    curl -s http://localhost:3000/health | jq . 2>/dev/null || curl -s http://localhost:3000/health
else
    echo "   ❌ API Server (port 3000) is NOT running"
    echo "      Run: ./demo-hackathon-all.sh"
fi

echo ""

if curl -sf http://localhost:8000/health > /dev/null 2>&1; then
    echo "   ✅ Dashboard Server (port 8000) is running"
else
    echo "   ❌ Dashboard Server (port 8000) is NOT running"
    echo "      Run: ./demo-hackathon-all.sh"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "📱 Test from Surface Go 4:"
echo ""
echo "   1. Open PowerShell or Command Prompt on Surface Go 4"
echo ""
echo "   2. Test connectivity:"
echo "      ping $MAC_IP"
echo ""
echo "   3. Test API endpoint:"
echo "      curl http://$MAC_IP:3000/health"
echo ""
echo "   4. Open browser and visit:"
echo "      http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo ""
