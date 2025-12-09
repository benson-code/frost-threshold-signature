@echo off
echo 🔨 編譯 FROST CLI 工具...
echo.

cargo build --bin frost-cli

if %errorlevel% equ 0 (
    echo.
    echo ✅ 編譯成功！
    echo.
    echo 執行以下命令測試：
    echo   cargo run --bin frost-cli -- --help
) else (
    echo.
    echo ❌ 編譯失敗，請檢查錯誤訊息
)

pause
