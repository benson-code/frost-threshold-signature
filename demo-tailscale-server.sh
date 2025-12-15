#!/bin/bash
# ============================================================================
# FROST-T Tailscale Demo - Server Script (Mac mini)
# ============================================================================
#
# Purpose: Start FROST services accessible via Tailscale
# Usage:
#   chmod +x demo-tailscale-server.sh
#   ./demo-tailscale-server.sh
#
# ============================================================================

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                                                                ║"
echo "║   FROST-T Tailscale Demo - Server Mode                        ║"
echo "║   Remote access via Tailscale VPN                             ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Get Tailscale IP
echo "🔍 Detecting Tailscale configuration..."
TAILSCALE_IP=$(/Applications/Tailscale.app/Contents/MacOS/Tailscale ip -4 2>/dev/null || echo "")

if [ -z "$TAILSCALE_IP" ]; then
    echo "❌ Error: Tailscale not running or not configured"
    echo "   Please start Tailscale and try again."
    exit 1
fi

echo "✓ Tailscale IP: $TAILSCALE_IP"
echo ""

# Also get local IP for reference
LOCAL_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)
if [ -n "$LOCAL_IP" ]; then
    echo "ℹ️  Local IP: $LOCAL_IP (for reference)"
fi

echo ""
echo "📋 Services Configuration:"
echo "   ┌────────────────────────────────────────────────────┐"
echo "   │  API Server:       http://$TAILSCALE_IP:3000      │"
echo "   │  Dashboard Server: http://$TAILSCALE_IP:8000      │"
echo "   └────────────────────────────────────────────────────┘"
echo ""
echo "📱 Access from Surface Go 4 (via Tailscale):"
echo "   Welcome Page: http://$TAILSCALE_IP:8000/index-tailscale.html"
echo "   Dashboard:    http://$TAILSCALE_IP:8000/dashboard.html?api=http://$TAILSCALE_IP:3000"
echo ""
echo "🔗 Quick Access URLs:"
echo "   Copy these to Surface Go 4 browser:"
echo "   ├─ http://$TAILSCALE_IP:8000/index-tailscale.html"
echo "   └─ http://$TAILSCALE_IP:8000/dashboard.html?api=http://$TAILSCALE_IP:3000"
echo ""

# Check if already compiled
if [ ! -f "target/release/frost-threshold-signature" ]; then
    echo "⚙️  Building project..."
    cargo build --release
    echo "✓ Build complete"
    echo ""
fi

# Create temp directory for PID files
mkdir -p /tmp/frost-demo

# Cleanup function
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

# Start API Server (background)
echo "   [1/2] Starting FROST API Server on port 3000..."
HOST=0.0.0.0 PORT=3000 cargo run --bin frost-threshold-signature --release > /tmp/frost-demo/api.log 2>&1 &
API_PID=$!
echo $API_PID > /tmp/frost-demo/api.pid
echo "   ✓ API Server started (PID: $API_PID)"

# Wait for API server to start
sleep 3

# Start Dashboard Server (background)
echo "   [2/2] Starting Dashboard Server on port 8000..."
python3 -m http.server 8000 > /tmp/frost-demo/dashboard.log 2>&1 &
DASH_PID=$!
echo $DASH_PID > /tmp/frost-demo/dashboard.pid
echo "   ✓ Dashboard Server started (PID: $DASH_PID)"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All services running!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Service Status:"
echo "   API Server:       Running on $TAILSCALE_IP:3000"
echo "   Dashboard Server: Running on $TAILSCALE_IP:8000"
echo ""
echo "📝 Logs:"
echo "   API:       tail -f /tmp/frost-demo/api.log"
echo "   Dashboard: tail -f /tmp/frost-demo/dashboard.log"
echo ""
echo "🎯 Next Steps:"
echo "   1. On Surface Go 4, open browser"
echo "   2. Navigate to: http://$TAILSCALE_IP:8000/index-tailscale.html"
echo "   3. Click on 'Dashboard' to see FROST status"
echo ""
echo "⏸️  Press Ctrl+C to stop all services"
echo ""

# Wait forever (until Ctrl+C)
wait
