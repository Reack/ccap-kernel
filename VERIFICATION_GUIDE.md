# CCAP-Kernel Formal Verification Guide

This guide describes how to verify the topological accuracy and semantic uniqueness of a project map using the `ccap-kernel verify` tool.

## 1. Fast Grammar Verification (保底層)

The default verification uses **Tree-sitter** for a zero-config, ultra-fast scan.

```bash
ccap-kernel verify <project_path>
```

### Understanding the Report
*   **Semantic Confidence**: A percentage representing how unique your symbols are. Grammar scanning typically achieves **90-95%** on large Python/JS projects.
*   **Symbol Ambiguities**: Lists symbols that have duplicate IDs. This happens when the parser cannot distinguish between siblings or inner classes.
*   **Algebraic Connectivity**: Measures how well-connected your project graph is. Values > 0 are healthy.

---

## 2. Formal Semantic Verification (精確層)

To achieve **100% confidence** and eliminate all ambiguities, you must inject a formal semantic index (SCIP).

### Step 1: Generate a SCIP index
Use the recommended tool for your language:
*   **Python**: `pip install scip-python && scip-python index`
*   **C/C++**: Use `scip-clang`
*   **Go**: `scip-go`

### Step 2: Inject and Verify
```bash
ccap-kernel verify <project_path> --scip index.scip
```

---

## 3. Tiered Precision Repair Guide

If your verification fails (IDs FAILED), follow these human-centric suggestions:

| Extension | Recommendation | Reason |
| :--- | :--- | :--- |
| `.py` | Install `scip-python` | Resolves complex decorator and inner class nesting. |
| `.ts/.js` | Install `scip-typescript` | Resolves cross-file exports and type aliases. |
| `.cpp/.c` | Install `scip-clang` | Resolves macros and template overloading. |

---

## 4. 0-Token Navigation Test (0-Token 盲測)

To truly verify the "0-Token" promise:
1.  Initialize your project: `ccap-kernel init .`
2.  Provide ONLY the `.ccap/maps/root.st.aaak` file to an AI agent.
3.  Ask: *"Where is the core initialization logic for the database?"*
4.  Success: If the AI identifies the correct high-gravity symbol without reading any `.rs` or `.py` files.
