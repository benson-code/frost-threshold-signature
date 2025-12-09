@echo off
echo ========================================
echo  FROST GitHub 部署腳本
echo ========================================
echo.

REM 設置 GitHub CLI 路徑
set GH="C:\Program Files\GitHub CLI\gh.exe"

echo 步驟 1: 登入 GitHub
echo ----------------------------------------
%GH% auth login --web
if errorlevel 1 (
    echo 登入失敗，請檢查網路連接
    pause
    exit /b 1
)
echo.

echo 步驟 2: 創建 GitHub 倉庫
echo ----------------------------------------
%GH% repo create frost-threshold-signature --public --description "Enterprise-grade Bitcoin-compatible Schnorr threshold signature service using FROST protocol" --source=. --remote=origin --push
if errorlevel 1 (
    echo 倉庫創建失敗
    pause
    exit /b 1
)
echo.

echo ========================================
echo  部署完成！
echo ========================================
echo.
echo 你的倉庫已成功創建：
echo https://github.com/benson-code/frost-threshold-signature
echo.
pause
