# CCAP 數學語義壓縮協定 (Mathematical Semantic Compression Protocol)

**版本**: V3.0 (Paradigm Shift)
**目標**: 放棄自然語言總結，改以線性代數、圖論與資訊熵為基礎，對軟體專案進行「高維度特徵空間映射」與「無損拓樸壓縮」，實現 LLM 的直接張量級理解。

---

## 1. 核心理念：從「閱讀文字」到「空間映射」
大型語言模型 (LLM) 底層是基於 Transformer 的注意力矩陣運算。傳統的「自然語言地圖」存在極大的語法冗餘 (Syntactic Redundancy)。
*   **舊典範**: 將程式碼總結為文字，讓 LLM 透過文字重建依賴圖譜。（高 Token 消耗，易產生幻覺）
*   **新典範**: 透過本地靜態分析，直接計算專案的**有向無環圖 (DAG) 拓樸**與**模組特徵向量 (Feature Vectors)**，並以極致高熵的符號編碼 (AAAK-Math) 直接注入 LLM 的上下文。

---

## 2. 專案特徵空間映射 (Project Eigen-Space Mapping)
將任何軟體模組抽象為一個 $N$ 維向量空間中的座標。

### 2.1 正交基底定義 (Orthogonal Basis)
定義一組描述軟體模組本質的正交基底向量 $V = [v_1, v_2, v_3, v_4]$：
*   $v_1$ (`C`): **Control / Logic** (業務邏輯、演算法複雜度)
*   $v_2$ (`D`): **Data / State** (資料模型、狀態存儲、DTO)
*   $v_3$ (`I`): **I/O / Side-Effect** (網路、資料庫、硬體通訊)
*   $v_4$ (`P`): **Presentation** (UI、渲染、視圖)

### 2.2 模組狀態標量 (State Scalars)
利用三元邏輯 (-1, 0, 1) 或標準化數值定義模組的健康度與生命週期：
*   $S_1$ (`St`): **Stability** (穩定度: `1`=穩定, `0`=重構中, `-1`=已知缺陷/Broken)
*   $S_2$ (`Rk`): **Risk/Coupling** (修改風險: 高耦合或核心路徑的脆弱度)

### 2.3 編碼範例 (Tensor Representation)
一個處理登入邏輯並連線資料庫的模組 `AuthService.ts`，傳統需要 50 Tokens 描述。
**高熵數學編碼**:
`@M:[AuthSvc|V:0.8,0.2,0.9,0.0|S:1,R:0.8]`
*(解碼: 高邏輯、高 I/O、無 UI、狀態穩定但修改風險高)*

---

## 3. 無損拓樸壓縮與圖論表示 (Lossless Topological Compression)
軟體架構的本質是依賴網路。我們不使用文字列出依賴，而是直接編碼**稀疏 adjacency list (鄰接表)** 或 **邊界權重 (Edge Weights)**。

### 3.1 邊界與權重編碼 (Edge & Weight Encoding)
*   **格式**: `A >[ B(w1), C(w2) ]` (A 依賴 B 和 C，括號內為耦合權重或呼叫頻率)。
*   **雙向/循環依賴**: `A <> B` (這在軟體工程中是架構壞味道，LLM 看到此符號能瞬間鎖定架構缺陷)。

### 3.2 全景拓樸骨架 (Global Spanning Tree Skeleton)
會話初始化時，不載入文字地圖，而是載入專案的核心拓樸骨架：
```
@DAG:
{ 
  UI >[ Auth, Dashboard ], 
  Auth >[ DB_Pool(w:0.9), Crypt(w:0.2) ], 
  Dashboard >[ DB_Pool(w:0.5) ] 
}
```
*(LLM 透過注意力機制瞬間理解 `DB_Pool` 是系統瓶頸 (High In-degree)，無需任何文字解釋。)*

---

## 4. 結構感知 RAG (Structure-Aware RAG)
結合向量資料庫與圖神經網路 (GNN) 的概念，實現精確的局部展開：
1.  **全景降維 (Dimensionality Reduction)**: 啟動時僅提供頂層模組的拓樸骨架與特徵中心點 (Centroids)。
2.  **流形展開 (Manifold Unfolding)**: 當 LLM 決定介入 `Auth` 模組時，發出指令。本地工具 (`ccap_navigator`) 計算該節點在拓樸空間中的**一階/二階鄰居 (1st/2nd degree neighbors)**，並將此局部子圖 (Local Subgraph) 以高熵符號展開給 LLM。
3.  **精確手術 (Surgical Edits)**: LLM 在精確的局部拓樸與狀態標量引導下，進行零副作用的代碼修改。

---
**[DOCUMENT END]**
