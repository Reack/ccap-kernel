use std::fs;
use std::path::Path;
use crate::engine::extractor::ScipSymbol;

pub struct Patcher;

impl Patcher {
    /// Surgically replaces a code block defined by a symbol's byte range.
    pub fn apply_patch(
        repo_root: &str,
        file_rel_path: &str,
        symbol: &ScipSymbol,
        new_code: &str,
    ) -> anyhow::Result<()> {
        let abs_path = Path::new(repo_root).join(file_rel_path);
        let mut content = fs::read_to_string(&abs_path)?;

        let (start, end) = symbol.range;
        if start > content.len() || end > content.len() || start > end {
            return Err(anyhow::anyhow!("Invalid byte range for patching. File might have shifted."));
        }

        // Surgical byte-level replacement
        content.replace_range(start..end, new_code);

        // Atomic-like write back to original file
        fs::write(&abs_path, content)?;
        
        println!("🩹  Surgical Patch applied to: {} @ {}", file_rel_path, symbol.id);
        Ok(())
    }
}
