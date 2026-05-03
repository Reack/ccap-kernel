use crate::engine::extractor::FileFeatures;
use crate::engine::Linker;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VerificationReport {
    pub scip_parity_passed: bool,
    pub symbol_collisions: usize,
    pub format_passed: bool,
    pub offending_ids: Vec<String>,
    pub collision_details: Vec<String>,
    pub recommended_tools: Vec<String>,
    pub confidence_score: f32,
}

pub struct Verifier;

impl Verifier {
    /// Verifies SCIP IDs for uniqueness and format.
    pub fn verify_scip_ids(repo_root: &str, results: &[(String, FileFeatures)]) -> VerificationReport {
        let mut all_ids = std::collections::HashMap::new();
        let mut collisions = 0;
        let mut format_passed = true;
        let mut offending_ids = Vec::new();
        let mut collision_details = Vec::new();
        let mut conflicting_exts = std::collections::HashSet::new();

        for (path, features) in results {
            for sym in &features.exports {
                if !sym.id.starts_with("ccap . . ") {
                    format_passed = false;
                    offending_ids.push(format!("Format Failure in {}: ID=[{}]", path, sym.id));
                }
                
                if let Some(existing_path) = all_ids.insert(sym.id.clone(), (path.clone(), sym.line, sym.gravity)) {
                    collisions += 1;
                    
                    if let Some(ext) = std::path::Path::new(path).extension().and_then(|s| s.to_str()) {
                        conflicting_exts.insert(ext.to_string());
                    }

                    let full_path = std::path::Path::new(repo_root).join(path);
                    let snippet = if let Ok(code) = std::fs::read_to_string(full_path) {
                        code.lines().nth(sym.line.saturating_sub(1)).unwrap_or("").trim().to_string()
                    } else {
                        "N/A".to_string()
                    };
                    
                    // Highlight gravity in details
                    collision_details.push(format!(
                        "Ambiguity: {} | G:{:.2} vs G:{:.2} | Used in [{}:{}] and [{}:{}] -> Snippet: [{}]", 
                        sym.id, existing_path.2, sym.gravity, existing_path.0, existing_path.1, path, sym.line, snippet
                    ));
                }
            }
        }

        let mut recommended_tools = Vec::new();
        let guide = Self::get_guide_map();
        for ext in conflicting_exts {
            if let Some(tool_info) = guide.get(&ext) {
                recommended_tools.push(format!(".{} -> {}", ext, tool_info));
            }
        }

        // Confidence starts high, drops with collisions
        let mut confidence = 1.0;
        if collisions > 0 {
            confidence = (1.0 - (collisions as f32 / 100.0)).max(0.1);
        }

        VerificationReport {
            scip_parity_passed: format_passed && collisions == 0,
            symbol_collisions: collisions,
            format_passed,
            offending_ids,
            collision_details,
            recommended_tools,
            confidence_score: confidence,
        }
    }

    fn get_guide_map() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("py".to_string(), "scip-python (pip install scip-python)".to_string());
        m.insert("ts".to_string(), "scip-typescript (npm install -g @sourcegraph/scip-typescript)".to_string());
        m.insert("js".to_string(), "scip-typescript (npm install -g @sourcegraph/scip-typescript)".to_string());
        m.insert("go".to_string(), "scip-go (go install github.com/sourcegraph/scip-go/cmd/scip-go@latest)".to_string());
        m.insert("rs".to_string(), "scip-rust (cargo install scip-rust)".to_string());
        m.insert("cpp".to_string(), "scip-clang (https://github.com/sourcegraph/scip-clang)".to_string());
        m.insert("c".to_string(), "scip-clang (https://github.com/sourcegraph/scip-clang)".to_string());
        m
    }

    /// Measures the structural fidelity using Algebraic Connectivity (lambda 2).
    pub fn calculate_fidelity(node_count: usize, edges: &[(usize, usize, f32)]) -> f32 {
        if node_count < 2 { return 1.0; }
        
        let mut laplacian: nalgebra::DMatrix<f32> = nalgebra::DMatrix::zeros(node_count, node_count);
        for &(u, v, w) in edges {
            if u >= node_count || v >= node_count { continue; }
            laplacian[(u, u)] += w;
            laplacian[(v, v)] += w;
            laplacian[(u, v)] -= w;
            laplacian[(v, u)] -= w;
        }

        let eig = laplacian.symmetric_eigen();
        let mut values: Vec<f32> = eig.eigenvalues.iter().cloned().collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        *values.get(1).unwrap_or(&0.0)
    }

    pub fn verify_topological_accuracy(
        linker: &Linker,
        results: &[(String, FileFeatures)],
    ) -> f32 {
        let mut correct_resolutions = 0;
        let total_tests = results.len().min(20);

        for (path, _) in results.iter().take(total_tests) {
            if let Some(_) = linker.calculate_impact(path) {
                correct_resolutions += 1;
            }
        }

        if total_tests > 0 {
            correct_resolutions as f32 / total_tests as f32
        } else {
            1.0
        }
    }
}
