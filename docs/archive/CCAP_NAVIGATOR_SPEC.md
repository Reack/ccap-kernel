# CCAP Navigator (ccap_navigator) 工具規格書

**版本**: V2.0 (Math-Semantic Topology Edition)
**目標**: 透過高維特徵向量與有向無環圖 (DAG) 拓樸結構，對專案結構進行無損數學壓縮，並提供結構感知的精確檢索與展開。

---

## 1. 核心設計概念：張量檢索與結構感知 RAG (Structure-Aware RAG)
`ccap_navigator` 不再處理單純的自然語言，而是處理 **軟體特徵空間 (Software Eigen-Space)**。它負責在本地計算模組的特徵向量 (Control, Data, I/O, UI) 與依賴權重矩陣，並透過 AAAK-Math 協定將「拓樸流形 (Topological Manifold)」直接映射到 LLM 的潛空間中。

---

## 2. 指令集 (The Tensor Command Set)

### `ccap_navigator skeleton [path]`
*   **用途**: 獲取全域或局部拓樸骨架 (Global/Local Spanning Tree)。
*   **本地行為**: 遍歷 `.meta.json` 依賴樹，提取指定深度的高權重邊界與特徵向量。
*   **回傳格式 (AAAK-Math)**:
    `@DAG:{ UI>[Auth], Auth>[DB_Pool(0.9)] }`
    `@V:[Auth(0.8,0.2,0.9,0.0|S:1)]`

### `ccap_navigator unfold [symbol_or_path] --degree [n]`
*   **用途**: 流形展開 (Manifold Unfolding)。當需要深度理解特定模組時，將其在高維空間中的一階/二階鄰居展平。
*   **本地行為**: 
    1. 計算該節點的 In-degree (呼叫者) 與 Out-degree (依賴項)。
    2. 若有必要，解壓該節點的自然語言目的 (Purpose)。
*   **回傳格式**:
    `UNFOLD:[AuthSvc] | In:[UI_Ctrl] | Out:[DB_Pool, LogSvc] | Purp:"處理使用者 JWT 授權"`

### `ccap_navigator trace [symbol]`
*   **用途**: 耦合與熱點追蹤 (Hot-path Tracing)。
*   **本地行為**: 基於依賴權重 (w) 找出最核心的執行路徑，用於尋找效能瓶頸或修改風險。
*   **回傳格式**:
    `PATH:[AuthSvc] -> [DB_Pool] (Risk: HIGH, Weight: 0.9)`

---

## 3. Token 節省技術 (Mathematical Token Optimization)

1.  **高熵符號編碼 (High-Entropy Symbol Encoding)**: 放棄自然語言的文法冗餘，使用 `@M:[Name|V:c,d,i,p|S:s,R:r]` 等張量符號，達到近乎 100:1 的無損壓縮。
2.  **空間降維 (Dimensionality Reduction)**: 初始僅提供核心骨架，避免載入末端葉節點 (Leaf nodes)，防止 Context Window 溢出。
3.  **直接神經映射 (Direct Neural Mapping)**: LLM 原生支援關聯結構 (Attention) 與特徵捕捉 (Embeddings)，AAAK-Math 格式無需「解碼」即可直接作為邏輯推理的上下文基底。

---

## 4. 跨平台與語言無關性
*   **實作**: 使用 Python 撰寫。
*   **解析器**: 內建簡單的 YAML/JSON 解析邏輯與 Tree-sitter 介面。
*   **相容性**: 支援 POSIX 路徑與 Windows 路徑自動轉換。

---
**[SPEC END]**
