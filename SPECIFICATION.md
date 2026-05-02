# CCAP 4.0 核心技術規格書 (The Master Specification)

**版本**: 4.0 (Industrial Grade)
**狀態**: 定案 / 施工中
**宗旨**: 透過數學拓樸與語義電報技術，實現百萬行專案在 AI 協作下的 0 Token 冷啟動、無損導航與極致資源節省。

---

## 第一章：核心架構 (System Architecture)

### 1.1 CSK (CCAP Static Kernel) - 核心引擎
*   **技術選型**: Rust + Tree-sitter + nalgebra (線性代數核心)。
*   **定位**: 語言無關、平台無關的高效能「語義編譯器」。
*   **分發**: 透過 `pip` (PyO3) 與 `npm` (N-API) 混合分發，確保「隨插即用」的開發者體驗。

### 1.2 影子目錄 (.ccap/)
*   **VNM (物理層)**: `_MAP.meta.json` 存儲機器可讀的 AST 特徵、符號座標與依賴權重矩陣。
*   **SCA (語義層)**: `_MAP.md` 存儲 AI/人類可讀的素顏摘要，嚴禁任何 Markdown 視覺裝飾。

---

## 第二章：ST-AAAK 語義電報協定 (Semantic Telegram Protocol)

### 2.1 數學驅動的語義映射
CSK 在本地 0 Token 成本下，將數學計算結果映射為 LLM 強力先驗的高權重 Token。

| 物理指標 | 映射語義標籤 | 說明 |
| :--- | :--- | :--- |
| 高特徵中心度 | `ROLE:CORE` | 系統邏輯重心 |
| 高入度 (In-degree) | `ROLE:HUB` | 共享組件 / 瓶頸 |
| 高變更熵 (Edit Entropy) | `STATE:VOLATILE` | 不穩定 / 頻繁修改區 |
| 循環依賴 (Cycle) | `WARN:LOOP` | 架構壞味道 (由數學引擎 100% 偵測) |

### 2.2 輸出格式規範
```text
@NODE[path] ROLE:label TYPE:label ACT:[symbols] BOND:label STAT:label
```
*   **特性**: 高熵、低噪音、LLM 注意力機制天然敏感。

---

## 第三章：百萬行級冷啟動策略 (1M+ Scaling Strategy)

### 3.1 譜分群與圖塌陷 (Topological Squeezing)
*   **原理**: 利用拉普拉斯矩陣與譜分群 (Spectral Clustering) 演算法。
*   **實作**: 自動將萬級檔案目錄「塌陷」為具有語義內聚性的分區。
*   **初始化**: 啟動時僅加載全域拓樸骨架 (Skeleton)，Token 成本封頂在 2.5k 內。

### 3.2 按需閃回 (On-demand Flashback)
AI 始終在「低維拓樸空間」導航，僅在執行手術級修改時，由 CSK 即時「解壓」局部原始碼片段。

---

## 第四章：數據治理與資產保護 (Security & Sovereignty)

### 4.1 語義保險箱 (Semantic Vault)
*   **預設**: 本地建置產物 (Local Artifact)，嚴禁無意提交至 Git。
*   **協作**: 支援 **AES-256-GCM** 加密同步。團隊共享金鑰，地圖數據以「加密 Side-channel」形式安全流轉。
*   **模糊化**: 提供特徵向量微擾與符號哈希功能，防止對企業資產的逆向語義攻擊。

---

## 第五章：形式化驗證與保真度基準 (Formal Verification)

為確保 `ccap-kernel` 產出的地圖具備「工業級可信任度」，系統必須通過以下三重驗證：

### 5.1 物理層：SCIP 等價性校驗 (SCIP Parity)
*   **真值來源**: Sourcegraph 官方 SCIP 索引格式。
*   **驗證屬性**: 
    *   **FQN 唯一性**: 符號生成的 `scip-id` 必須符合 `Scheme + Package + Descriptors` 規範，且在全域符號表 (Global Symbol Table) 中無碰撞。
    *   **圖同構 (Isomorphism)**: 透過 SCIP 引用鏈建立的 `BOND` 圖，必須與原始編譯器 (LSP) 的 AST 調用圖完全同構。
*   **指標**: $Precision_{SCIP} = 1.0$, $Recall_{SCIP} \ge 0.98$。

### 5.2 數學層：拓樸特徵保全 (Topological Preservation)
*   **驗證方法**: 代數連通度比對 (Algebraic Connectivity)。
*   **指標**: $\Delta \lambda_2 \le 0.05$。
*   **目標**: 證明「語義室」塌陷後的圖譜，其譜間隙 (Spectral Gap) 與原始碼結構保持一致，確保壓縮不遺失架構層級的連通資訊。

### 5.3 語義層：高熵電報預測力 (Predictive Precision)
*   **驗證方法**: 零樣本 (Zero-shot) 盲測定位。
*   **指標**: $Recall@3 \ge 0.95$。
*   **目標**: 當 AI 僅讀取 `ST-AAAK` 電報時，鎖定特定功能模組（如 OAuth2 Flow）的正確率必須達到工業級水準。

---
**[SPECIFICATION END]**

