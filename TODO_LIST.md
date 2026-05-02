# CCAP-Kernel 開發任務列表 (TODO)

## Phase 1: 核心引擎骨架 (Foundation)
- [x] 初始化 Cargo 專案架構 (Rust)
- [x] 整合 Tree-sitter 核心與多語言語法包
- [x] 實作基礎 AST 特徵提取器 (Feature Extractor) - **[Robust Version Done]**
- [x] 實作 ST-AAAK 語義電報映射邏輯 (Deterministic Mapping)
- [ ] 實作 CLI 基礎指令 (skeleton, brief)


## Phase 2: 數學導航與拓樸 (Math & Topology)
- [x] 整合 nalgebra 進行線性代數運算
- [x] 實作全專案依賴圖 (DAG) 生成引擎
- [x] 實作「譜分群」圖塌陷演算法 (Topological Squeezing)
- [ ] 實作依賴權重 (BOND) 計算公式


## Phase 3: 影子系統與安全性 (Storage & Security)
- [ ] 實作 `.ccap` 影子目錄管理邏輯 (VNM/SCA 分離)
- [ ] 實作 AES-256-GCM 語義加密與 Side-channel 同步
- [ ] 實作符號混淆與特徵擾動 (Obfuscation)

## Phase 4: 多語言 SDK 與 分發
- [ ] 實作 PyO3 Python 綁定
- [ ] 實作 N-API Node.js 綁定
- [ ] 建立自動化 Benchmark 測試平台
- [ ] 發佈至 PyPI 與 npm (Pre-release)
