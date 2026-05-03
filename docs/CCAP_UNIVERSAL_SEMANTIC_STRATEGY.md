# CCAP 通用語義策略 (Universal Semantic Strategy)

**版本**: 1.0
**日期**: 2026-05-04
**狀態**: 執行中 (In Progress)

## 1. 核心願景 (Vision)
`ccap-kernel` 的目標不是成為另一個編譯器，而是成為 **AI 與百萬行代碼之間的「語義通訊協議」**。我們致力於透過「數據蒸餾」實現 0 Token 冷啟動，將龐大的工程事實壓縮成極輕量的拓樸地圖。

## 2. 責任邊界 (Responsibility Boundary)

### 2.1 不做的事情 (Non-Goals)
*   **拒絕語言特定邏輯**：不再手動編寫追蹤 Python 縮進、C++ 宏、或 Java 重載的代碼。
*   **不管理編譯環境**：使用者需自行安裝並執行該語言的專業分析工具（如 SCIP 索引器）。

### 2.2 著重的核心 (Core Focus)
*   **語義消費 (Semantic Consumption)**：消費由專業工具產出的 SCIP/LSIF 數據。
*   **拓樸壓縮 (Topological Compression)**：將數 MB 的索引蒸餾為 < 10KB 的語義地圖。
*   **診斷與導引 (Diagnostic Guidance)**：當偵測到符號碰撞時，提供精確的「人性化修復建議」。

## 3. 架構層級 (Architecture Layers)

### 第一層：快速掃描 (Fast / Tree-sitter Layer)
*   **目的**: 極速建立初始結構地圖。
*   **局限**: 僅具備語法識別，無語義鏈結（會產生符號碰撞）。
*   **狀態**: 標註為 `[UNVERIFIED_SYNTAX]`。

### 第二層：深層語義 (Deep / SCIP Layer)
*   **目的**: 消除歧義，建立跨文件引用與精確作用域。
*   **來源**: 讀取外部 `.scip` 文件。
*   **狀態**: 標註為 `[FORMAL_SEMANTICS]`。

## 4. 語言導引表 (Language Guide Map)

| 語言 (Ext) | 推薦 SCIP 工具 | 獲取方式 |
| :--- | :--- | :--- |
| `.py` | `scip-python` | `pip install scip-python` |
| `.ts/.js` | `scip-typescript` | `npm install -g @sourcegraph/scip-typescript` |
| `.go` | `scip-go` | `go install github.com/sourcegraph/scip-go/cmd/scip-go@latest` |
| `.rs` | `scip-rust` | `cargo install scip-rust` |
| `.cpp/.c` | `scip-clang` | [Sourcegraph Github](https://github.com/sourcegraph/scip-clang) |

## 5. 驗證標準 (Verification Standards)
*   **無碰撞地圖**：在 `FORMAL_SEMANTICS` 模式下，碰撞率必須為 0%。
*   **極低消耗**：產出的 Map 文件大小必須與 SCIP 原文件維持 1:100 以上的壓縮比。
*   **0 Token 準確度**：AI 僅憑 Map 就能精確定位 95% 以上的符號。
