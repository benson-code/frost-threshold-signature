@echo off
REM ============================================================================
REM FROST-T Dashboard - Tailscale 快捷方式
REM ============================================================================
REM
REM 用途：在 Surface Go 4 上透過 Tailscale VPN 開啟 Dashboard
REM 前提：Mac mini 和 Surface Go 4 都已連接到 Tailscale
REM
REM ============================================================================

REM Mac mini 的 Tailscale IP
set MAC_TAILSCALE_IP=100.110.164.70

echo ╔════════════════════════════════════════════════════════════════╗
echo ║   FROST-T Dashboard - Tailscale Connection                    ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo Mac mini Tailscale IP: %MAC_TAILSCALE_IP%
echo.
echo Testing Tailscale connection...
ping -n 2 %MAC_TAILSCALE_IP% >nul 2>&1

if errorlevel 1 (
    echo ❌ Cannot reach Mac mini via Tailscale
    echo.
    echo Troubleshooting:
    echo   1. Check if Tailscale is running on both devices
    echo   2. Ensure both devices are logged into same Tailscale account
    echo   3. Run: tailscale status
    echo.
    pause
    exit /b 1
)

echo ✅ Tailscale connection successful!
echo.
echo Testing API...
curl -s -m 3 http://%MAC_TAILSCALE_IP%:3000/health >nul 2>&1

if errorlevel 1 (
    echo ⚠️  API might not be running on Mac mini
    echo.
    echo Please ensure services are running:
    echo   SSH to Mac mini and run: ./demo-hackathon-all.sh
    echo.
) else (
    echo ✅ API is responding!
)

echo.
echo Opening Dashboard in browser...
echo.

REM 開啟 Dashboard
start http://%MAC_TAILSCALE_IP%:8000/dashboard.html?api=http://%MAC_TAILSCALE_IP%:3000

echo Dashboard URL:
echo   http://%MAC_TAILSCALE_IP%:8000/dashboard.html?api=http://%MAC_TAILSCALE_IP%:3000
echo.
echo Quick Links:
echo   Homepage:  http://%MAC_TAILSCALE_IP%:8000
echo   API Test:  http://%MAC_TAILSCALE_IP%:3000/health
echo.
echo ✅ Done! Press any key to exit.
pause >nul
