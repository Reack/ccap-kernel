pub struct ProxyEngine;

impl ProxyEngine {
    /// production-grade cleaning and compression of CLI output.
    pub fn clean_output(input: &str, repo_root: &str) -> String {
        let mut lines = Vec::new();
        
        let abs_root = if let Ok(abs) = std::fs::canonicalize(repo_root) {
            let s = abs.to_string_lossy().to_string();
            if s.starts_with(r"\\?\") { s[4..].to_string() } else { s }.replace("\\", "/")
        } else {
            repo_root.replace("\\", "/")
        };
        
        let mut last_line = String::new();
        let mut repeat_count = 0;

        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() { continue; }

            let processed_line = raw_line.replace("\\", "/").replace(&abs_root, "@root");

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
