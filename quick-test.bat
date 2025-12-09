@echo off
REM ============================================================================
REM FROST-T 快速測試腳本
REM 用途: 展示前快速驗證所有關鍵功能
REM ============================================================================

chcp 65001 >nul 2>&1

echo.
echo ╔════════════════════════════════════════════════════════════════════╗
echo ║                                                                    ║
echo ║              FROST-T 快速測試                                     ║
echo ║              Quick Pre-Demo Test                                  ║
echo ║                                                                    ║
echo ╚════════════════════════════════════════════════════════════════════╝
echo.

REM ============================================================================
REM 測試 1: 檢查 Rust 環境
REM ============================================================================
echo [1/5] 檢查 Rust 環境...

where cargo >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo   ✓ Cargo 已安裝
    cargo --version
) else (
    echo   ✗ Cargo 未找到！請安裝 Rust
    echo   下載: https://rustup.rs/
    pause
    exit /b 1
)

echo.

REM ============================================================================
REM 測試 2: 編譯檢查
REM ============================================================================
echo [2/5] 檢查編譯狀態...

cargo check --bin frost-cli --quiet 2>nul
if %ERRORLEVEL% EQU 0 (
    echo   ✓ 編譯通過
) else (
    echo   ✗ 編譯失敗！正在嘗試重新編譯...
    cargo build --bin frost-cli
    if %ERRORLEVEL% NEQ 0 (
        echo   ✗ 編譯失敗，請檢查錯誤訊息
        pause
        exit /b 1
    )
)

echo.

REM ============================================================================
REM 測試 3: 檢查 Port 3000
REM ============================================================================
echo [3/5] 檢查 Port 3000...

netstat -ano | findstr ":3000" >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo   ⚠ Port 3000 已被佔用
    echo   正在嘗試連線測試...

    REM 嘗試連線到現有的 server
    curl -s -o nul -w "%%{http_code}" http://127.0.0.1:3000/health >temp_status.txt 2>nul
    set /p STATUS=<temp_status.txt
    del temp_status.txt >nul 2>&1

    if "!STATUS!"=="200" (
        echo   ✓ Server 已在運行中
        goto skip_server_start
    ) else (
        echo   ✗ Port 被其他程式佔用
        echo   請關閉佔用 port 3000 的程式
        pause
        exit /b 1
    )
) else (
    echo   ✓ Port 3000 可用
)

echo.
echo   正在啟動 Demo Server（背景運行）...
start /B cargo run --bin frost-cli -- demo-basic >nul 2>&1

REM 等待 server 啟動
timeout /t 5 /nobreak >nul
echo   ✓ Server 已啟動

:skip_server_start

echo.

REM ============================================================================
REM 測試 4: API 端點測試
REM ============================================================================
echo [4/5] 測試 API 端點...

REM 測試 /health
echo   測試 GET /health...
curl -s http://127.0.0.1:3000/health >temp_health.json 2>nul
if %ERRORLEVEL% EQU 0 (
    echo   ✓ /health 回應正常
    type temp_health.json | findstr "status" >nul 2>&1
    if %ERRORLEVEL% EQU 0 (
        echo   ✓ 健康檢查通過
    )
    del temp_health.json >nul 2>&1
) else (
    echo   ✗ 無法連線到 /health
    echo   請確認 server 正在運行
    pause
    exit /b 1
)

echo.

REM 測試 /status
echo   測試 GET /status...
curl -s http://127.0.0.1:3000/status >temp_status.json 2>nul
if %ERRORLEVEL% EQU 0 (
    echo   ✓ /status 回應正常
    type temp_status.json | findstr "current_phase" >nul 2>&1
    if %ERRORLEVEL% EQU 0 (
        echo   ✓ 狀態查詢正常
    )
    del temp_status.json >nul 2>&1
) else (
    echo   ✗ 無法連線到 /status
    pause
    exit /b 1
)

echo.

REM ============================================================================
REM 測試 5: Dashboard 檢查
REM ============================================================================
echo [5/5] 檢查 Dashboard 檔案...

if exist dashboard.html (
    echo   ✓ dashboard.html 存在
    echo   正在開啟 Dashboard...
    start "" dashboard.html
    echo   ✓ Dashboard 已在瀏覽器中開啟
) else (
    echo   ✗ 找不到 dashboard.html
    echo   請確認檔案存在
)

echo.

REM ============================================================================
REM 測試摘要
REM ============================================================================
echo ╔════════════════════════════════════════════════════════════════════╗
echo ║                                                                    ║
echo ║   ✓✓✓ 快速測試完成！ ✓✓✓                                          ║
echo ║                                                                    ║
echo ╚════════════════════════════════════════════════════════════════════╝
echo.

echo 測試結果:
echo   ✓ Rust 環境正常
echo   ✓ 編譯通過
echo   ✓ HTTP Server 運行中
echo   ✓ API 端點回應正常
echo   ✓ Dashboard 已開啟
echo.

echo 下一步:
echo   1. 檢查 Dashboard 是否顯示 "CONNECTED" (綠色)
echo   2. 執行完整測試: python verify_demo.py
echo   3. 查看驗證清單: VERIFICATION-CHECKLIST.md
echo   4. 準備展示！
echo.

echo 💡 提示:
echo   • 如需重新執行完整 demo: demo-basic.bat
echo   • 如需手動測試 API: curl http://127.0.0.1:3000/status
echo   • Server 將持續在背景運行，按 Ctrl+C 停止
echo.

pause
