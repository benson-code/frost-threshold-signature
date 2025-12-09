#!/usr/bin/env python3
"""
FROST-T Demo 驗證腳本
===================

這個腳本會自動驗證 FROST-T demo 的所有功能，包括：
1. 健康檢查
2. 狀態監控
3. 簽章請求
4. 結果驗證

使用方法：
    python verify_demo.py

前置條件：
    1. 先執行 demo-basic.bat 或手動啟動 HTTP server
    2. 確保 http://127.0.0.1:3000 可以連線
"""

import requests
import time
import json
import sys
from datetime import datetime
from typing import Dict, Any, Optional

# 終端機顏色碼
class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'
    RESET = '\033[0m'

def print_header(text: str):
    """印出標題"""
    print(f"\n{Colors.BOLD}{Colors.CYAN}{'='*70}{Colors.RESET}")
    print(f"{Colors.BOLD}{Colors.CYAN}{text.center(70)}{Colors.RESET}")
    print(f"{Colors.BOLD}{Colors.CYAN}{'='*70}{Colors.RESET}\n")

def print_success(text: str):
    """印出成功訊息"""
    print(f"{Colors.GREEN}✓ {text}{Colors.RESET}")

def print_error(text: str):
    """印出錯誤訊息"""
    print(f"{Colors.RED}✗ {text}{Colors.RESET}")

def print_warning(text: str):
    """印出警告訊息"""
    print(f"{Colors.YELLOW}⚠ {text}{Colors.RESET}")

def print_info(text: str):
    """印出資訊"""
    print(f"{Colors.BLUE}ℹ {text}{Colors.RESET}")

def format_timestamp() -> str:
    """取得格式化的時間戳"""
    return datetime.now().strftime("%H:%M:%S")

# API 基礎 URL
BASE_URL = "http://127.0.0.1:3000"

def check_health() -> bool:
    """
    步驟 1: 健康檢查
    確認 HTTP server 正在運行
    """
    print_header("步驟 1: 健康檢查")

    try:
        response = requests.get(f"{BASE_URL}/health", timeout=5)

        if response.status_code == 200:
            data = response.json()
            print_success(f"Server 正常運行")
            print_info(f"  服務: {data.get('service', 'N/A')}")
            print_info(f"  版本: {data.get('version', 'N/A')}")
            print_info(f"  狀態: {data.get('status', 'N/A')}")
            return True
        else:
            print_error(f"健康檢查失敗 (HTTP {response.status_code})")
            return False

    except requests.exceptions.ConnectionError:
        print_error("無法連線到 server！")
        print_warning("請確認:")
        print_warning("  1. 已執行 demo-basic.bat 或手動啟動 server")
        print_warning("  2. Server 正在監聽 http://127.0.0.1:3000")
        return False
    except Exception as e:
        print_error(f"健康檢查發生錯誤: {e}")
        return False

def monitor_status(duration_seconds: int = 3) -> Optional[Dict[str, Any]]:
    """
    步驟 2: 狀態監控
    持續監控 /status 端點，觀察初始狀態
    """
    print_header("步驟 2: 初始狀態監控")

    print_info(f"開始監控 {duration_seconds} 秒...")
    print()

    start_time = time.time()
    last_status = None

    while time.time() - start_time < duration_seconds:
        try:
            response = requests.get(f"{BASE_URL}/status", timeout=2)

            if response.status_code == 200:
                status = response.json()
                last_status = status

                # 印出關鍵資訊
                timestamp = format_timestamp()
                phase = status.get('current_phase', 'Unknown')
                progress = status.get('progress', 0.0) * 100
                total_msgs = status.get('total_messages', 0)
                total_bytes = status.get('total_bytes', 0)
                retries = status.get('total_retries', 0)
                rssi = status.get('rssi', 0)

                print(f"[{timestamp}] Phase: {Colors.MAGENTA}{phase:15s}{Colors.RESET} | "
                      f"Progress: {Colors.CYAN}{progress:5.1f}%{Colors.RESET} | "
                      f"Messages: {Colors.YELLOW}{total_msgs:3d}{Colors.RESET} | "
                      f"Bytes: {total_bytes:5d} | "
                      f"Retries: {retries:2d} | "
                      f"RSSI: {rssi:4d} dBm")

            time.sleep(0.5)  # 每 0.5 秒查詢一次

        except Exception as e:
            print_error(f"狀態監控錯誤: {e}")
            break

    print()
    if last_status:
        print_success("狀態監控完成")
        return last_status
    else:
        print_error("無法取得狀態資訊")
        return None

