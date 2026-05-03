use ccap_kernel::engine::Patcher;
use ccap_kernel::engine::extractor::{Extractor, ScipSymbol};
use std::fs;
use std::path::Path;

#[test]
fn test_patcher_surgical_precision() {
    let repo_root = "temp_patch_test";
    let file_path = "logic.rs";
    let abs_path = Path::new(repo_root).join(file_path);
    
    // 1. Prepare Mock File
    fs::create_dir_all(repo_root).unwrap();
    let original_code = "fn old_function() { println!(\"Old\"); }\nfn stay() { println!(\"Stay\"); }";
    fs::write(&abs_path, original_code).unwrap();

    // 2. Mock SCIP Symbol for 'old_function'
    // Range: 0 to 39 (covers 'fn old_function() { println!("Old"); }')
    let symbol = ScipSymbol {
        id: "ccap . . logic#old_function#".to_string(),
        name: "old_function".to_string(),
        range: (0, 39),
        ..Default::default()
    };

    // 3. Apply Patch
    let new_code = "fn new_function() { println!(\"New\"); }";
    Patcher::apply_patch(repo_root, file_path, &symbol, new_code).expect("Patch application failed");

    // 4. Verify Physical Content
    let patched_content = fs::read_to_string(&abs_path).unwrap();
    assert!(patched_content.contains("fn new_function()"), "Content must be replaced with new function");
    assert!(patched_content.contains("fn stay()"), "Other parts of the file must remain intact");
    assert!(!patched_content.contains("fn old_function()"), "Old function must be removed");

    // 5. Cleanup
    fs::remove_dir_all(repo_root).unwrap();
}

#[test]
fn test_patcher_out_of_bounds_protection() {
    let repo_root = "temp_bounds_test";
    let file_path = "short.rs";
    let abs_path = Path::new(repo_root).join(file_path);
    
    fs::create_dir_all(repo_root).unwrap();
    fs::write(&abs_path, "short content").unwrap();

    let invalid_symbol = ScipSymbol {
        id: "error".to_string(),
        range: (100, 200), // Far beyond "short content"
        ..Default::default()
    };

    let result = Patcher::apply_patch(repo_root, file_path, &invalid_symbol, "ignored");
    
    assert!(result.is_err(), "Patching out of bounds must fail");
    assert!(result.unwrap_err().to_string().contains("Invalid byte range"), "Error message must be descriptive");

    fs::remove_dir_all(repo_root).unwrap();
}
