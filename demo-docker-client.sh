#!/bin/bash
# ============================================================================
# FROST-T Docker Client - Surface Go 4 展示腳本
# ============================================================================
#
# 用途：在 Surface Go 4 上透過 SSH 控制 Mac mini Docker 容器
# 優點：Surface Go 4 完全不需要安裝任何東西（只需瀏覽器和 SSH）
#
# 使用方式：
#   ./demo-docker-client.sh [MAC_IP] [COMMAND]
#
# 範例：
#   ./demo-docker-client.sh 192.168.1.100 demo
#   ./demo-docker-client.sh 192.168.1.100 status
#
# ============================================================================

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 檢查參數
if [ $# -eq 0 ]; then
    echo "Usage: $0 <MAC_IP> [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  demo    - Run CLI demo (default)"
    echo "  status  - Check service status"
    echo "  logs    - View logs"
    echo "  custom  - Run custom CLI command"
    echo ""
    echo "Example:"
    echo "  $0 192.168.1.100 demo"
    echo ""
    exit 1
fi

MAC_IP=$1
MAC_USER=${MAC_USER:-mac}
PROJECT_PATH=${PROJECT_PATH:-"~/Documents/Prj/frost-threshold-signature"}
COMMAND=${2:-demo}

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   FROST-T Docker Client - Surface Go 4                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📋 Configuration:"
echo "   • Mac mini IP:  $MAC_IP"
echo "   • SSH User:     $MAC_USER"
echo "   • Command:      $COMMAND"
echo ""

# 測試連接
echo -e "${BLUE}🔍 Testing connection to Mac mini...${NC}"
if ! ssh -o ConnectTimeout=5 $MAC_USER@$MAC_IP "echo ''" 2>/dev/null; then
    echo -e "${RED}❌ Cannot connect to Mac mini${NC}"
    echo ""
    echo "Troubleshooting:"
    echo "  1. Check network: ping $MAC_IP"
    echo "  2. Test SSH: ssh $MAC_USER@$MAC_IP"
    echo "  3. Check Mac mini firewall settings"
    exit 1
fi

echo -e "${GREEN}✓ Connection successful${NC}"
echo ""

# 執行命令
case "$COMMAND" in
    demo)
        echo -e "${BLUE}🚀 Running FROST CLI demo in Docker container...${NC}"
        echo "═══════════════════════════════════════════════════════════════"
        echo ""

        ssh -t $MAC_USER@$MAC_IP "cd $PROJECT_PATH && \
            docker exec -it frost-api /app/frost-cli demo-basic -m 'Bitcoin++ Taipei 2025!'"

        echo ""
        echo "═══════════════════════════════════════════════════════════════"
        echo -e "${GREEN}✅ Demo completed!${NC}"
        ;;

    status)
        echo -e "${BLUE}📈 Checking service status...${NC}"
        echo ""

        ssh -t $MAC_USER@$MAC_IP "cd $PROJECT_PATH && ./demo-docker.sh status"
        ;;

    logs)
        echo -e "${BLUE}📊 Viewing logs (Press Ctrl+C to exit)...${NC}"
        echo ""

        ssh -t $MAC_USER@$MAC_IP "cd $PROJECT_PATH && ./demo-docker.sh logs"
        ;;

    custom)
        echo -e "${BLUE}💻 Custom CLI command mode${NC}"
        echo "Enter your CLI command (or 'exit' to quit):"
        echo ""

        read -p "> frost-cli " CLI_ARGS

        if [ "$CLI_ARGS" != "exit" ]; then
            ssh -t $MAC_USER@$MAC_IP "cd $PROJECT_PATH && \
                docker exec -it frost-api /app/frost-cli $CLI_ARGS"
        fi
        ;;

    *)
        echo -e "${RED}❌ Unknown command: $COMMAND${NC}"
        echo ""
        echo "Available commands: demo, status, logs, custom"
        exit 1
        ;;
esac

echo ""
echo -e "${BLUE}📊 View Dashboard:${NC}"
echo "   http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
echo ""
