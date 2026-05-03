use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::HashMap;
use crate::engine::extractor::FileFeatures;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScipIndex {
    pub documents: Vec<ScipDocument>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScipDocument {
    pub relative_path: String,
    pub occurrences: Vec<ScipOccurrence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScipOccurrence {
    pub range: Vec<i32>,
    pub symbol: String,
}

pub struct ScipConsumer;

impl ScipConsumer {
    /// Injects SCIP semantics into existing Tree-sitter results.
    pub fn inject_semantics(scip_path: &str, results: &mut [(String, FileFeatures)]) -> anyhow::Result<()> {
        let content = fs::read_to_string(scip_path)?;
        let index: ScipIndex = serde_json::from_str(&content)?;

        // Map relative path to symbol overrides
        let mut scip_map = HashMap::new();
        for doc in index.documents {
            let mut syms = Vec::new();
            for occ in doc.occurrences {
                if occ.symbol.contains('#') { // Likely a definition
                    syms.push(occ);
                }
            }
            scip_map.insert(doc.relative_path.replace("\\", "/"), syms);
        }

        for (path, features) in results {
            if let Some(scip_syms) = scip_map.get(path) {
                for export in &mut features.exports {
                    // Try to find matching SCIP symbol by line
                    if let Some(matching) = scip_syms.iter().find(|s| s.range[0] as usize + 1 == export.line) {
                        export.id = format!("ccap . . [SCIP]{}", matching.symbol);
                        export.gravity = 1.0; // SCIP verified symbols get max gravity
                    }
                }
            }
        }

        Ok(())
    }
}
