#!/bin/bash
# ============================================================================
# Mac mini 防火牆診斷和修復腳本
# ============================================================================
#
# 用途：檢查並修復 Mac mini 防火牆配置，允許外部訪問
#
# ============================================================================

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   Mac mini 防火牆診斷和修復                                    ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 獲取 Mac mini IP
MAC_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)
echo -e "${BLUE}🔍 Mac mini IP: ${GREEN}$MAC_IP${NC}"
echo ""

# 檢查 1: 檢查服務是否在正確的介面上監聽
echo "═══════════════════════════════════════════════════════════════"
echo "檢查 1: 服務綁定狀態"
echo "═══════════════════════════════════════════════════════════════"

echo -e "\nPort 3000 (API Server):"
lsof -i :3000 -n -P | grep LISTEN | while read line; do
    if echo "$line" | grep -q "\*:3000"; then
        echo -e "  ${GREEN}✓ 正確綁定到所有介面 (*:3000)${NC}"
    elif echo "$line" | grep -q "127.0.0.1:3000"; then
        echo -e "  ${RED}✗ 僅綁定到 localhost！${NC}"
        echo -e "  ${YELLOW}  需要修改為 0.0.0.0${NC}"
    fi
    echo "  $line"
done

echo -e "\nPort 8000 (Dashboard Server):"
lsof -i :8000 -n -P | grep LISTEN | while read line; do
    if echo "$line" | grep -q "\*:8000"; then
        echo -e "  ${GREEN}✓ 正確綁定到所有介面 (*:8000)${NC}"
    elif echo "$line" | grep -q "127.0.0.1:8000"; then
        echo -e "  ${RED}✗ 僅綁定到 localhost！${NC}"
        echo -e "  ${YELLOW}  需要重新啟動 Dashboard 服務器${NC}"
    fi
    echo "  $line"
done

echo ""

# 檢查 2: 測試本地訪問
echo "═══════════════════════════════════════════════════════════════"
echo "檢查 2: 本地訪問測試"
echo "═══════════════════════════════════════════════════════════════"

echo -e "\n測試 localhost:3000 ..."
if curl -sf -m 2 http://localhost:3000/health > /dev/null; then
    echo -e "${GREEN}✓ localhost:3000 可訪問${NC}"
else
    echo -e "${RED}✗ localhost:3000 無法訪問${NC}"
fi

echo -e "\n測試 $MAC_IP:3000 ..."
if curl -sf -m 2 http://$MAC_IP:3000/health > /dev/null; then
    echo -e "${GREEN}✓ $MAC_IP:3000 可訪問${NC}"
else
    echo -e "${RED}✗ $MAC_IP:3000 無法訪問${NC}"
fi

echo -e "\n測試 localhost:8000 ..."
if curl -sf -m 2 http://localhost:8000/ > /dev/null; then
    echo -e "${GREEN}✓ localhost:8000 可訪問${NC}"
else
    echo -e "${RED}✗ localhost:8000 無法訪問${NC}"
fi

echo -e "\n測試 $MAC_IP:8000 ..."
if curl -sf -m 2 http://$MAC_IP:8000/ > /dev/null; then
    echo -e "${GREEN}✓ $MAC_IP:8000 可訪問${NC}"
else
    echo -e "${RED}✗ $MAC_IP:8000 無法訪問${NC}"
fi

echo ""

# 檢查 3: 防火牆狀態（需要密碼）
echo "═══════════════════════════════════════════════════════════════"
echo "檢查 3: macOS 防火牆狀態"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo -e "${YELLOW}注意：以下命令需要管理員權限${NC}"
echo ""

# 嘗試檢查防火牆狀態
if /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate 2>/dev/null | grep -q "enabled"; then
    echo -e "${YELLOW}⚠️  防火牆已啟用${NC}"
    echo ""
    echo "這可能阻擋了外部訪問。建議："
    echo "  1. 系統偏好設定 → 安全性與隱私 → 防火牆"
    echo "  2. 點擊「防火牆選項」"
    echo "  3. 確保 Python 和 frost-threshold-signature 被允許"
    echo ""
    echo "或者暫時關閉防火牆進行測試："
    echo "  系統偏好設定 → 安全性與隱私 → 防火牆 → 關閉"
    echo ""
elif /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate 2>/dev/null | grep -q "disabled"; then
    echo -e "${GREEN}✓ 防火牆已關閉${NC}"
else
    echo -e "${BLUE}ℹ️  無法檢查防火牆狀態（需要 sudo）${NC}"
    echo ""
    echo "手動檢查方式："
    echo "  1. 打開「系統偏好設定」"
    echo "  2. 安全性與隱私 → 防火牆"
    echo "  3. 查看防火牆是否啟用"
fi

echo ""

# 檢查 4: 網路連接
echo "═══════════════════════════════════════════════════════════════"
echo "檢查 4: 網路介面資訊"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "所有網路介面的 IP 地址："
ifconfig | grep "inet " | grep -v 127.0.0.1 | while read line; do
    IP=$(echo "$line" | awk '{print $2}')
    echo "  • $IP"
done

echo ""

# 總結和建議
echo "═══════════════════════════════════════════════════════════════"
echo "診斷總結和建議"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo -e "${BLUE}📋 請在 Surface Go 4 上嘗試以下操作：${NC}"
echo ""
echo "1. 打開 PowerShell 或 CMD，執行："
echo "   ping $MAC_IP"
echo ""
echo "2. 如果 ping 成功，測試端口連接："
echo "   Test-NetConnection -ComputerName $MAC_IP -Port 3000"
echo "   Test-NetConnection -ComputerName $MAC_IP -Port 8000"
echo ""
echo "3. 如果端口測試失敗，問題可能是："
echo "   a) Mac mini 防火牆阻擋"
echo "   b) WiFi 路由器 AP 隔離（將設備隔離在不同子網）"
echo "   c) Windows 防火牆阻擋出站連接"
echo ""

echo -e "${YELLOW}🛠️  快速修復建議：${NC}"
echo ""
echo "方案 1: 暫時關閉 Mac mini 防火牆（最簡單）"
echo "  • 系統偏好設定 → 安全性與隱私 → 防火牆 → 關閉"
echo ""
echo "方案 2: 使用 USB 網路共享（100% 可靠）"
echo "  • Surface Go 4 USB 連接到 Mac mini"
echo "  • Mac mini: 系統偏好設定 → 共享 → 網際網路共享"
echo "  • 選擇 Wi-Fi 共享到 USB"
echo ""
echo "方案 3: 建立 Mac mini WiFi 熱點"
echo "  • 系統偏好設定 → 共享 → 網際網路共享"
echo "  • 共享來自：乙太網路"
echo "  • 使用以下連接埠共享：Wi-Fi"
echo "  • Surface Go 4 連接到 Mac mini 的熱點"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo ""
