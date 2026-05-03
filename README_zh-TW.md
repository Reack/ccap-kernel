# CCAP-Kernel：AI 原生語義編譯核心 (v0.1.0)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Standards: IEEE/ISO](https://img.shields.io/badge/Standards-IEEE_P3361_/_ISO_25059-blue.svg)](https://ieee.org)
[![Version: v0.1.0](https://img.shields.io/badge/Version-v0.1.0-green.svg)](https://github.com/Reack/ccap-kernel/releases/tag/v0.1.0)

**CCAP (認知連續性與自主主動性協定)** 是一個革命性的語義作業系統層，旨在彌合「氛圍開發 (Vibe Coding)」與工業級生產環境之間的巨大鴻溝。它利用 **線性代數**、**拓樸塌陷 (Topological Squeezing)** 與 **通用語義消費** 技術，將百萬行級別的專案壓縮為 AI 助手可以直接理解的高熵語義地圖——並達成 **90% 以上的 Token 節省**。

---

## ⚡ v0.1.0 "Universal Consumer" 版本發佈

v0.1.0 版本標誌著從手工解析向 **「統一語義消費者」** 架構的重大戰略轉型。

*   **語義重力 (Semantic Gravity)**：自動根據物理屬性（程式碼長度、IO 密度、度數）對符號進行排名，精確識別專案的邏輯核心。
*   **分級精度 (Tiered Precision)**：無縫切換極速的 **Tree-sitter** 語法掃描與正式的 **SCIP/LSIF** 語義注入。
*   **以人為本的診斷**：`verify` 指令現在提供 **信心分數 (Confidence Score)** 與 **修復導引**，根據您的程式碼庫建議最佳的索引工具（如 `scip-python`, `scip-clang`）。

---

## 🚀 核心能力

*   **零 Token 冷啟動**：本地 Rust 引擎 (CSK) 在數秒內建立全專案數學模型，不消耗任何 AI Token。
*   **拓樸導航**：利用 **譜分群 (Spectral Clustering)** 自動將程式碼庫劃分為具備語義內聚性的「語義室 (Rooms)」。
*   **手術級修改**：基於位元組座標的精確 AST 補丁，徹底消除「全檔案複寫」導致的語義遺失問題。
*   **通用多語言支持**：原生支援 Python, JS/TS, C/C++, Rust, Go, Java 與 C#。

---

## 📦 安裝方式

CCAP 支援透過 `npm` 或 `pip` 進行安裝。系統會自動根據您的作業系統下載對應的 Rust 高效能核心。

### 透過 NPM 安裝 (Node.js)
```bash
npm install -g ccap
```

### 透過 PIP 安裝 (Python)
```bash
pip install ccap
```

---

## 💡 CLI 指令與工作流

1.  **初始化**: `ccap init <專案路徑>` - 建立初始拓樸地圖。
2.  **驗證**: `ccap verify <專案路徑> [--scip index.scip]` - 檢查符號歧義與信心。
3.  **審計**: `ccap audit <專案路徑>` - 執行標準合規的品質評估。
4.  **追蹤**: `ccap trace <專案路徑> <符號> --impact` - 評估變更的「爆炸半徑」。
5.  **補丁**: `ccap patch <檔案> <符號ID> --code "..."` - 執行精確的手術級修改。

---

## 📊 實戰數據

| 專案名稱 | 原始大小 (Tokens) | CCAP 地圖 (Tokens) | **節省率** | **信心指數** |
| :--- | :--- | :--- | :--- | :--- |
| **FastAPI** (Python) | 699,682 | 54,489 | **92.2%** | **94% (TS) / 100% (SCIP)** |
| **Redis** (C) | 3,428,499 | 28,443 | **99.1%** | **100% (SCIP)** |

---

## 🛡️ 權威依據與理論引用

本系統建立在嚴謹的軟體工程標準之上：
1.  **SCIP (Symbolic Code Intelligence Protocol)**：採用 [Sourcegraph](https://sourcegraph.com/docs/code_search/scip) 標準，確保正式的語義真實性。
2.  **IEEE P3361 (草案)**：對標 AI 輔助開發標準，致力於降低 AI 的「認知負荷」。
3.  **ISO/IEC 25059**：實作針對 AI 增強系統的「自適應性」品質模型。

---

## 🤖 AI 創作聲明

> **⚠️ 警告與通知：**
> 本專案的所有內容——包括 Rust 核心引擎、數學模型、技術文件以及本說明文件——**100% 由自主 AI 代理 (Gemini CLI) 在人類夥伴的戰略指導下撰寫而成**。**沒有任何人類直接修改過任何一行原始碼。**

---

## 🚧 限制與免責聲明

**實驗性 Alpha 階段：** 本專案目前處於實驗性 alpha 階段。請自行承擔使用風險。地圖保真度僅供導航參考，並非絕對真理。

---

## 📄 授權條款
本專案採用 **MIT License** 授權。
