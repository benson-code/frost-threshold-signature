@echo off
REM ============================================================================
REM Surface Go 4 診斷測試腳本
REM ============================================================================
REM
REM 用途：在 Surface Go 4 上測試與 Mac mini 的連接
REM 使用：將此文件複製到 Surface Go 4 並執行
REM
REM ============================================================================

set MAC_IP=192.168.68.51

echo ╔════════════════════════════════════════════════════════════════╗
echo ║   Surface Go 4 Network Diagnostics                            ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.
echo Target Mac mini IP: %MAC_IP%
echo.
echo ═══════════════════════════════════════════════════════════════
echo Test 1: Ping Mac mini
echo ═══════════════════════════════════════════════════════════════
ping -n 4 %MAC_IP%

echo.
echo ═══════════════════════════════════════════════════════════════
echo Test 2: Test API Port 3000
echo ═══════════════════════════════════════════════════════════════
curl -v -m 5 http://%MAC_IP%:3000/health 2>&1
if errorlevel 1 (
    echo ❌ Cannot connect to API port 3000
) else (
    echo ✅ API port 3000 is accessible
)

echo.
echo ═══════════════════════════════════════════════════════════════
echo Test 3: Test Dashboard Port 8000
echo ═══════════════════════════════════════════════════════════════
curl -v -m 5 http://%MAC_IP%:8000/dashboard.html 2>&1 | findstr /C:"200 OK" /C:"Connection refused" /C:"timed out"

echo.
echo ═══════════════════════════════════════════════════════════════
echo Test 4: DNS Resolution
echo ═══════════════════════════════════════════════════════════════
nslookup %MAC_IP%

echo.
echo ═══════════════════════════════════════════════════════════════
echo Test 5: Route Trace
echo ═══════════════════════════════════════════════════════════════
tracert -d -h 5 %MAC_IP%

echo.
echo ═══════════════════════════════════════════════════════════════
echo Summary
echo ═══════════════════════════════════════════════════════════════
echo.
echo If all tests pass but browser still doesn't work:
echo   1. Try different browser (Chrome, Firefox)
echo   2. Clear browser cache (Ctrl+Shift+Delete)
echo   3. Try Incognito/Private mode
echo   4. Check browser console (F12) for errors
echo.
echo If tests fail:
echo   1. Ensure both devices on same WiFi
echo   2. Check Mac mini firewall settings
echo   3. Restart WiFi router
echo.
pause
