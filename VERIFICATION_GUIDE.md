# CCAP V4.0 實戰驗證指南 (Verification Guide)

歡迎驗證 CCAP V4.0 核心引擎。請依照以下步驟執行：

## 1. 準備環境
*   確保系統安裝了 **Rust 1.70+** (cargo)。
*   支援 Windows, macOS, 與 Linux。

## 2. 編譯核心
```bash
cd ccap-kernel
cargo build --release
```
編譯產物將位於 `target/release/ccap-kernel`。

## 3. 執行實戰驗證 (以 FastAPI 為例)
1.  **下載目標專案**: `git clone --depth 1 https://github.com/fastapi/fastapi.git`
2.  **執行初始化同步**:
    `./target/release/ccap-kernel init <fastapi_path>`
    *(這將在 10 秒內完成 1,000 個檔案的數學建模)*
3.  **執行綜合 Benchmark**:
    `./target/release/ccap-kernel benchmark <fastapi_path>`
    *(驗證 Token 節省率、SCIP 合規度與連通度指標)*
4.  **按需展開房間**:
    `./target/release/ccap-kernel inspect-room <fastapi_path> <room_name>`
    *(驗證 AI 是否能精確調閱細節)*

## 4. 驗證安全性 (加密會話)
```bash
./target/release/ccap-kernel init <any_project_path> --key "your-key"
```
檢查 `.ccap/maps/` 下是否產出了 `.enc` 加密檔案，證明語義資產受到 AES-256 保護。
