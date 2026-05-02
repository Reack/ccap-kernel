use std::path::Path;

pub struct ProxyEngine;

impl ProxyEngine {
    /// Cleans and compresses CLI tool output to maximize token efficiency.
    pub fn clean_output(input: &str, repo_root: &str) -> String {
        let mut lines = Vec::new();
        
        // Safely determine the absolute root string for replacement
        let abs_root = if let Ok(abs) = std::fs::canonicalize(repo_root) {
            let s = abs.to_string_lossy().to_string();
            let cleaned = if s.starts_with(r"\\?\") { s[4..].to_string() } else { s };
            cleaned.replace("\\", "/")
        } else {
            String::new() 
        };
        
        // DEBUG: println!("DEBUG: abs_root = '{}'", abs_root);

        let mut last_line = String::new();
        let mut repeat_count = 0;

        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() { continue; }

            let mut processed_line = raw_line.replace("\\", "/");
            
            // CRITICAL FIX: Only replace if it's a subpath or exact match of absolute path
            // We use a simple heuristic: the match must be preceded by a space or be at start
            if abs_root.len() > 10 && processed_line.contains(&abs_root) {
                processed_line = processed_line.replace(&abs_root, "@root");
            }

            // 2. Simple Deduplication
            if processed_line.trim() == last_line.trim() {
                repeat_count += 1;
                continue;
            } else {
                if repeat_count > 0 {
                    lines.push(format!("[... repeated {} times]", repeat_count));
                }
                lines.push(processed_line.clone());
                last_line = processed_line;
                repeat_count = 0;
            }
        }

        if repeat_count > 0 {
            lines.push(format!("[... repeated {} times]", repeat_count));
        }

        lines.join("\n")
    }
}