def send_sign_request() -> Optional[Dict[str, Any]]:
    """
    步驟 3: 發送簽章請求
    透過 POST /sign 觸發簽章流程
    """
    print_header("步驟 3: 發送簽章請求")

    request_payload = {
        "message": "bitcoin++ Taipei 2025 - FROST-T Demo",
        "signer_ids": [1, 2, 3]
    }

    print_info(f"發送簽章請求:")
    print_info(f"  訊息: {request_payload['message']}")
    print_info(f"  簽署者: {request_payload['signer_ids']}")
    print()

    try:
        # 記錄開始前的狀態
        status_before = requests.get(f"{BASE_URL}/status").json()
        bytes_before = status_before.get('total_bytes', 0)
        progress_before = status_before.get('progress', 0.0)

        print_info(f"開始前狀態: total_bytes={bytes_before}, progress={progress_before:.2f}")
        print()

        # 發送 POST 請求
        print_info("正在執行簽章流程...")
        response = requests.post(
            f"{BASE_URL}/sign",
            json=request_payload,
            timeout=30
        )

        if response.status_code == 200:
            result = response.json()

            print_success("簽章請求成功！")
            print()
            print_info("簽章結果:")
            print_info(f"  訊息: {result.get('message', 'N/A')}")
            print_info(f"  簽署者: {result.get('signer_ids', [])}")
            print_info(f"  簽章 (hex): {result.get('signature', 'N/A')[:80]}...")

            verified = result.get('verified', False)
            if verified:
                print_success(f"  驗證狀態: {Colors.BOLD}通過 ✓{Colors.RESET}")
            else:
                print_error(f"  驗證狀態: {Colors.BOLD}失敗 ✗{Colors.RESET}")

            print()

            # 檢查狀態變化
            time.sleep(0.5)
            status_after = requests.get(f"{BASE_URL}/status").json()
            bytes_after = status_after.get('total_bytes', 0)
            progress_after = status_after.get('progress', 0.0)

            print_info(f"完成後狀態: total_bytes={bytes_after}, progress={progress_after:.2f}")
            print()

            # 驗證變化
            if bytes_after > bytes_before:
                print_success(f"✓ total_bytes 有增加 ({bytes_before} → {bytes_after})")
            else:
                print_warning(f"⚠ total_bytes 沒有增加 ({bytes_before} → {bytes_after})")

            if progress_after >= 0.99:  # 允許浮點數誤差
                print_success(f"✓ progress 達到 1.0 ({progress_after:.2f})")
            else:
                print_warning(f"⚠ progress 未達 1.0 ({progress_after:.2f})")

            return result
        else:
            print_error(f"簽章請求失敗 (HTTP {response.status_code})")
            print_error(f"回應: {response.text}")
            return None

    except requests.exceptions.Timeout:
        print_error("簽章請求逾時（超過 30 秒）")
        return None
    except Exception as e:
        print_error(f"簽章請求發生錯誤: {e}")
        return None

