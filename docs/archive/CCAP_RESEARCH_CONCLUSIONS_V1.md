# CCAP 研究結論 V2: 數據與工具的深度解構

## 1. 導航模式的典範轉移 (Paradigm Shift in Navigation)
*   **結論**: **「工具過濾」優於「直接閱讀」**。
*   **新協定**: AI 不應直接讀取 `_MAP.md` 或 `_MAP.meta.json`。應透過 `ccap_navigator` 獲取融合後的資訊。
*   **效果**: Token 密度提升 3-5 倍，消除 90% 的 Markdown 語法噪音。

## 2. 物理與語義的分層存儲 (Tiered Storage)
*   **架構**:
    *   **SCA (語義)**: `.md` 檔案，儲存人類可讀的目的與 Gotchas。
    *   **VNM (物理)**: `.json` 檔案，儲存機器可讀的 Hashes, Symbols 與依賴權重。
*   **整合**: 兩者在影子目錄 (Shadow Directory) 中以 Sidecar 模式共存。

## 3. 符號級精確度與關聯強度
*   **方案**: 透過 LSP/Tree-sitter 在背景預計算符號連結與引用權重。
*   **應用**: AI 透過地圖標籤 (如 `⭐️⭐️⭐️`) 識別核心模組，並透過 `lookup` 指令實現「手術刀式」的精確跳轉。

## 4. 跨平台通用診斷 (Universal DRP)
*   **協定**: 診斷工具必須輸出統一的 `CCAP_DRP_V1` JSON 格式。
*   **適配**: 透過 Python 封裝 OS 底層差異，確保 AI 的修復大腦在 Win/Mac/Linux 上獲得完全一致的輸入。

---

## 5. 待決議：符號與權重數據源 (Data Sourcing Decisions)
在實作 `ccap_navigator` 前，需確定物理數據（符號、依賴、行號）的獲取路徑：
*   **方案 A (依賴外部工具)**: 呼叫環境已有的 `ctags`, `pyright`, `tsc`。
    *   *優點*: 數據最準確。 *缺點*: 環境依賴強，部署複雜。
*   **方案 B (輕量級內建解析)**: 在 Python 中整合 `Tree-sitter`。
    *   *優點*: 跨平台零依賴，速度極快，能自主產出 JSON。 *缺點*: 需針對不同語言維護語法檔。
*   **方案 C (AI 自我標註)**: 在 AI 總結地圖時同步輸出關鍵符號。
    *   *優點*: 實現最簡單。 *缺點*: 精度較低，且增加地圖生成時的 Token 消耗。

