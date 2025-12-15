@echo off
REM ============================================================================
REM FROST-T Tailscale Client - Surface Go 4 展示腳本
REM ============================================================================
REM
REM 用途：透過 Tailscale VPN 連接 Mac mini 並執行 FROST CLI demo
REM 前提：Mac mini 和 Surface Go 4 都已連接到 Tailscale
REM
REM ============================================================================

set MAC_TAILSCALE_IP=100.110.164.70
set MAC_USER=mac
set PROJECT_PATH=~/Documents/Prj/frost-threshold-signature

echo ╔════════════════════════════════════════════════════════════════╗
echo ║   FROST-T Tailscale Demo - Remote Execution                   ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo Mac mini Tailscale IP: %MAC_TAILSCALE_IP%
echo SSH User: %MAC_USER%
echo.

REM 測試 Tailscale 連接
echo Testing Tailscale connection...
ping -n 2 %MAC_TAILSCALE_IP% >nul 2>&1
if errorlevel 1 (
    echo ❌ Cannot reach Mac mini via Tailscale
    echo.
    echo Troubleshooting:
    echo   1. Ensure Tailscale is running on both devices
    echo   2. Check: tailscale status
    echo   3. Verify both devices logged into same account
    echo.
    pause
    exit /b 1
)

echo ✅ Tailscale connection successful!
echo.

REM 測試 SSH
echo Testing SSH connection...
ssh -o ConnectTimeout=5 %MAC_USER%@%MAC_TAILSCALE_IP% "echo SSH OK" >nul 2>&1
if errorlevel 1 (
    echo ❌ Cannot SSH to Mac mini
    echo.
    echo Setup SSH access:
    echo   1. On Mac mini: System Preferences → Sharing → Remote Login
    echo   2. On Surface Go 4: ssh-keygen (if not done)
    echo   3. Copy key: ssh-copy-id %MAC_USER%@%MAC_TAILSCALE_IP%
    echo.
    pause
    exit /b 1
)

echo ✅ SSH connection successful!
echo.

REM 選擇操作
echo What would you like to do?
echo.
echo   1. Run basic FROST demo
echo   2. Run demo with custom message
echo   3. Run demo with custom signers
echo   4. Check service status
echo   5. View logs
echo.
set /p CHOICE="Enter choice (1-5): "

if "%CHOICE%"=="1" goto basic_demo
if "%CHOICE%"=="2" goto custom_message
if "%CHOICE%"=="3" goto custom_signers
if "%CHOICE%"=="4" goto check_status
if "%CHOICE%"=="5" goto view_logs

echo Invalid choice!
pause
exit /b 1

:basic_demo
echo.
echo Running basic FROST demo...
echo ════════════════════════════════════════════════════════════════
echo.
ssh -t %MAC_USER%@%MAC_TAILSCALE_IP% "cd %PROJECT_PATH% && cargo run --bin frost-cli -- demo-basic -m 'Bitcoin++ Taipei 2025 via Tailscale!'"
goto end

:custom_message
echo.
set /p MESSAGE="Enter your message: "
echo.
echo Running demo with message: %MESSAGE%
echo ════════════════════════════════════════════════════════════════
echo.
ssh -t %MAC_USER%@%MAC_TAILSCALE_IP% "cd %PROJECT_PATH% && cargo run --bin frost-cli -- demo-basic -m '%MESSAGE%'"
goto end

:custom_signers
echo.
echo Available signers: 1, 2, 3, 4, 5 (need 3)
set /p SIGNERS="Enter signers (e.g., 1,2,3): "
echo.
echo Running demo with signers: %SIGNERS%
echo ════════════════════════════════════════════════════════════════
echo.
ssh -t %MAC_USER%@%MAC_TAILSCALE_IP% "cd %PROJECT_PATH% && cargo run --bin frost-cli -- demo-basic --signers %SIGNERS%"
goto end

:check_status
echo.
echo Checking service status...
echo ════════════════════════════════════════════════════════════════
echo.
ssh -t %MAC_USER%@%MAC_TAILSCALE_IP% "cd %PROJECT_PATH% && lsof -i :3000 -i :8000"
goto end

:view_logs
echo.
echo Viewing logs...
echo ════════════════════════════════════════════════════════════════
echo.
ssh -t %MAC_USER%@%MAC_TAILSCALE_IP% "tail -50 /tmp/frost-demo/api.log"
goto end

:end
echo.
echo ════════════════════════════════════════════════════════════════
echo ✅ Done!
echo.
echo View Dashboard:
echo   http://%MAC_TAILSCALE_IP%:8000/dashboard.html?api=http://%MAC_TAILSCALE_IP%:3000
echo.
echo Press any key to open Dashboard...
pause >nul

REM 開啟 Dashboard
start http://%MAC_TAILSCALE_IP%:8000/dashboard.html?api=http://%MAC_TAILSCALE_IP%:3000
