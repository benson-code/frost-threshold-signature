@echo off
echo ╔════════════════════════════════════════════════════════════════╗
echo ║                                                                ║
echo ║   FROST-T Phase 2 Demo Launcher                               ║
echo ║   SimulatedLoRaTransport + Cyberpunk Dashboard                ║
echo ║                                                                ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo 🚀 準備啟動...
echo.
echo 📋 這個 Demo 會做什麼：
echo   1. 啟動 HTTP Server (port 3000)
echo   2. 執行完整的 3-of-5 FROST 簽章流程
echo   3. 使用 SimulatedLoRaTransport 模擬無線傳輸
echo   4. 即時更新 Dashboard 狀態
echo.
echo 🌐 接下來會自動開啟 Dashboard...
timeout /t 2 >nul
echo.

REM 在背景開啟 Dashboard
start "" dashboard.html

echo ✓ Dashboard 已在瀏覽器中開啟
echo.
echo 🎬 開始執行 FROST 流程...
echo   （CLI 會顯示詳細過程，Dashboard 會即時更新）
echo.
timeout /t 2 >nul

REM 執行 FROST Demo
cargo run --bin frost-cli -- demo-basic

echo.
echo.
echo ╔════════════════════════════════════════════════════════════════╗
echo ║  Demo 完成！                                                   ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo 💡 其他測試選項：
echo   • 不同的訊息：
echo     cargo run --bin frost-cli -- demo-basic -m "bitcoin++"
echo.
echo   • 不同的簽署者組合：
echo     cargo run --bin frost-cli -- demo-basic --signers 1,3,5
echo.
echo   • 顯示完整 payload：
echo     cargo run --bin frost-cli -- demo-basic --full-payload
echo.

pause
