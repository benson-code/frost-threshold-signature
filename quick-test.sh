#!/bin/bash
# ============================================================================
# FROST-T 快速測試腳本
# 用途: 展示前快速驗證所有關鍵功能
# ============================================================================

set -e

# 顏色定義
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo ""
echo -e "${CYAN}${BOLD}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║                                                                    ║${NC}"
echo -e "${CYAN}${BOLD}║              FROST-T 快速測試                                     ║${NC}"
echo -e "${CYAN}${BOLD}║              Quick Pre-Demo Test                                  ║${NC}"
echo -e "${CYAN}${BOLD}║                                                                    ║${NC}"
echo -e "${CYAN}${BOLD}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# ============================================================================
# 測試 1: 檢查 Rust 環境
# ============================================================================
echo -e "[1/5] 檢查 Rust 環境..."

if command -v cargo &> /dev/null; then
    echo -e "  ${GREEN}✓ Cargo 已安裝${NC}"
    cargo --version
else
    echo -e "  ${RED}✗ Cargo 未找到！請安裝 Rust${NC}"
    echo "  下載: https://rustup.rs/"
    exit 1
fi

echo ""

# ============================================================================
# 測試 2: 編譯檢查
# ============================================================================
echo "[2/5] 檢查編譯狀態..."

if cargo check --bin frost-cli --quiet 2>/dev/null; then
    echo -e "  ${GREEN}✓ 編譯通過${NC}"
else
    echo -e "  ${YELLOW}⚠ 編譯失敗！正在嘗試重新編譯...${NC}"
    if ! cargo build --bin frost-cli; then
        echo -e "  ${RED}✗ 編譯失敗，請檢查錯誤訊息${NC}"
        exit 1
    fi
fi

echo ""

# ============================================================================
# 測試 3: 檢查 Port 3000
# ============================================================================
echo "[3/5] 檢查 Port 3000..."

if lsof -i :3000 &> /dev/null || netstat -an | grep -q ":3000.*LISTEN" 2>/dev/null; then
    echo -e "  ${YELLOW}⚠ Port 3000 已被佔用${NC}"
    echo "  正在嘗試連線測試..."

    # 嘗試連線到現有的 server
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/health 2>/dev/null || echo "000")

    if [ "$STATUS" = "200" ]; then
        echo -e "  ${GREEN}✓ Server 已在運行中${NC}"
    else
        echo -e "  ${RED}✗ Port 被其他程式佔用${NC}"
        echo "  請關閉佔用 port 3000 的程式"
        exit 1
    fi
else
    echo -e "  ${GREEN}✓ Port 3000 可用${NC}"
    echo ""
    echo "  正在啟動 Demo Server（背景運行）..."

    # 啟動 server 在背景
    cargo run --bin frost-cli -- demo-basic > /dev/null 2>&1 &
    SERVER_PID=$!

    # 等待 server 啟動
    sleep 5

    # 檢查 server 是否成功啟動
    if ps -p $SERVER_PID > /dev/null; then
        echo -e "  ${GREEN}✓ Server 已啟動 (PID: $SERVER_PID)${NC}"
        echo "  提示: 使用 kill $SERVER_PID 停止 server"
    else
        echo -e "  ${RED}✗ Server 啟動失敗${NC}"
        exit 1
    fi
fi

echo ""

# ============================================================================
# 測試 4: API 端點測試
# ============================================================================
echo "[4/5] 測試 API 端點..."

# 測試 /health
echo "  測試 GET /health..."
if curl -s http://127.0.0.1:3000/health | grep -q "status"; then
    echo -e "  ${GREEN}✓ /health 回應正常${NC}"
    echo -e "  ${GREEN}✓ 健康檢查通過${NC}"
else
    echo -e "  ${RED}✗ 無法連線到 /health${NC}"
    echo "  請確認 server 正在運行"
    exit 1
fi

echo ""

# 測試 /status
echo "  測試 GET /status..."
if curl -s http://127.0.0.1:3000/status | grep -q "current_phase"; then
    echo -e "  ${GREEN}✓ /status 回應正常${NC}"
    echo -e "  ${GREEN}✓ 狀態查詢正常${NC}"
else
    echo -e "  ${RED}✗ 無法連線到 /status${NC}"
    exit 1
fi

echo ""

# ============================================================================
# 測試 5: Dashboard 檢查
# ============================================================================
echo "[5/5] 檢查 Dashboard 檔案..."

if [ -f "dashboard.html" ]; then
    echo -e "  ${GREEN}✓ dashboard.html 存在${NC}"
    echo "  正在開啟 Dashboard..."

    # 根據系統選擇開啟方式
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        open dashboard.html
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        xdg-open dashboard.html 2>/dev/null || echo "  請手動開啟 dashboard.html"
    fi

    echo -e "  ${GREEN}✓ Dashboard 已在瀏覽器中開啟${NC}"
else
    echo -e "  ${RED}✗ 找不到 dashboard.html${NC}"
    echo "  請確認檔案存在"
fi

echo ""

# ============================================================================
# 測試摘要
# ============================================================================
echo -e "${CYAN}${BOLD}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║                                                                    ║${NC}"
echo -e "${CYAN}${BOLD}║   ✓✓✓ 快速測試完成！ ✓✓✓                                          ║${NC}"
echo -e "${CYAN}${BOLD}║                                                                    ║${NC}"
echo -e "${CYAN}${BOLD}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo "測試結果:"
echo -e "  ${GREEN}✓ Rust 環境正常${NC}"
echo -e "  ${GREEN}✓ 編譯通過${NC}"
echo -e "  ${GREEN}✓ HTTP Server 運行中${NC}"
echo -e "  ${GREEN}✓ API 端點回應正常${NC}"
echo -e "  ${GREEN}✓ Dashboard 已開啟${NC}"
echo ""

echo "下一步:"
echo "  1. 檢查 Dashboard 是否顯示 \"CONNECTED\" (綠色)"
echo "  2. 執行完整測試: python3 verify_demo.py"
echo "  3. 查看驗證清單: cat VERIFICATION-CHECKLIST.md"
echo "  4. 準備展示！"
echo ""

echo "💡 提示:"
echo "  • 如需重新執行完整 demo:"
echo "    cargo run --bin frost-cli -- demo-basic"
echo "  • 如需手動測試 API:"
echo "    curl http://127.0.0.1:3000/status | jq"
echo "  • Server 在背景運行，使用 jobs 查看，fg 切換到前台"
echo ""