def monitor_signing_process(duration_seconds: int = 5):
    """
    步驟 4: 監控簽章過程
    觀察簽章過程中的狀態變化
    """
    print_header("步驟 4: 監控簽章過程")

    print_info(f"監控簽章過程 {duration_seconds} 秒...")
    print_info("觀察重點: progress 從 0.0 → 1.0, phase 變化, retries 增加")
    print()

    start_time = time.time()
    seen_phases = set()
    max_progress = 0.0
    max_retries = 0

    while time.time() - start_time < duration_seconds:
        try:
            response = requests.get(f"{BASE_URL}/status", timeout=2)

            if response.status_code == 200:
                status = response.json()

                timestamp = format_timestamp()
                phase = status.get('current_phase', 'Unknown')
                progress = status.get('progress', 0.0)
                total_msgs = status.get('total_messages', 0)
                total_bytes = status.get('total_bytes', 0)
                retries = status.get('total_retries', 0)

                seen_phases.add(phase)
                max_progress = max(max_progress, progress)
                max_retries = max(max_retries, retries)

                # 動態進度條
                bar_width = 30
                filled = int(bar_width * progress)
                bar = '█' * filled + '░' * (bar_width - filled)

                print(f"[{timestamp}] {Colors.CYAN}{bar}{Colors.RESET} "
                      f"{progress*100:5.1f}% | "
                      f"Phase: {Colors.MAGENTA}{phase:15s}{Colors.RESET} | "
                      f"Retries: {Colors.YELLOW}{retries:2d}{Colors.RESET}")

            time.sleep(0.5)

        except Exception as e:
            print_error(f"監控錯誤: {e}")
            break

    print()
    print_success("監控完成")
    print()
    print_info(f"觀察到的階段: {', '.join(seen_phases)}")
    print_info(f"最大進度: {max_progress:.2f}")
    print_info(f"總重傳次數: {max_retries}")
    print()

    # 驗證重傳次數
    if max_retries > 0:
        print_success(f"✓ 觀察到重傳行為 (total_retries = {max_retries})")
        print_info("  這證明了 10% 掉包率的模擬正在運作")
    else:
        print_warning("⚠ 未觀察到重傳 (total_retries = 0)")
        print_info("  這可能是機率問題，10% 掉包率不保證一定發生")

def verify_signature_response(result: Dict[str, Any]) -> bool:
    """
    步驟 5: 驗證簽章回應格式
    """
    print_header("步驟 5: 驗證簽章回應")

    checks_passed = 0
    total_checks = 4

    # 檢查 1: 是否包含 signature 欄位
    if 'signature' in result and result['signature']:
        if result['signature'].startswith('Error:'):
            print_error("簽章包含錯誤訊息")
        else:
            print_success("✓ 包含 signature 欄位")
            checks_passed += 1
    else:
        print_error("✗ 缺少 signature 欄位")

    # 檢查 2: 是否包含 verified 欄位
    if 'verified' in result:
        if result['verified'] is True:
            print_success("✓ verified = true (簽章驗證通過)")
            checks_passed += 1
        else:
            print_error("✗ verified = false (簽章驗證失敗)")
    else:
        print_error("✗ 缺少 verified 欄位")

    # 檢查 3: 是否包含 message 欄位
    if 'message' in result:
        print_success(f"✓ 包含 message 欄位: '{result['message']}'")
        checks_passed += 1
    else:
        print_error("✗ 缺少 message 欄位")

    # 檢查 4: 是否包含 signer_ids 欄位
    if 'signer_ids' in result and isinstance(result['signer_ids'], list):
        print_success(f"✓ 包含 signer_ids 欄位: {result['signer_ids']}")
        checks_passed += 1
    else:
        print_error("✗ 缺少或格式錯誤的 signer_ids 欄位")

    print()
    print_info(f"通過檢查: {checks_passed}/{total_checks}")

    return checks_passed == total_checks

