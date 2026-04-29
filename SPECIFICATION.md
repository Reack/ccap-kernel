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

## 第五章：形式化驗證與 Benchmarks

*   **壓縮率 ($C_R$)**: 目標 $\ge 20:1$ (相對於傳統地圖)。
*   **路由精確度**: 在 0 原始碼閱讀下，AI 尋找 Bug 的命中率需高於文字地圖模式。
*   **幻覺抑制**: 透過數學標籤的確定性，消除 90% 以上的無效修改提議。

---
**[SPECIFICATION END]**
