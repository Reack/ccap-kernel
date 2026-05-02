use std::fs;
use std::path::Path;
use crate::engine::extractor::ScipSymbol;

pub struct Patcher;

impl Patcher {
    /// Surgically replaces a code block and reports the physical delta.
    pub fn apply_patch(
        repo_root: &str,
        file_rel_path: &str,
        symbol: &ScipSymbol,
        new_code: &str,
    ) -> anyhow::Result<()> {
        let abs_path = Path::new(repo_root).join(file_rel_path);
        
        // 1. Capture OLD state for delta analysis
        let mut extractor = crate::engine::Extractor::new();
        let old_features = extractor.analyze_file(abs_path.to_str().unwrap(), file_rel_path)?;

        // 2. Perform Physical Patch
        let mut content = fs::read_to_string(&abs_path)?;
        let (start, end) = symbol.range;
        if start > content.len() || end > content.len() || start > end {
            return Err(anyhow::anyhow!("Invalid byte range for patching. File might have shifted."));
        }
        content.replace_range(start..end, new_code);
        fs::write(&abs_path, &content)?;

        // 3. Capture NEW state
        let new_features = extractor.analyze_file(abs_path.to_str().unwrap(), file_rel_path)?;

        // 4. Report Deterministic Delta
        let delta = crate::engine::DeltaEngine::calculate_delta(&old_features, &new_features);
        crate::engine::DeltaEngine::print_telegram(&delta);
        
        println!("🩹  Surgical Patch applied to: {} @ {}", file_rel_path, symbol.id);
        Ok(())
    }
}
