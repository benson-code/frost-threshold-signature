# 部署指南 | Deployment Guide

## 🚀 部署到 GitHub | Deploy to GitHub

### 方法 1: 使用 GitHub CLI (推薦)

如果您安裝了 GitHub CLI (`gh`):

```bash
# 1. 登入 GitHub (如果還未登入)
gh auth login

# 2. 創建倉庫並推送
gh repo create frost-threshold-signature --public --source=. --remote=origin --push

# 完成！您的倉庫現在位於:
# https://github.com/benson-code/frost-threshold-signature
```

### 方法 2: 手動創建倉庫

如果沒有 GitHub CLI，請按照以下步驟：

#### 步驟 1: 在 GitHub 上創建新倉庫

1. 訪問 https://github.com/new
2. 填寫以下信息：
   - **Repository name**: `frost-threshold-signature`
   - **Description**: `Enterprise-grade Bitcoin-compatible Schnorr threshold signature service using FROST protocol`
   - **Visibility**: Public (或 Private，根據您的需求)
   - **不要勾選** "Initialize this repository with a README"（因為我們已經有了）

3. 點擊 "Create repository"

#### 步驟 2: 添加遠程倉庫並推送

在專案目錄中運行：

```bash
# 添加遠程倉庫
git remote add origin https://github.com/benson-code/frost-threshold-signature.git

# 推送代碼
git push -u origin main
```

如果遇到分支名稱問題（`master` vs `main`），運行：

```bash
# 重命名分支為 main
git branch -M main

# 再次推送
git push -u origin main
```

#### 步驟 3: 驗證部署

訪問您的倉庫：
```
https://github.com/benson-code/frost-threshold-signature
```

您應該看到：
- ✅ 完整的源代碼
- ✅ 雙語 README（中英文）
- ✅ MIT 授權
- ✅ 所有示例代碼

---

## 🔧 後續配置 | Post-Deployment Configuration

### 1. 設置 GitHub Topics

在倉庫頁面點擊 "⚙️ Settings" → "General" → "Topics"，添加：

```
rust, cryptography, bitcoin, frost, threshold-signature,
schnorr, secp256k1, axum, tokio, blockchain
```

### 2. 啟用 GitHub Pages (可選)

如果要創建專案網站：

1. Settings → Pages
2. Source: Deploy from a branch
3. Branch: `main` / `docs` (如果有)

### 3. 添加 GitHub Actions (可選)

創建 `.github/workflows/rust.yml` 用於自動化測試：

```yaml
name: Rust CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Build
      run: cargo build --release
    - name: Run tests
      run: cargo test
    - name: Check formatting
      run: cargo fmt -- --check
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

### 4. 保護主分支 (推薦)

Settings → Branches → Add rule:
- Branch name pattern: `main`
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging

---

## 📊 項目狀態徽章 | Status Badges

在 README.md 中添加更多徽章：

```markdown
[![Build Status](https://github.com/benson-code/frost-threshold-signature/workflows/Rust%20CI/badge.svg)](https://github.com/benson-code/frost-threshold-signature/actions)
[![codecov](https://codecov.io/gh/benson-code/frost-threshold-signature/branch/main/graph/badge.svg)](https://codecov.io/gh/benson-code/frost-threshold-signature)
```

---

## ✅ 部署檢查清單 | Deployment Checklist

- [x] Git 倉庫已初始化
- [x] 所有文件已提交
- [x] 雙語 README 已創建
- [x] LICENSE 文件已添加
- [x] .gitignore 已配置
- [ ] 遠程倉庫已創建
- [ ] 代碼已推送到 GitHub
- [ ] GitHub Topics 已設置
- [ ] Repository description 已填寫
- [ ] (可選) GitHub Actions 已配置
- [ ] (可選) 主分支保護已啟用

---

## 🎉 完成！

您的專案現在已經準備好展示給世界了！

**倉庫 URL**: https://github.com/benson-code/frost-threshold-signature

祝您面試順利！🚀
