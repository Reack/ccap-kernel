use serde::{Serialize, Deserialize};
use crate::engine::extractor::ScipSymbol;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaReport {
    pub lost_symbols: Vec<String>,
    pub gained_symbols: Vec<String>,
    pub link_shift: f32,
    pub vector_shift: String,
}

pub struct DeltaEngine;

impl DeltaEngine {
    pub fn calculate_delta(old_symbols: &[ScipSymbol], new_symbols: &[ScipSymbol]) -> DeltaReport {
        let old_set: HashSet<_> = old_symbols.iter().map(|s| &s.id).collect();
        let new_set: HashSet<_> = new_symbols.iter().map(|s| &s.id).collect();

        let lost: Vec<_> = old_set.difference(&new_set).map(|s| (*s).clone()).collect();
        let gained: Vec<_> = new_set.difference(&old_set).map(|s| (*s).clone()).collect();

        let shift = if old_symbols.is_empty() { 0.0 } else { (lost.len() + gained.len()) as f32 / old_symbols.len() as f32 };

        DeltaReport {
            lost_symbols: lost,
            gained_symbols: gained,
            link_shift: shift,
            vector_shift: if shift > 0.5 { "CRITICAL" } else if shift > 0.1 { "MODERATE" } else { "STABLE" }.to_string(),
        }
    }

    pub fn print_telegram(report: &DeltaReport) {
        println!("📡  [Δ] VECTOR: {}", report.vector_shift);
        if !report.lost_symbols.is_empty() {
            println!("    [-] LOST: {:?}", report.lost_symbols);
        }
        if !report.gained_symbols.is_empty() {
            println!("    [+] GAINED: {:?}", report.gained_symbols);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::extractor::ScipSymbol;

    #[test]
    fn test_delta_stable() {
        let syms = vec![ScipSymbol { id: "a".to_string(), name: "a".to_string(), line: 1, range: (0, 10) }];
        let report = DeltaEngine::calculate_delta(&syms, &syms);
        assert_eq!(report.lost_symbols.len(), 0);
        assert_eq!(report.gained_symbols.len(), 0);
        assert_eq!(report.vector_shift, "STABLE");
    }

    #[test]
    fn test_delta_critical() {
        let old_syms = vec![ScipSymbol { id: "a".to_string(), name: "a".to_string(), line: 1, range: (0, 10) }];
        let new_syms = vec![ScipSymbol { id: "b".to_string(), name: "b".to_string(), line: 1, range: (0, 10) }];
        let report = DeltaEngine::calculate_delta(&old_syms, &new_syms);
        assert_eq!(report.lost_symbols[0], "a");
        assert_eq!(report.gained_symbols[0], "b");
        assert_eq!(report.vector_shift, "CRITICAL");
    }
}
