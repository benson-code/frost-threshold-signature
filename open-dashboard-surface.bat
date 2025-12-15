@echo off
REM ============================================================================
REM Surface Go 4 - 開啟 Dashboard 快捷方式
REM ============================================================================
REM
REM 用途：在 Surface Go 4 上一鍵開啟正確的 Dashboard URL
REM 使用：將此文件複製到 Surface Go 4，雙擊執行
REM
REM ============================================================================

REM 設定 Mac mini IP（請根據實際情況修改）
set MAC_IP=192.168.68.51

echo ╔════════════════════════════════════════════════════════════════╗
echo ║   Opening FROST-T Dashboard on Surface Go 4                   ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo Mac mini IP: %MAC_IP%
echo.
echo Testing connection...
ping -n 1 %MAC_IP% >nul 2>&1

if errorlevel 1 (
    echo ❌ Cannot reach Mac mini at %MAC_IP%
    echo.
    echo Troubleshooting:
    echo   1. Check if Mac mini and Surface Go 4 are on same WiFi
    echo   2. Verify Mac mini IP address
    echo   3. Ensure services are running on Mac mini
    echo.
    pause
    exit /b 1
)

echo ✅ Connection successful!
echo.
echo Opening Dashboard in browser...
echo.

REM 開啟 Dashboard
start http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000

echo Dashboard URL:
echo   http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000
echo.
echo ✅ Done! Press any key to exit.
pause >nul
