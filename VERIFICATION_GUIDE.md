# CCAP V0.0.7 終極驗證指南 (Verification Guide)

本指南旨在引導使用者（或代理 AI）如何完整驗證 **CCAP (Cognitive Continuity and Autonomous Proactivity Protocol)** 的核心效能與物理保真度。

---

## 🚀 核心驗證矩陣 (V0.0.7)

### 1. 物理冷啟動與拓樸生成
*   **指令**: `ccap-kernel init <project_path>`
*   **驗證點**: 是否在 5 秒內完成百萬行級別的掃描，並在 `.ccap/maps/` 產出 `root.st.aaak`。

### 2. 120x Token 節省率審計
*   **指令**: `ccap-kernel stats <project_path>`
*   **驗證點**: 觀察物理位元組節省率。預期在大型 C/Python 專案中達到 90% 以上的 Token 減免。

### 3. 三層階層式互動 Wiki
*   **指令**: `ccap-kernel wiki <project_path> --html`
*   **驗證動作**:
    *   **Level 1**: 點擊琥珀綠戰略大球。
    *   **Level 2**: 進入區域網格，點擊檔案小球。
    *   **Level 3**: 檢視檔案內部的 SCIP 符號座標與複雜度。
    *   **返回**: 點擊左上角「← 返回專案總覽」確保導航狀態正確回歸。

### 4. 物理證明套件 (Three Axioms)
*   **指令**: `ccap-kernel prove <project_path>`
*   **驗證點**: 是否通過「代幣位移」、「同構正確性」與「認知熵減」三大公理測試。

### 5. 手術級 Patch 與差異電報
*   **指令**: `ccap-kernel patch <path> <file> <scip_id> --code "<new_code>"`
*   **驗證點**: 觀察是否產出 **[Δ] VECTOR** 偏移報告與符號增減清單。

---

## 🛡️ 專家級安全性驗證 (Red-Team)

1.  **零執行測試**: 對包含惡意指令的檔案執行 `init`，確認 `security_leak.txt` 不會被建立。
2.  **加密秘密性**: 啟用 `--key` 後，確認 `.enc` 檔案內容為不可讀之二進位噪音。

---
**[GUIDE END - VERSION v0.0.7]**
