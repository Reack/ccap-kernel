use crate::engine::extractor::FileFeatures;
use crate::engine::math::MathEngine;
use regex::Regex;
use std::collections::HashSet;

pub struct Verifier;

#[derive(Debug)]
pub struct VerificationReport {
    pub scip_parity_passed: bool,
    pub symbol_collisions: usize,
    pub format_errors: Vec<String>,
    pub algebraic_connectivity: f32,
}

impl Verifier {
    pub fn verify_scip_ids(analysis_results: &[(String, FileFeatures)]) -> VerificationReport {
        let mut report = VerificationReport {
            scip_parity_passed: true,
            symbol_collisions: 0,
            format_errors: Vec::new(),
            algebraic_connectivity: 0.0,
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

    pub fn calculate_fidelity(n: usize, edges: &[(usize, usize, f32)]) -> f32 {
        MathEngine::compute_fiedler_value(n, edges)
    }
}
