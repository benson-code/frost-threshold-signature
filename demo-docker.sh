#!/bin/bash
# ============================================================================
# FROST-T Docker 一鍵展示腳本
# ============================================================================
#
# 用途：在 Mac mini 上使用 Docker 一鍵啟動所有服務
# 優點：
#   - 無需安裝 Rust 工具鏈（Docker 容器內編譯）
#   - 環境隔離，不污染系統
#   - 一鍵啟動/停止
#   - Surface Go 4 完全不需要安裝任何東西
#
# 前提：Mac mini 已安裝 Docker Desktop
#   下載：https://www.docker.com/products/docker-desktop
#
# 使用方式：
#   ./demo-docker.sh start    # 啟動服務
#   ./demo-docker.sh stop     # 停止服務
#   ./demo-docker.sh restart  # 重啟服務
#   ./demo-docker.sh logs     # 查看日誌
#   ./demo-docker.sh status   # 查看狀態
#
# ============================================================================

set -e

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 函數：印出橫幅
print_banner() {
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                                                                ║"
    echo "║   FROST-T Hackathon Demo - Docker Mode                        ║"
    echo "║   One-Click Deployment with Docker Compose                    ║"
    echo "║                                                                ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
}

# 函數：檢查 Docker
check_docker() {
    echo -e "${BLUE}🔍 Checking Docker installation...${NC}"

    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker is not installed${NC}"
        echo ""
        echo "Please install Docker Desktop:"
        echo "  https://www.docker.com/products/docker-desktop"
        echo ""
        exit 1
    fi

    if ! docker info &> /dev/null; then
        echo -e "${RED}❌ Docker daemon is not running${NC}"
        echo ""
        echo "Please start Docker Desktop and try again."
        echo ""
        exit 1
    fi

    echo -e "${GREEN}✓ Docker is installed and running${NC}"

    if ! command -v docker-compose &> /dev/null; then
        echo -e "${RED}❌ docker-compose is not installed${NC}"
        echo ""
        echo "Please install docker-compose or use Docker Desktop."
        echo ""
        exit 1
    fi

    echo -e "${GREEN}✓ docker-compose is available${NC}"
    echo ""
}

# 函數：獲取 IP 地址
get_ip() {
    MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

    if [ -z "$MAC_IP" ]; then
        MAC_IP="127.0.0.1"
    fi

    echo "$MAC_IP"
}

# 函數：啟動服務
start_services() {
    print_banner
    check_docker

    echo -e "${BLUE}🚀 Starting FROST-T services with Docker...${NC}"
    echo ""

    # 構建並啟動容器
    echo -e "${YELLOW}Building Docker images (first time may take 5-10 minutes)...${NC}"
    docker-compose build

    echo ""
    echo -e "${YELLOW}Starting containers...${NC}"
    docker-compose up -d

    echo ""
    echo -e "${GREEN}✅ Services started successfully!${NC}"
    echo ""

    # 等待服務啟動
    echo -e "${YELLOW}Waiting for services to be ready...${NC}"
    sleep 5

    # 檢查健康狀態
    if curl -sf http://localhost:3000/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ API Server is healthy${NC}"
    else
        echo -e "${YELLOW}⚠ API Server is starting...${NC}"
    fi

    if curl -sf http://localhost:8000/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Dashboard Server is healthy${NC}"
    else
        echo -e "${YELLOW}⚠ Dashboard Server is starting...${NC}"
    fi

    echo ""

    # 顯示訪問資訊
    MAC_IP=$(get_ip)

    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${GREEN}🎉 FROST-T is now running!${NC}"
    echo ""
    echo "📋 Access Information:"
    echo "   ┌────────────────────────────────────────────────────┐"
    echo "   │  API Server:       http://$MAC_IP:3000        │"
    echo "   │  Dashboard Server: http://$MAC_IP:8000        │"
    echo "   └────────────────────────────────────────────────────┘"
    echo ""
    echo "📱 From Surface Go 4 (or any device):"
    echo "   Dashboard: http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
    echo ""
    echo "💻 Run CLI demo:"
    echo "   docker exec -it frost-api /app/frost-cli demo-basic -m \"Bitcoin++ 2025\""
    echo ""
    echo "📊 View logs:"
    echo "   ./demo-docker.sh logs"
    echo ""
    echo "🛑 Stop services:"
    echo "   ./demo-docker.sh stop"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
}

# 函數：停止服務
stop_services() {
    echo -e "${BLUE}🛑 Stopping FROST-T services...${NC}"
    echo ""

    docker-compose down

    echo ""
    echo -e "${GREEN}✅ Services stopped successfully!${NC}"
    echo ""
}

# 函數：重啟服務
restart_services() {
    echo -e "${BLUE}🔄 Restarting FROST-T services...${NC}"
    echo ""

    docker-compose restart

    echo ""
    echo -e "${GREEN}✅ Services restarted successfully!${NC}"
    echo ""
}

# 函數：查看日誌
view_logs() {
    echo -e "${BLUE}📊 Viewing service logs (Press Ctrl+C to exit)...${NC}"
    echo ""

    docker-compose logs -f
}

# 函數：查看狀態
show_status() {
    echo -e "${BLUE}📈 Service Status:${NC}"
    echo ""

    docker-compose ps

    echo ""
    echo -e "${BLUE}🔍 Health Check:${NC}"
    echo ""

    if curl -sf http://localhost:3000/health > /dev/null 2>&1; then
        echo -e "  API Server:       ${GREEN}✓ Healthy${NC}"
    else
        echo -e "  API Server:       ${RED}✗ Unhealthy${NC}"
    fi

    if curl -sf http://localhost:8000/health > /dev/null 2>&1; then
        echo -e "  Dashboard Server: ${GREEN}✓ Healthy${NC}"
    else
        echo -e "  Dashboard Server: ${RED}✗ Unhealthy${NC}"
    fi

    echo ""
}

# 主程式
case "${1:-start}" in
    start)
        start_services
        ;;
    stop)
        stop_services
        ;;
    restart)
        restart_services
        ;;
    logs)
        view_logs
        ;;
    status)
        show_status
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|status}"
        echo ""
        echo "Commands:"
        echo "  start   - Build and start all services"
        echo "  stop    - Stop all services"
        echo "  restart - Restart all services"
        echo "  logs    - View service logs"
        echo "  status  - Show service status"
        echo ""
        exit 1
        ;;
esac
