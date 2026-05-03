# CCAP 語義重力與多級精度架構 (Semantic Gravity & Tiered Precision)

**宗旨**: 透過物理指標自動識別代碼核心，並保留專業工作者手動精確化的彈性。

## 1. 重力運算層 (Gravity Layer - 預設、高效)
利用 Rust 核心計算每個符號的 **「物理特徵向量 (Physical Feature Vector)」**，自動判定權重。

### 1.1 權重公式 (Gravity Score)
每個符號的重力 $G$ 由以下指標組成：
*   **$D$ (Degree)**: 入度與出度（被調用與調用他人）。越多代表權重越高。
*   **$C$ (Complexity)**: 循環複雜度（Control Flow）。越複雜代表邏輯密度越高。
*   **$I$ (IO Density)**: 關鍵字密度（如 `sql`, `api`, `fetch`）。判定是否為系統邊界。
*   **$L$ (Location)**: 在目錄樹中的深度。根部通常具備全局性。

### 1.2 高熵區標註 (High Entropy Zones)
當 Tree-sitter 無法區分同名符號時，標註為 `[AMBIGUOUS_GRAVITY]`。這不是錯誤，而是一個「待觀察區」。

---

## 2. 專業工作者的精確化彈性 (Precision Tiers)
為了保留專業使用者的控制權，我們設計了三種「升級路徑」：

### 2.1 語義錨點 (Manual Anchoring)
使用者可以透過註解或命令列將特定符號標記為 **「Archetype」**。
*   **指令**: `ccap-kernel anchor <symbol_id> --as GATEWAY`
*   **效果**: 強制將該符號的重力設為 1.0，無視自動運算結果。

### 2.2 按需升級 (On-demand Promotion)
系統平時以重力圖運作。當使用者需要進行「手術級」修改時，可以針對特定文件或模組進行「語義升級」。
*   **指令**: `ccap-kernel promote <path>`
*   **彈性**: 此時系統會提示使用者提供 SCIP 文件，將該路徑下的符號從「模糊重力」提升為「形式化精確」。

### 2.3 外部真相注入 (Formal Truth Injection)
完美相容專業索引器。
*   **支援**: 讀取 `.scip`, `.lsif` 或 `srctx` 的輸出。
*   **合併邏輯**: 當外部索引存在時，`ccap-kernel` 會執行 **「語義對齊（Semantic Alignment）」**。
    *   以外部索引的 ID 為唯一事實。
    *   以重力運算的權重為 AI 的閱讀優先級。
    *   **成果**: 既精確（無碰撞）又具備權重（懂重點）。

---

## 3. 驗證與回饋機制 (Feedback Loop)

### 3.1 信心指數 (Confidence Index)
產出的 Map 會包含一個 `confidence` 欄位：
*   `confidence: 0.7` (Tree-sitter 掃描 + 重力運算)
*   `confidence: 1.0` (SCIP 已注入，已消除所有碰撞)

### 3.2 0-Token 導航驗證
我們如何驗證這有效？
1.  **盲測**: 讓 AI 在不讀代碼、只讀 Map 的透過，嘗試回答「這個 API 的核心校驗邏輯在哪裡？」
2.  **命中率**: 若 AI 優先選擇了高重力節點，且該節點確實是核心，則驗證成功。

---

## 4. 戰略總結：專業工作者的獲得感

*   **新手/快速場景**：0 配置，拿起來就掃，重力圖幫你指出重點，碰撞區會報警提醒。
*   **專家/複雜場景**：
    *   你可以透過 `ccap-kernel verify` 看到地圖的弱點。
    *   你可以選擇性地在關鍵模組跑 `scip-python` 並注入。
    *   你依然享有 ccap-kernel 帶來的 **100x 壓縮率**（SCIP 太大了，AI 讀不動，我們幫你壓縮）。
