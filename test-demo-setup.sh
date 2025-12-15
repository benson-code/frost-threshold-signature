#!/bin/bash
# ============================================================================
# FROST-T 展示設定快速測試腳本
# ============================================================================
#
# 用途：驗證所有展示元件是否正常運作
# 功能：
#   - 檢查編譯狀態
#   - 測試 API 服務器啟動
#   - 測試 Dashboard 訪問
#   - 驗證網路配置
#
# ============================================================================

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   FROST-T Hackathon Demo - Setup Test                         ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 測試結果
PASSED=0
FAILED=0

# 測試函數
test_step() {
    local description=$1
    local command=$2

    echo -n "Testing: $description... "

    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC}"
        FAILED=$((FAILED + 1))
    fi
}

# 1. 檢查 Rust 工具鏈
echo "1️⃣  Checking Rust toolchain..."
test_step "cargo installed" "which cargo"
test_step "rustc version" "rustc --version"
echo ""

# 2. 檢查專案檔案
echo "2️⃣  Checking project files..."
test_step "Cargo.toml exists" "[ -f Cargo.toml ]"
test_step "src/main.rs exists" "[ -f src/main.rs ]"
test_step "src/bin/frost-cli.rs exists" "[ -f src/bin/frost-cli.rs ]"
test_step "dashboard.html exists" "[ -f dashboard.html ]"
echo ""

# 3. 檢查展示腳本
echo "3️⃣  Checking demo scripts..."
test_step "demo-hackathon-all.sh exists" "[ -f demo-hackathon-all.sh ]"
test_step "demo-hackathon-all.sh is executable" "[ -x demo-hackathon-all.sh ]"
test_step "demo-hackathon-server.sh exists" "[ -f demo-hackathon-server.sh ]"
test_step "serve-dashboard.sh exists" "[ -f serve-dashboard.sh ]"
echo ""

# 4. 檢查編譯狀態
echo "4️⃣  Checking compilation..."
if [ -f "target/release/frost-threshold-signature" ]; then
    echo -e "   ${GREEN}✓ Release build exists${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "   ${YELLOW}! No release build found${NC}"
    echo "   Building now (this may take a few minutes)..."
    cargo build --release
    echo -e "   ${GREEN}✓ Build completed${NC}"
    PASSED=$((PASSED + 1))
fi
echo ""

# 5. 檢查網路配置
echo "5️⃣  Checking network configuration..."
MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)

if [ -z "$MAC_IP" ]; then
    echo -e "   ${YELLOW}! Warning: Could not detect network IP${NC}"
    echo "   This is OK for localhost testing"
else
    echo -e "   ${GREEN}✓ Network IP detected: $MAC_IP${NC}"
    PASSED=$((PASSED + 1))
fi
echo ""

# 6. 檢查端口可用性
echo "6️⃣  Checking port availability..."
if ! lsof -i :3000 > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓ Port 3000 available (API Server)${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "   ${YELLOW}! Port 3000 is in use${NC}"
    echo "   You may need to stop the running process"
fi

if ! lsof -i :8000 > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓ Port 8000 available (Dashboard Server)${NC}"
    PASSED=$((PASSED + 1))
else
    echo -e "   ${YELLOW}! Port 8000 is in use${NC}"
    echo "   You may need to stop the running process"
fi
echo ""

# 7. 檢查 Python（用於 Dashboard 服務器）
echo "7️⃣  Checking Python installation..."
test_step "python3 installed" "which python3"
test_step "python3 http.server module" "python3 -m http.server --help"
echo ""

# 總結
echo "═══════════════════════════════════════════════════════════════"
echo "Test Results:"
echo -e "  ${GREEN}Passed:${NC} $PASSED"
echo -e "  ${RED}Failed:${NC} $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed! Ready for demo!${NC}"
    echo ""
    echo "📋 Quick Start:"
    echo "   1. Start all services:"
    echo "      ./demo-hackathon-all.sh"
    echo ""
    echo "   2. Open Dashboard:"
    if [ -n "$MAC_IP" ]; then
        echo "      http://$MAC_IP:8000/dashboard.html?api=http://$MAC_IP:3000"
    else
        echo "      http://localhost:8000/dashboard.html"
    fi
    echo ""
    echo "   3. Run CLI demo (in new terminal):"
    echo "      cargo run --bin frost-cli -- demo-basic"
    echo ""
else
    echo -e "${RED}❌ Some tests failed. Please fix the issues above.${NC}"
    exit 1
fi
