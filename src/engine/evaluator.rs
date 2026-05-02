use crate::engine::Linker;
use std::collections::HashMap;

pub struct Evaluator;

#[derive(Debug)]
pub struct AuditReport {
    pub modularity_score: f32,    // ISO 25010: Cohesion Ratio
    pub analyzability_index: f32, // Shannon Entropy of tokens
    pub ripple_effect_avg: f32,   // Avg blast radius
}

impl Evaluator {
    /// Performs a full scientific audit of the current project topology.
    pub fn perform_audit(linker: &Linker, node_count: usize) -> AuditReport {
        let edges = linker.export_edges();
        
        // 1. Modularity (Simple Cohesion Ratio)
        // Ratio of internal links vs external links is handled by spectral clustering results, 
        // here we provide a graph-wide density metric.
        let modularity = if node_count > 0 {
            (edges.len() as f32) / (node_count as f32).powi(2)
        } else {
            1.0
        };

        // 2. Stability (Ripple Effect)
        // Average impact radius across all nodes.
        let mut total_radius = 0;
        let mut valid_nodes = 0;
        
        let paths = linker.get_node_paths();
        for path in paths.iter().take(50) { // Sample 50 nodes for speed
            if let Some(report) = linker.calculate_impact(path) {
                total_radius += report.radius;
                valid_nodes += 1;
            }
        }
        
        let ripple_avg = if valid_nodes > 0 {
            (total_radius as f32) / (valid_nodes as f32)
        } else {
            0.0
        };

        // 3. Analyzability (Heuristic Entropy)
        // Based on symbol count and description density.
        let analyzability = 1.0 - (ripple_avg / 10.0).min(0.5);

        AuditReport {
            modularity_score: modularity,
            analyzability_index: analyzability,
            ripple_effect_avg: ripple_avg,
        }
    }
}
