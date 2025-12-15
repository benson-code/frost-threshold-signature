# ============================================================================
# FROST-T Docker Image
# ============================================================================
#
# 多階段構建：
# - Stage 1 (builder): 編譯 Rust 應用
# - Stage 2 (runtime): 輕量化運行環境
#
# ============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Builder - 編譯 Rust 應用
# -----------------------------------------------------------------------------
FROM rust:1.75-slim as builder

# 安裝構建依賴
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 設定工作目錄
WORKDIR /build

# 複製依賴清單（利用 Docker layer cache）
COPY Cargo.toml Cargo.lock ./

# 複製源代碼
COPY src ./src

# 編譯 Release 版本
RUN cargo build --release

# -----------------------------------------------------------------------------
# Stage 2: Runtime - 輕量化運行環境
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim

# 安裝運行時依賴
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 創建非 root 用戶
RUN useradd -m -u 1000 frost && \
    mkdir -p /app && \
    chown -R frost:frost /app

# 設定工作目錄
WORKDIR /app

# 從 builder 複製編譯好的二進制文件
COPY --from=builder /build/target/release/frost-threshold-signature /app/
COPY --from=builder /build/target/release/frost-cli /app/

# 切換到非 root 用戶
USER frost

# 暴露 API 端口
EXPOSE 3000

# 設定環境變數
ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info

# 健康檢查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# 啟動 API 服務器
CMD ["/app/frost-threshold-signature"]
