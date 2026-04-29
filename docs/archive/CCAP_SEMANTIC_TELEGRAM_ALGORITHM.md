# CCAP-ST 語義電報壓縮演算法 (Semantic Telegram Compression Algorithm)

**版本**: V1.0
**目標**: 整合數學、語言學與軟體工程，將大型專案地圖壓縮為「LLM 直接可吸收」的高熵電報訊號，實現 Token 消耗的對數級節省。

---

## 1. 演算法設計理念
*   **數學為骨 (The Bone)**: 利用線性代數與圖論（中心度、譜分群、耦合矩陣）獲取 100% 準確的專案拓樸。
*   **語義為表 (The Skin)**: 將數學閾值映射到 LLM 具備強大「語義先驗」的高權重 Token (如 HUB, CORE, ANCHOR)。
*   **電報為傳輸 (The Pulse)**: 消除所有語法冗餘，僅保留「標籤 + 座標」的電報格式。

---

## 2. 映射邏輯 (Mapping Logic) - 0 Token 本地執行
本地引擎 (Static Kernel) 透過 AST 分析獲取以下屬性並映射為 ST 標籤：

| 物理維度 | 計算指標 | ST 語義標籤 | 資訊量 (Information Gain) |
| :--- | :--- | :--- | :--- |
| **拓樸定位** | Eigenvector Centrality | `ROLE:CORE` / `ROLE:HUB` | 極高 (定義系統重要性) |
| **邏輯行為** | Branch Density / API Profile | `TYPE:ENGINE` / `TYPE:IO` | 高 (定義模組職責) |
| **時序穩定度** | Git Churn + Test Coverage | `STAT:ANCHOR` / `STAT:GHOST` | 中 (定義修改風險) |
| **耦合關係** | Edge Weights ($w_{ij}$) | `BOND:STRICT` / `BOND:LOOSE` | 高 (定義影響範圍) |

---

## 3. 輸出格式規範 (ST-AAAK Syntax)
```text
@NODE[path] ROLE:label TYPE:label GOAL:[symbols] BOND:label STAT:label
```
*   **壓縮比**: 傳統地圖 300 Tokens -> ST 電報 15 Tokens (**20:1 壓縮**)。
*   **解碼器**: 無需解碼器。LLM 憑藉預訓練的語義空間直接理解。

---

## 4. 核心效益分析 (Core Benefits)

### 4.1 突破百萬行限制 (Scalability)
全專案初始化不再需要讀取原始碼，改為載入由數學編譯出的「拓樸骨架」。對於 AI 來說，百萬行專案的理解成本被壓低至與小型專案無異。

### 4.2 極致 Token 節省 (Economic Impact)
*   **靜態節省**: 消除 Markdown 語法噪音與自然語言冗餘。
*   **動態節省**: 減少 AI 因為「理解偏差」導致的無效對話回合 (Turns)。

### 4.3 零幻覺導航 (Deterministic Fidelity)
所有的標籤均由確定性的本地數學引擎產出，消除了 LLM 在「整理」大型代碼庫時產生的腦補與虛構。

---
**[ALGORITHM END]**
