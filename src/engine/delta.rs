use crate::engine::extractor::FileFeatures;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhysicalDelta {
    pub lost_symbols: Vec<String>,
    pub gained_symbols: Vec<String>,
    pub link_shift: Vec<String>, // "target: +1" or "target: -1"
    pub vector_shift: [f32; 3], // [ΔC, ΔD, ΔI]
}

pub struct DeltaEngine;

impl DeltaEngine {
    /// Calculates the deterministic physical difference between two snapshots.
    pub fn calculate_delta(old: &FileFeatures, new: &FileFeatures) -> PhysicalDelta {
        let mut delta = PhysicalDelta::default();

        // 1. Symbol G/L (Based on SCIP IDs)
        let old_ids: std::collections::HashSet<_> = old.exports.iter().map(|s| &s.id).collect();
        let new_ids: std::collections::HashSet<_> = new.exports.iter().map(|s| &s.id).collect();

        for id in old_ids.difference(&new_ids) {
            delta.lost_symbols.push(id.to_string());
        }
        for id in new_ids.difference(&old_ids) {
            delta.gained_symbols.push(id.to_string());
        }

        // 2. Link Momentum (Coupling Delta)
        let old_refs: std::collections::HashSet<_> = old.references.iter().collect();
        let new_refs: std::collections::HashSet<_> = new.references.iter().collect();

        for r in old_refs.difference(&new_refs) {
            delta.link_shift.push(format!("{}: -1", r));
        }
        for r in new_refs.difference(&old_refs) {
            delta.link_shift.push(format!("{}: +1", r));
        }

        // 3. Feature Vector Shift
        delta.vector_shift = [
            new.control_flow_score - old.control_flow_score,
            new.data_density_score - old.data_density_score,
            new.io_density_score - old.io_density_score,
        ];

        delta
    }

    pub fn print_telegram(delta: &PhysicalDelta) {
        println!("\n📊  CCAP PHYSICAL DELTA TELEGRAM");
        println!("----------------------------------------------------");
        for s in &delta.lost_symbols { println!("  [-] LOST:   {}", s); }
        for s in &delta.gained_symbols { println!("  [+] GAINED: {}", s); }
        for l in &delta.link_shift { println!("  [~] BOND:   {}", l); }
        println!("  [Δ] VECTOR: [C:{:+.2}, D:{:+.2}, I:{:+.2}]", 
            delta.vector_shift[0], delta.vector_shift[1], delta.vector_shift[2]);
        println!("----------------------------------------------------\n");
    }
}
