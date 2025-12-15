//! # Simulated LoRa Transport - 虛擬 LoRa 傳輸層
//!
//! 模擬低頻寬、高延遲、可能掉包的無線傳輸環境。
//! 用於展示 FROST 協議在惡劣網路環境下的強健性。
//!
//! ## 特性
//!
//! - **延遲模擬**：每個封包傳輸有固定延遲（例如 500ms）
//! - **掉包重傳**：模擬封包遺失（例如 10% 機率），並自動重傳
//! - **封包分片**：大型訊息切割成小片段（例如 64 bytes）
//! - **狀態追蹤**：記錄所有傳輸事件，供 Dashboard 查詢

use super::{MessageMetadata, MessageType, Transport, TransportStats};
use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================================
// LoRa 傳輸配置
// ============================================================================

/// LoRa 傳輸配置參數
#[derive(Debug, Clone)]
pub struct LoRaConfig {
    /// 每個封包的延遲（毫秒）
    pub latency_ms: u64,

    /// 封包遺失率（0.0 ~ 1.0）
    pub packet_loss_rate: f64,

    /// 分片大小（bytes）- LoRa 典型的 payload 限制
    pub fragment_size: usize,

    /// 最大重傳次數
    pub max_retries: u32,
}

impl Default for LoRaConfig {
    fn default() -> Self {
        Self {
            latency_ms: 500,        // 500ms 延遲（模擬遠距離傳輸）
            packet_loss_rate: 0.1,  // 10% 掉包率
            fragment_size: 64,      // 64 bytes per fragment（LoRa SF7 典型值）
            max_retries: 3,         // 最多重傳 3 次
        }
    }
}

// ============================================================================
// 傳輸事件（供 Dashboard 查詢）
// ============================================================================

/// 傳輸事件類型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransportEvent {
    /// 開始傳輸
    TransmitStart {
        from: String,
        to: String,
        message_type: MessageType,
        total_bytes: usize,
        fragments: usize,
    },

    /// 傳輸片段
    TransmitFragment {
        fragment_id: usize,
        total_fragments: usize,
        bytes: usize,
    },

    /// 封包遺失
    PacketLost {
        fragment_id: usize,
        retry_count: u32,
    },

    /// 重傳成功
    RetrySuccess {
        fragment_id: usize,
        retry_count: u32,
    },

    /// 傳輸完成
    TransmitComplete {
        total_time_ms: u64,
        retries: u32,
    },
}

/// LoRa 傳輸狀態（共享給 HTTP API）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoRaTransportState {
    /// 當前階段
    pub current_phase: String,

    /// 總訊息數
    pub total_messages: usize,

    /// 總位元組數
    pub total_bytes: usize,

    /// 當前傳輸進度（0.0 ~ 1.0）
    pub progress: f64,

    /// 虛擬訊號強度 (RSSI, -120 ~ -30 dBm)
    pub rssi: i32,

    /// 最近的傳輸事件（最多保留 100 條）
    pub recent_events: Vec<TransportEvent>,

    /// 按訊息類型統計
    pub by_type: HashMap<String, usize>,

    /// 總重傳次數
    pub total_retries: u32,

    /// CLI 輸出日誌（最多保留 500 行）
    pub cli_output: Vec<String>,
}

impl Default for LoRaTransportState {
    fn default() -> Self {
        Self {
            current_phase: "Idle".to_string(),
            total_messages: 0,
            total_bytes: 0,
            progress: 0.0,
            rssi: -80, // 初始訊號強度
            recent_events: Vec::new(),
            by_type: HashMap::new(),
            total_retries: 0,
            cli_output: Vec::new(),
        }
    }
}

// ============================================================================
// Simulated LoRa Transport
// ============================================================================

