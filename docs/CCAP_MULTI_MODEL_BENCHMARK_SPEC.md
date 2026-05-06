# CCAP 跨模型語義壓縮評測規範 (Multi-Model Benchmark Specification)

## 1. 核心願景
為了解決 AI 協作中「Token 成本黑盒」的問題，`ccap-kernel` v0.2.0 將引入跨模型的本地模擬計數器。這能讓使用者在不產生任何 API 費用的情況下，獲得針對不同主流 LLM 的精確節省預估。

## 2. 評測模型對象與工具
本系統針對以下三個目前最主流的 AI 開發模型進行數據適配：

1.  **OpenAI 系列 (GPT-4o/o1)**: 
    *   分詞器：`cl100k_base` (via `tiktoken-rs`)。
    *   特性：對程式碼結構非常敏感，ST-AAAK 高熵電報的最佳載體。
2.  **Anthropic 系列 (Claude 3.5 Sonnet)**:
    *   分詞器：`claude-tokenizer` (via HuggingFace config)。
    *   特性：偏好結構化描述（如 XML）。評測將包含「格式膨脹 vs. 理解深度」的權衡分析。
3.  **Google 系列 (Gemini 1.5 Pro/Flash)**:
    *   分詞器：`sentencepiece` 基礎配置。
    *   特性：超長 Context Window 支持。評測重點在於大規模地圖下的「認知雜訊比」。

## 3. 實作原則：誠實的估計 (The Honesty Principle)

> **[DISCLAIMER]**
> 本工具產出的 Token 數據屬於「科學估計」。由於 LLM 供應商可能會：
> 1. 動態更新其分詞演算法。
> 2. 在系統提示詞（System Prompts）中加入隱形權重。
> 3. 對特定格式進行後台優化。
> 
> 因此，`ccap-kernel` 的評測數據不代表 100% 的最終帳單，僅作為架構優化與預算規劃的參考。

## 4. 關鍵指標 (Key Performance Indicators)
*   **ST-Gain (Spectral Telegram Gain)**: 原始碼體積 vs. 譜地圖體積。
*   **Format Overhead (格式開銷)**: ST-AAAK 轉為 XML/JSON 後的 Token 增加比例。
*   **Decoupling Score**: 模型對隱式依賴的感知偏差。

## 5. 指令設計
*   `ccap stats --compare`: 輸出全模型對比矩陣。
*   `ccap export --flavor [gemini|claude|openai]`: 針對特定模型輸出最優化的電報格式。