def check_event_log():
    """
    步驟 6: 檢查事件日誌
    """
    print_header("步驟 6: 檢查事件日誌")

    try:
        response = requests.get(f"{BASE_URL}/status")
        status = response.json()

        events = status.get('recent_events', [])

        if not events:
            print_warning("事件日誌為空")
            print_info("這可能表示 demo-basic 已經執行完成，狀態已被 POST /sign 重置")
            return

        print_info(f"最近的 {len(events)} 個事件:")
        print()

        for i, event in enumerate(events[-10:], 1):  # 只顯示最後 10 個
            event_type = event.get('type', 'Unknown')

            if event_type == 'TransmitStart':
                from_node = event.get('from', 'N/A')
                to_node = event.get('to', 'N/A')
                msg_type = event.get('message_type', 'N/A')
                print(f"  {i}. {Colors.GREEN}[START]{Colors.RESET} {from_node} → {to_node} | {msg_type}")

            elif event_type == 'TransmitFragment':
                frag_id = event.get('fragment_id', 0)
                total = event.get('total_fragments', 0)
                bytes_sent = event.get('bytes', 0)
                print(f"  {i}. {Colors.CYAN}[FRAGMENT]{Colors.RESET} {frag_id}/{total} | {bytes_sent} bytes")

            elif event_type == 'PacketLost':
                frag_id = event.get('fragment_id', 0)
                retry = event.get('retry_count', 0)
                print(f"  {i}. {Colors.RED}[LOST]{Colors.RESET} Fragment {frag_id} | Retry {retry}")

            elif event_type == 'RetrySuccess':
                frag_id = event.get('fragment_id', 0)
                retry = event.get('retry_count', 0)
                print(f"  {i}. {Colors.YELLOW}[RETRY OK]{Colors.RESET} Fragment {frag_id} | After {retry} retries")

            elif event_type == 'TransmitComplete':
                time_ms = event.get('total_time_ms', 0)
                retries = event.get('retries', 0)
                print(f"  {i}. {Colors.GREEN}[COMPLETE]{Colors.RESET} {time_ms}ms | {retries} retries")

        print()
        print_success("事件日誌檢查完成")

    except Exception as e:
        print_error(f"檢查事件日誌時發生錯誤: {e}")

def print_final_summary(all_checks_passed: bool):
    """印出最終摘要"""
    print_header("驗證摘要")

    if all_checks_passed:
        print(f"{Colors.GREEN}{Colors.BOLD}")
        print("  ✓✓✓ 所有驗證項目通過！ ✓✓✓")
        print(f"{Colors.RESET}")
        print()
        print_success("FROST-T Demo 運作正常，可以進行展示！")
        print()
        print_info("下一步:")
        print_info("  1. 確認 dashboard.html 在瀏覽器中正常顯示")
        print_info("  2. 執行完整的 demo-basic.bat 觀察視覺化效果")
        print_info("  3. 準備好在 bitcoin++ Taipei 2025 大展身手！")
    else:
        print(f"{Colors.RED}{Colors.BOLD}")
        print("  ✗✗✗ 部分驗證項目未通過 ✗✗✗")
        print(f"{Colors.RESET}")
        print()
        print_warning("請檢查錯誤訊息並修正問題")
        print()
        print_info("常見問題:")
        print_info("  1. Server 未啟動 → 執行 demo-basic.bat")
        print_info("  2. Port 3000 被佔用 → 關閉其他程式或修改 port")
        print_info("  3. 編譯錯誤 → 執行 cargo check --bin frost-cli")

def main():
    """主程式"""
    print()
    print(f"{Colors.BOLD}{Colors.MAGENTA}")
    print("╔════════════════════════════════════════════════════════════════════╗")
    print("║                                                                    ║")
    print("║              FROST-T Demo 自動化驗證腳本                          ║")
    print("║              bitcoin++ Taipei 2025                                ║")
    print("║                                                                    ║")
    print("╚════════════════════════════════════════════════════════════════════╝")
    print(f"{Colors.RESET}")

    all_checks = []

    # 步驟 1: 健康檢查
    health_ok = check_health()
    all_checks.append(health_ok)

    if not health_ok:
        print_error("\n無法繼續測試，請先啟動 HTTP server")
        sys.exit(1)

    # 步驟 2: 初始狀態監控
    initial_status = monitor_status(duration_seconds=3)
    all_checks.append(initial_status is not None)

    # 步驟 3: 發送簽章請求
    sign_result = send_sign_request()
    all_checks.append(sign_result is not None)

    if sign_result:
        # 步驟 5: 驗證簽章回應
        result_valid = verify_signature_response(sign_result)
        all_checks.append(result_valid)

    # 步驟 6: 檢查事件日誌
    check_event_log()

    # 最終摘要
    all_passed = all(all_checks)
    print_final_summary(all_passed)

    # 回傳狀態碼
    sys.exit(0 if all_passed else 1)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n\n{Colors.YELLOW}使用者中斷測試{Colors.RESET}")
        sys.exit(130)
    except Exception as e:
        print(f"\n{Colors.RED}發生未預期的錯誤: {e}{Colors.RESET}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