/// 虛擬 LoRa 傳輸層
///
/// 模擬真實的 LoRa 無線傳輸環境，包括：
/// - 低頻寬（每個封包 64 bytes）
/// - 高延遲（500ms per packet）
/// - 封包遺失與重傳
/// - 即時狀態追蹤
pub struct SimulatedLoRaTransport {
    /// LoRa 配置
    config: LoRaConfig,

    /// 共享狀態（供 HTTP API 讀取）
    state: Arc<Mutex<LoRaTransportState>>,

    /// 隨機數生成器（用於模擬掉包）
    rng: rand::rngs::ThreadRng,

    /// 累計統計
    stats: TransportStats,
}

impl SimulatedLoRaTransport {
    /// 建立新的 LoRa Transport（使用預設配置）
    pub fn new() -> Self {
        Self::new_with_config(LoRaConfig::default())
    }

    /// 建立新的 LoRa Transport（自訂配置）
    pub fn new_with_config(config: LoRaConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(LoRaTransportState::default())),
            rng: rand::thread_rng(),
            stats: TransportStats::default(),
        }
    }

    /// 獲取共享狀態（供 HTTP API 使用）
    pub fn get_state(&self) -> Arc<Mutex<LoRaTransportState>> {
        Arc::clone(&self.state)
    }

    /// 記錄 CLI 輸出（供 Dashboard 顯示）
    pub fn log_cli_output(&self, line: String) {
        let mut state = self.state.lock().unwrap();
        state.cli_output.push(line);

        // 保持最近 500 行
        if state.cli_output.len() > 500 {
            state.cli_output.remove(0);
        }
    }

    /// 模擬封包傳輸（包含延遲和可能的掉包）
    fn transmit_fragment(&mut self, fragment_id: usize, total_fragments: usize, data: &[u8]) -> bool {
        // 模擬傳輸延遲
        thread::sleep(Duration::from_millis(self.config.latency_ms));

        // 模擬掉包
        let lost = self.rng.gen::<f64>() < self.config.packet_loss_rate;

        // 更新狀態
        let mut state = self.state.lock().unwrap();

        if lost {
            state.recent_events.push(TransportEvent::PacketLost {
                fragment_id,
                retry_count: 0,
            });

            // 保持最近 100 條事件
            if state.recent_events.len() > 100 {
                state.recent_events.remove(0);
            }

            // 訊號強度下降
            state.rssi = (state.rssi - 5).max(-120);
        } else {
            state.recent_events.push(TransportEvent::TransmitFragment {
                fragment_id,
                total_fragments,
                bytes: data.len(),
            });

            if state.recent_events.len() > 100 {
                state.recent_events.remove(0);
            }

            // 訊號強度略微改善
            state.rssi = (state.rssi + 2).min(-50);
        }

        // 更新進度
        state.progress = (fragment_id + 1) as f64 / total_fragments as f64;

        !lost
    }

    /// 傳輸一個完整的訊息（包含分片和重傳）
    fn transmit_with_fragmentation(&mut self, metadata: &MessageMetadata, payload: &str) {
        let payload_bytes = payload.as_bytes();
        let total_bytes = payload_bytes.len();

        // 計算需要的片段數
        let total_fragments = (total_bytes + self.config.fragment_size - 1) / self.config.fragment_size;

        // 更新狀態：開始傳輸
        {
            let mut state = self.state.lock().unwrap();
            state.current_phase = format!("{:?}", metadata.message_type);
            state.recent_events.push(TransportEvent::TransmitStart {
                from: metadata.from.clone(),
                to: metadata.to.clone(),
                message_type: metadata.message_type,
                total_bytes,
                fragments: total_fragments,
            });

            if state.recent_events.len() > 100 {
                state.recent_events.remove(0);
            }
        }

        let start_time = std::time::Instant::now();
        let mut total_retries = 0u32;

        // 傳輸每個片段
        for i in 0..total_fragments {
            let start = i * self.config.fragment_size;
            let end = ((i + 1) * self.config.fragment_size).min(total_bytes);
            let fragment = &payload_bytes[start..end];

            // 嘗試傳輸（包含重傳）
            let mut retry_count = 0u32;
            loop {
                print!("  📡 Fragment {}/{} ({} bytes)... ", i + 1, total_fragments, fragment.len());

                if self.transmit_fragment(i, total_fragments, fragment) {
                    println!("✓");
                    break;
                } else {
                    println!("✗ (掉包)");

                    retry_count += 1;
                    total_retries += 1;

                    if retry_count >= self.config.max_retries {
                        println!("     ❌ 超過最大重傳次數，放棄此片段");
                        break;
                    }

                    println!("     🔄 重傳 {}/{}...", retry_count, self.config.max_retries);

                    // 記錄重傳事件
                    let mut state = self.state.lock().unwrap();
                    state.recent_events.push(TransportEvent::PacketLost {
                        fragment_id: i,
                        retry_count,
                    });

                    if state.recent_events.len() > 100 {
                        state.recent_events.remove(0);
                    }

                    // 重傳前稍微等待
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }

        let total_time_ms = start_time.elapsed().as_millis() as u64;

        // 更新狀態：傳輸完成
        {
            let mut state = self.state.lock().unwrap();
            state.total_retries += total_retries;
            state.recent_events.push(TransportEvent::TransmitComplete {
                total_time_ms,
                retries: total_retries,
            });

            if state.recent_events.len() > 100 {
                state.recent_events.remove(0);
            }
        }

        if total_retries > 0 {
            println!("  ⚠️  傳輸完成（總重傳次數：{}，總耗時：{}ms）", total_retries, total_time_ms);
        } else {
            println!("  ✓ 傳輸完成（無掉包，總耗時：{}ms）", total_time_ms);
        }
    }
}

impl Default for SimulatedLoRaTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for SimulatedLoRaTransport {
    fn send(&mut self, metadata: MessageMetadata, payload: &str) {
        // 更新統計
        self.stats.total_messages += 1;
        self.stats.total_bytes += payload.len();
        *self.stats.by_type.entry(metadata.message_type).or_insert(0) += 1;

        // 更新共享狀態
        {
            let mut state = self.state.lock().unwrap();
            state.total_messages = self.stats.total_messages;
            state.total_bytes = self.stats.total_bytes;

            let type_key = format!("{:?}", metadata.message_type);
            *state.by_type.entry(type_key).or_insert(0) += 1;
        }

        // 打印傳輸開始資訊
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📡 LoRa 傳輸開始");
        println!("   類型: {:?}", metadata.message_type);
        println!("   從: {} → 到: {}", metadata.from, metadata.to);
        println!("   Payload 大小: {} bytes", payload.len());
        println!("   預計片段數: {}", (payload.len() + self.config.fragment_size - 1) / self.config.fragment_size);
        println!();

        // 執行分片傳輸
        self.transmit_with_fragmentation(&metadata, payload);

        println!();
    }

    fn get_stats(&self) -> Option<TransportStats> {
        Some(self.stats.clone())
    }

    fn reset(&mut self) {
        self.stats = TransportStats::default();

        let mut state = self.state.lock().unwrap();
        *state = LoRaTransportState::default();
    }
}

// ============================================================================
// 測試
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lora_transport_fragmentation() {
        let mut transport = SimulatedLoRaTransport::new();

        // 測試小型訊息（不需分片）
        transport.send(
            MessageMetadata {
                from: "test".to_string(),
                to: "dest".to_string(),
                message_type: MessageType::Other,
                timestamp: None,
            },
            "small",
        );

        // 測試大型訊息（需要分片）
        let large_payload = "x".repeat(200);
        transport.send(
            MessageMetadata {
                from: "test".to_string(),
                to: "dest".to_string(),
                message_type: MessageType::Other,
                timestamp: None,
            },
            &large_payload,
        );

        let stats = transport.get_stats().unwrap();
        assert_eq!(stats.total_messages, 2);
    }
}
