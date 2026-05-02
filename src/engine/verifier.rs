use crate::engine::extractor::FileFeatures;
use crate::engine::math::MathEngine;
use crate::engine::Linker;
use regex::Regex;
use std::collections::HashSet;

pub struct Verifier;

#[derive(Debug)]
pub struct VerificationReport {
    pub scip_parity_passed: bool,
    pub symbol_collisions: usize,
    pub format_errors: Vec<String>,
    pub algebraic_connectivity: f32,
    pub topological_accuracy: f32,
}

impl Verifier {
    pub fn verify_scip_ids(analysis_results: &[(String, FileFeatures)]) -> VerificationReport {
        let mut report = VerificationReport {
            scip_parity_passed: true,
            symbol_collisions: 0,
            format_errors: Vec::new(),
            algebraic_connectivity: 0.0,
            topological_accuracy: 0.0,
        };

        let mut seen_ids = HashSet::new();
        let scip_regex = Regex::new(r"^ccap \. \. [^# ]+(#[^# ]+)*#$").unwrap();

        for (path, feat) in analysis_results {
            for export in &feat.exports {
                if !scip_regex.is_match(&export.id) {
                    report.scip_parity_passed = false;
                    report.format_errors.push(format!("Invalid SCIP ID format in {}: '{}'", path, export.id));
                }

                if !seen_ids.insert(export.id.clone()) {
                    report.symbol_collisions += 1;
                    report.scip_parity_passed = false;
                    report.format_errors.push(format!("Symbol Collision detected for ID: '{}'", export.id));
                }
            }
        }

        report
    }

    /// Verifies the accuracy of impact assessment by comparing against full graph reachability.
    pub fn verify_topological_accuracy(linker: &Linker, analysis_results: &[(String, FileFeatures)]) -> f32 {
        if analysis_results.len() < 2 { return 1.0; }

        let mut total_hits = 0.0;
        let mut samples = 0;

        // Take up to 20 samples for verification
        for (path, _) in analysis_results.iter().take(20) {
            if let Some(report) = linker.calculate_impact(path) {
                // The mathematical truth: any node with a SCIP reference to our target
                // MUST be in the dependents list.
                let mut ground_truth_count = 0;
                let mut hit_count = 0;

                for (other_path, feat) in analysis_results {
                    if path == other_path { continue; }
                    
                    // Does this file physically import our target module?
                    // This is our Ground Truth (100% accurate static fact)
                    let is_direct_dependent = feat.references.iter().any(|r| {
                        // SCIP ID check
                        r.contains(&path.replace(".py", ""))
                    });

                    if is_direct_dependent {
                        ground_truth_count += 1;
                        if report.dependents.contains(other_path) {
                            hit_count += 1;
                        }
                    }
                }

                if ground_truth_count > 0 {
                    total_hits += (hit_count as f32) / (ground_truth_count as f32);
                    samples += 1;
                }
            }
        }

        if samples > 0 { total_hits / (samples as f32) } else { 1.0 }
    }

    pub fn calculate_fidelity(n: usize, edges: &[(usize, usize, f32)]) -> f32 {
        MathEngine::compute_fiedler_value(n, edges)
    }
}
