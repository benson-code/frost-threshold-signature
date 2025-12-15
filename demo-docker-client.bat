@echo off
REM ============================================================================
REM FROST-T Docker Client - Surface Go 4 展示腳本 (Windows)
REM ============================================================================
REM
REM 用途：在 Surface Go 4 (Windows) 上透過 SSH 控制 Mac mini Docker
REM 優點：Windows 完全不需要安裝任何東西（SSH 是 Windows 10/11 內建）
REM
REM 使用方式：
REM   demo-docker-client.bat [MAC_IP] [COMMAND]
REM
REM 範例：
REM   demo-docker-client.bat 192.168.1.100 demo
REM   demo-docker-client.bat 192.168.1.100 status
REM
REM ============================================================================

setlocal enabledelayedexpansion

REM 檢查參數
if "%~1"=="" (
    echo Usage: %~nx0 ^<MAC_IP^> [COMMAND]
    echo.
    echo Commands:
    echo   demo    - Run CLI demo ^(default^)
    echo   status  - Check service status
    echo   logs    - View logs
    echo   custom  - Run custom CLI command
    echo.
    echo Example:
    echo   %~nx0 192.168.1.100 demo
    echo.
    pause
    exit /b 1
)

set MAC_IP=%~1
set MAC_USER=mac
set PROJECT_PATH=~/Documents/Prj/frost-threshold-signature
set COMMAND=%~2
if "%COMMAND%"=="" set COMMAND=demo

echo ════════════════════════════════════════════════════════════════
echo    FROST-T Docker Client - Surface Go 4 (Windows)
echo ════════════════════════════════════════════════════════════════
echo.
echo Configuration:
echo   • Mac mini IP:  %MAC_IP%
echo   • SSH User:     %MAC_USER%
echo   • Command:      %COMMAND%
echo.

REM 測試連接
echo Testing connection to Mac mini...
ssh -o ConnectTimeout=5 %MAC_USER%@%MAC_IP% "echo Connection successful" >nul 2>&1
if errorlevel 1 (
    echo ❌ Cannot connect to Mac mini
    echo.
    echo Troubleshooting:
    echo   1. Check network: ping %MAC_IP%
    echo   2. Test SSH: ssh %MAC_USER%@%MAC_IP%
    echo   3. Ensure Docker services are running on Mac mini
    echo.
    pause
    exit /b 1
)

echo ✓ Connection successful
echo.

REM 執行命令
if "%COMMAND%"=="demo" goto :run_demo
if "%COMMAND%"=="status" goto :check_status
if "%COMMAND%"=="logs" goto :view_logs
if "%COMMAND%"=="custom" goto :custom_command

echo ❌ Unknown command: %COMMAND%
echo.
echo Available commands: demo, status, logs, custom
pause
exit /b 1

:run_demo
echo Running FROST CLI demo in Docker container...
echo ════════════════════════════════════════════════════════════════
echo.

ssh -t %MAC_USER%@%MAC_IP% "cd %PROJECT_PATH% && docker exec -it frost-api /app/frost-cli demo-basic -m 'Bitcoin++ Taipei 2025!'"

echo.
echo ════════════════════════════════════════════════════════════════
echo ✅ Demo completed!
goto :show_dashboard

:check_status
echo Checking service status...
echo.

ssh -t %MAC_USER%@%MAC_IP% "cd %PROJECT_PATH% && ./demo-docker.sh status"
goto :end

:view_logs
echo Viewing logs (Press Ctrl+C to exit)...
echo.

ssh -t %MAC_USER%@%MAC_IP% "cd %PROJECT_PATH% && ./demo-docker.sh logs"
goto :end

:custom_command
echo Custom CLI command mode
echo Enter your CLI command:
echo.

set /p CLI_ARGS="> frost-cli "

if not "%CLI_ARGS%"=="exit" (
    ssh -t %MAC_USER%@%MAC_IP% "cd %PROJECT_PATH% && docker exec -it frost-api /app/frost-cli %CLI_ARGS%"
)
goto :end

:show_dashboard
echo.
echo View Dashboard in browser:
echo   http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000
echo.
echo Press any key to open Dashboard...
pause >nul

REM 開啟瀏覽器
start http://%MAC_IP%:8000/dashboard.html?api=http://%MAC_IP%:3000

:end
echo.
