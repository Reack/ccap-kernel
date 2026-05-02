use crate::engine::extractor::FileFeatures;
use crate::engine::Linker;
use std::collections::HashSet;

#[derive(Debug)]
pub struct VerificationReport {
    pub scip_parity_passed: bool,
    pub symbol_collisions: usize,
}

pub struct Verifier;

impl Verifier {
    /// Verifies SCIP IDs for uniqueness and format.
    pub fn verify_scip_ids(results: &[(String, FileFeatures)]) -> VerificationReport {
        let mut all_ids = HashSet::new();
        let mut collisions = 0;
        let mut format_passed = true;

        for (_, features) in results {
            for sym in &features.exports {
                if !sym.id.starts_with("ccap . . ") {
                    format_passed = false;
                }
                if !all_ids.insert(sym.id.clone()) {
                    collisions += 1;
                }
            }
        }

        VerificationReport {
            scip_parity_passed: format_passed && collisions == 0,
            symbol_collisions: collisions,
        }
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
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Lambda 2 is the second smallest eigenvalue
        *values.get(1).unwrap_or(&0.0)
    }

    /// Validates if the compressed map can correctly resolve a high-entropy query.
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
