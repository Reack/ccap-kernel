# CCAP 巨人肩膀：技術文獻與工具集成調研報告 (CCAP Research & Integration Report)

**版本**: V1.0
**核心目標**: 透過整合 SCIP 協定、線性代數圖論與語言學理論，為 `ccap-kernel` 建立一套工業級、語言無關且 100% 準確的語義導航底座。

---

## 1. 符號與索引標準：SCIP (Source Code Indexing Protocol)

### 1.1 為什麼選擇 SCIP 而非 LSIF？
SCIP 是由 Sourcegraph 針對大專案搜尋與導航設計的最新標準。
*   **物理壓縮性**: SCIP 採用 Protobuf 編碼，比 JSON 格式的 LSIF 體積縮小 10 倍以上。
*   **語言無關的符號 ID**: SCIP 定義了一套標準的 Symbol Naming 規則：
    `scip-python . . db.Database#get_user().`
*   **對 CCAP 的意義**: 我們將採用 SCIP 的符號命名規範。這保證了不論是 Python 定義的符號，還是被 JS 呼叫的符號，在我們 Rust 核心的「特徵空間」中都有唯一的、可預測的座標。

---

## 2. 數學權重演算法：Component Rank (圖論與線性代數)

### 2.1 從 PageRank 到架構權重
我們將專案視為一個馬可夫鏈 (Markov Chain)，利用 Google PageRank 的變體演算法：
*   **權重矩陣 ($A$)**: 由符號的「定義與引用」鏈結組成。
*   **核心算法**: 求解特徵向量方程式 $Av = \lambda v$。
*   **結果指標**: 
    *   **Authority (權威度)**: 指向該節點的依賴越多，權威度越高 $\rightarrow$ 映射為 `ROLE:CORE`。
    *   **Hub (中轉度)**: 該節點呼叫的外部符號越多，中轉度越高 $\rightarrow$ 映射為 `ROLE:HUB`。
*   **優勢**: 這排除了人工設定閾值的偏差，改由純粹的線性代數揭示軟體架構的「真實重心」。

---

## 3. 語言學支撐：結構主義 (Structuralist Linguistics)

### 3.1 符號學與語義電報 (Semiotics)
根據 **Ferdinand de Saussure** 的理論：
*   **能指 (Signifier)**: 我們輸出的 ST-AAAK 標籤（如 `ROLE:HUB`）。
*   **所指 (Signified)**: 該模組在開發者腦中代表的「核心交換中心」概念。
*   **關聯價值**: 符號的價值不在於其本身，而在於與系統中其他符號的對比。
*   **對 CCAP 的意義**: 這證實了我們不需要傳輸「自然語言描述」。只要拓樸關係（結構）是準確的，這串「高熵電報」就能在 LLM 的潛空間中自動激發出正確的工程直覺。

---

## 4. 實作路徑：整合與落地 (Implementation Path)

我們將在 `ccap-kernel` 的下一階段進行以下整合：

1.  **SCIP-lite 產生器**:
    在 `Extractor` 中實作標準的 SCIP 符號生成邏輯，取代原本脆弱的字串比對。
2.  **Petgraph + nalgebra 混合運算**:
    將 `Linker` 偵測到的 BOND 轉換為 `nalgebra` 矩陣，執行 Component Rank 運算，精確量化每個節點的系統價值。
3.  **語義電報字典規範化**:
    基於結構主義語言學，選擇資訊熵最高、且最能引起 AI 語義共鳴的 16 個基石標籤。

---
**[RESEARCH END]**
