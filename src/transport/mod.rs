//! # Transport 抽象層 - 訊息傳遞介面
//!
//! 這個模組定義了 FROST 協議中「訊息傳遞」的抽象介面。
//! 將通訊邏輯與協議邏輯分離，方便未來擴展不同的傳輸方式。
//!
//! ## 設計理念
//!
//! 在真實的門檻簽章場景中，參與者可能透過不同的方式通訊：
//! - **離線方式**：檔案、QR Code、USB
//! - **低頻寬無線**：LoRa、衛星通訊
//! - **網路方式**：HTTP、WebSocket、P2P
//!
//! Transport trait 提供統一介面，讓上層邏輯不需要關心底層如何傳輸。
//!
//! ## 模組結構
//!
//! - `Transport` trait：定義傳輸介面
//! - `StdoutTransport`：終端機輸出實作（用於展示）
//! - `SimulatedLoRaTransport`：模擬 LoRa 傳輸（延遲、掉包、分片）
//! - 未來擴展：
//!   - `FileTransport`：檔案系統傳輸
//!   - `HttpTransport`：HTTP API 傳輸

use serde::{Deserialize, Serialize};

// ============================================================================
// 子模組
// ============================================================================

pub mod simulated_lora;

// 重新匯出常用類型
pub use simulated_lora::{LoRaConfig, LoRaTransportState, SimulatedLoRaTransport, TransportEvent};

// ============================================================================
// 訊息結構定義
// ============================================================================

/// 傳輸訊息的元數據
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// 發送者 ID
    pub from: String,

    /// 接收者 ID（"coordinator" 或 "signer_N"）
    pub to: String,

    /// 訊息類型
    pub message_type: MessageType,

    /// 時間戳（可選）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// 訊息類型（用於分類和統計）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    /// Round 1: Signer -> Coordinator 的承諾
    Round1Commitment,

    /// Round 1.5: Coordinator -> Signers 的簽章套件
    SigningPackage,

    /// Round 2: Signer -> Coordinator 的簽章分片
    Round2SignatureShare,

    /// 最終簽章（Coordinator -> 廣播）
    FinalSignature,

    /// 其他訊息
    Other,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Round1Commitment => write!(f, "Round1Commitment"),
            MessageType::SigningPackage => write!(f, "SigningPackage"),
            MessageType::Round2SignatureShare => write!(f, "Round2SignatureShare"),
            MessageType::FinalSignature => write!(f, "FinalSignature"),
            MessageType::Other => write!(f, "Other"),
        }
    }
}

// ============================================================================
// Transport Trait - 核心抽象介面
// ============================================================================

/// 訊息傳輸的抽象介面
///
/// ## 設計考量
///
/// 為什麼設計成同步介面而不是異步？
/// - 方便實作最簡單的版本（Stdout、File）
/// - FROST 協議本身是「分輪次」的，不需要高並發傳輸
/// - 未來如果需要異步版本，可以再定義 `AsyncTransport` trait
///
/// ## 使用範例
///
/// ```no_run
/// use frost_threshold_signature::transport::{Transport, StdoutTransport, MessageType, MessageMetadata};
///
/// let mut transport = StdoutTransport::new();
///
/// transport.send(MessageMetadata {
///     from: "signer_1".to_string(),
///     to: "coordinator".to_string(),
///     message_type: MessageType::Round1Commitment,
///     timestamp: None,
/// }, "commitment_hex_data");
/// ```
pub trait Transport {
    /// 發送訊息
    ///
    /// # 參數
    /// - `metadata`: 訊息元數據（發送者、接收者、類型等）
    /// - `payload`: 實際的訊息內容（通常是 hex 編碼的密碼學數據）
    fn send(&mut self, metadata: MessageMetadata, payload: &str);

    /// 獲取傳輸統計資訊（可選實作）
    ///
    /// 預設實作返回 None，表示不支援統計。
    /// 子類別可以 override 這個方法來提供統計資訊。
    fn get_stats(&self) -> Option<TransportStats> {
        None
    }

    /// 重置傳輸狀態（可選實作）
    ///
    /// 用於清除緩衝區、重置計數器等。
    fn reset(&mut self) {
        // 預設不做任何事
    }
}

/// 傳輸統計資訊
#[derive(Debug, Clone, Default)]
pub struct TransportStats {
    /// 總發送訊息數
    pub total_messages: usize,

    /// 按類型統計
    pub by_type: std::collections::HashMap<MessageType, usize>,

    /// 總發送位元組數
    pub total_bytes: usize,
}

// ============================================================================
// StdoutTransport - 終端機輸出實作
// ============================================================================

/// 終端機輸出的 Transport 實作
///
/// 這是最簡單的實作，將所有訊息印到 stdout。
/// 主要用於：
/// - Demo 展示
/// - 除錯
/// - 教學用途
///
/// ## 輸出格式
///
/// ```text
/// [Round1Commitment] signer_1 → coordinator
///   Payload: commitment_hex_data...
/// ```
pub struct StdoutTransport {
    /// 是否顯示完整的 payload（否則只顯示前 32 字元）
    show_full_payload: bool,

    /// 統計資訊
    stats: TransportStats,
}

impl StdoutTransport {
    /// 建立新的 StdoutTransport
    ///
    /// 預設只顯示 payload 的前 32 字元。
    pub fn new() -> Self {
        Self {
            show_full_payload: false,
            stats: TransportStats::default(),
        }
    }

    /// 建立顯示完整 payload 的 StdoutTransport
    pub fn new_full() -> Self {
        Self {
            show_full_payload: true,
            stats: TransportStats::default(),
        }
    }

    /// 設定是否顯示完整 payload
    pub fn set_show_full_payload(&mut self, show_full: bool) {
        self.show_full_payload = show_full;
    }
}

impl Default for StdoutTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for StdoutTransport {
    fn send(&mut self, metadata: MessageMetadata, payload: &str) {
        // 更新統計
        self.stats.total_messages += 1;
        self.stats.total_bytes += payload.len();
        *self.stats.by_type.entry(metadata.message_type).or_insert(0) += 1;

        // 格式化輸出
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📡 [{}] {} → {}",
            metadata.message_type,
            metadata.from,
            metadata.to
        );

        if let Some(timestamp) = metadata.timestamp {
            println!("   ⏰ {}", timestamp.format("%H:%M:%S%.3f"));
        }

        // 顯示 payload
        if self.show_full_payload {
            println!("   📦 Payload ({} bytes):", payload.len());
            println!("      {}", payload);
        } else {
            let preview = if payload.len() > 64 {
                format!("{}... ({} bytes total)", &payload[..64], payload.len())
            } else {
                payload.to_string()
            };
            println!("   📦 Payload: {}", preview);
        }

        println!();
    }

    fn get_stats(&self) -> Option<TransportStats> {
        Some(self.stats.clone())
    }

    fn reset(&mut self) {
        self.stats = TransportStats::default();
    }
}

// ============================================================================
// 測試
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdout_transport_basic() {
        let mut transport = StdoutTransport::new();

        transport.send(
            MessageMetadata {
                from: "signer_1".to_string(),
                to: "coordinator".to_string(),
                message_type: MessageType::Round1Commitment,
                timestamp: Some(chrono::Utc::now()),
            },
            "deadbeef",
        );

        let stats = transport.get_stats().unwrap();
        assert_eq!(stats.total_messages, 1);
        assert_eq!(stats.by_type.get(&MessageType::Round1Commitment), Some(&1));
    }
}
