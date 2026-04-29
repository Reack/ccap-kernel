# CCAP 通用診斷架構與跨語言協定規範 (Universal Diagnostic Architecture & Cross-Language Protocol)

**版本**: V1.0
**目標**: 實現跨平台 (Win/Mac/Linux)、語言無關 (Language Agnostic) 的低 Token 精確除錯。

---

## 1. 核心設計哲學：適配器與去耦
將 **「如何偵測 (Detection)」** 與 **「如何修復 (Fixing)」** 徹底解構。
*   **工具層 (Detection)**: 由本地高效能、零 Token 的指令/腳本執行。
*   **大腦層 (Fixing)**: AI 僅解析工具產出的「標準化報告」，透過地圖指標 (Atlas Pointers) 進行精確修復。

---

## 2. 三層診斷架構 (The Triple-Layer Architecture)

### [層級 A] 基礎資源層 (Hardware & OS Layer) - **100% 通用**
*   **偵測目標**: 記憶體洩漏 (Memory Leak)、CPU 瓶頸、進程崩潰、磁碟 I/O。
*   **實作技術**: Python `psutil`, `subprocess`, `os`。
*   **平台支援**: 透過 Python 統一 Windows/macOS/Linux 的差異。

### [層級 B] 語義與耦合層 (Static & Semantic Layer) - **協定通用**
*   **偵測目標**: 符號依賴、耦合性分析、影響範圍評估。
*   **實作技術**: 
    *   **LSP (Language Server Protocol)**: 呼叫當前環境的 Language Server。
    *   **Tree-sitter**: 本地進行 AST 語法樹分析。
*   **語言支援**: 透過標準 LSP 介面支援 C#, Python, Rust, Go, TypeScript 等。

### [層級 C] 模式與日誌層 (Pattern & Log Layer) - **注入通用**
*   **偵測目標**: 運行時異常、邏輯斷層、環境變數缺失。
*   **實作技術**: 
    *   **正則診斷引擎**: 統一解析各語言的 StackTrace。
    *   **環境審計 (SCM)**: 檢查本地運行環境快照。

---

## 3. 診斷回報協定 (DRP - Diagnostic Reporting Protocol)
所有底層工具必須將複雜數據壓縮為以下 JSON 格式回傳給 AI：

```json
{
  "protocol": "CCAP_DRP_V1",
  "issue": {
    "type": "MEMORY_LEAK | RACE_CONDITION | PERFORMANCE | LOGIC_GAP | ENV_DRIFT",
    "severity": "CRITICAL | WARNING | INFO"
  },
  "location": {
    "file": "relative/path/to/file",
    "line": 42,
    "symbol": "function_or_class_name",
    "confidence": 0.95
  },
  "evidence": {
    "summary": "簡短的文字證據，如：記憶體在 10 分鐘內增長了 500MB",
    "raw_data_pointer": "path/to/raw/log_or_profile"
  },
  "navigation": {
    "suggested_nodes": ["/src/module_a", "/src/libs/db_pool"],
    "impact_scope": ["/src/ui", "/api/v1"]
  }
}
```

---

## 4. 漸進式增強策略 (Progressive Enhancement)
診斷系統將根據當前設備的環境自動調整能力等級：
1.  **Level 1 (Zero-Dep)**: 僅使用內建 Python 庫。提供基礎地圖導航與資源監控。
2.  **Level 2 (Git-Aware)**: 偵測到 Git 環境。啟用 `couch-potato` 增量同步與版本回溯。
3.  **Level 3 (Tool-Rich)**: 偵測到 LSP 或 Profiler。啟用符號級追蹤與效能診斷。

---
**[DOCUMENT END]**
