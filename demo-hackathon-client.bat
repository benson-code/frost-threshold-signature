@echo off
REM ============================================================================
REM FROST-T Hackathon Demo - Client 展示腳本（Surface Go 4 - Windows）
REM ============================================================================
REM
REM 用途：在 Surface Go 4 (Windows) 上透過 SSH 執行 FROST CLI 命令
REM 前提：
REM   1. Mac mini 已啟動 API 服務器
REM   2. Windows 已安裝 SSH 客戶端（Windows 10/11 內建）
REM   3. 已設定 SSH 免密碼登入或準備輸入密碼
REM
REM 使用方式：
REM   demo-hackathon-client.bat [MAC_IP]
REM
REM 範例：
REM   demo-hackathon-client.bat 192.168.1.100
REM
REM ============================================================================

setlocal enabledelayedexpansion

REM 檢查參數
if "%~1"=="" (
    echo Usage: %~nx0 ^<MAC_IP^>
    echo.
    echo Example:
    echo   %~nx0 192.168.1.100
    echo.
    echo Make sure Mac mini is running the API server first!
    pause
    exit /b 1
)

set MAC_IP=%~1
set MAC_USER=mac
set PROJECT_PATH=~/Documents/Prj/frost-threshold-signature

echo ════════════════════════════════════════════════════════════════
echo.
echo   FROST-T Hackathon Demo - Client Mode
echo   Surface Go 4 (Windows) -^> Mac mini SSH Remote Execution
echo.
echo ════════════════════════════════════════════════════════════════
echo.
echo Configuration:
echo   • Mac mini IP:  %MAC_IP%
echo   • SSH User:     %MAC_USER%
echo   • Project Path: %PROJECT_PATH%
echo.

REM 測試連接
echo Testing connection to Mac mini...
ssh -o ConnectTimeout=5 %MAC_USER%@%MAC_IP% "echo Connection successful" >nul 2>&1
if errorlevel 1 (
    echo ❌ Cannot connect to Mac mini
    echo.
    echo Troubleshooting:
    echo   1. Check Mac mini IP: ping %MAC_IP%
    echo   2. Test SSH: ssh %MAC_USER%@%MAC_IP%
    echo   3. Ensure Mac mini SSH service is running
    echo.
    pause
    exit /b 1
)

echo ✓ SSH connection successful
echo.
echo Starting FROST Demo...
echo ════════════════════════════════════════════════════════════════
echo.

REM 執行 CLI demo
echo Executing FROST CLI demo on Mac mini...
echo.

ssh -t %MAC_USER%@%MAC_IP% "cd %PROJECT_PATH% && cargo run --bin frost-cli -- demo-basic -m 'Hello Bitcoin++ Taipei 2025!'"

echo.
echo ════════════════════════════════════════════════════════════════
echo ✅ Demo completed!
echo.
echo View Dashboard in browser:
echo   http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000
echo.
echo Press any key to open Dashboard...
pause >nul

REM 開啟瀏覽器
start http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000
