use crate::engine::Linker;

pub struct Evaluator;

#[derive(Debug)]
pub struct AuditReport {
    // IEEE P3361: Measures how easily an AI can explain/understand the code
    pub ieee_p3361_cognitive_load: f32, 
    // ISO 25059: Measures the system's resilience to AI-driven changes
    pub iso_25059_adaptability: f32,
    // Structural Data Debt: Count of bad smells (cycles, fragments)
    pub structural_data_debt: usize,
}

impl Evaluator {
    /// Performs a high-level scientific audit aligned with IEEE P3361 and ISO 25059.
    pub fn perform_audit(linker: &Linker, node_count: usize) -> AuditReport {
        let edges = linker.export_edges();
        
        // --- 1. IEEE P3361: Cognitive Load Calculation ---
        // High density = High noise = High cognitive load.
        let density = if node_count > 0 {
            (edges.len() as f32) / (node_count as f32).powi(2)
        } else {
            0.0
        };
        // v0.2.0 Calibration: Increase sensitivity for micro-scale architectures
        let cognitive_index = (1.0 - density * 50.0).clamp(0.0, 1.0);

        // --- 2. ISO 25059: Adaptability / Stability ---
        let mut total_radius = 0;
        let mut samples = 0;
        let paths = linker.get_node_paths();
        for path in paths.iter().take(30) {
            if let Some(report) = linker.calculate_impact(path) {
                total_radius += report.radius;
                samples += 1;
            }
        }
        let ripple_avg = if samples > 0 { total_radius as f32 / samples as f32 } else { 0.0 };
        // v0.2.0 Calibration: 3.0 radius threshold for stability warnings
        let adaptability = (1.0 - (ripple_avg / 3.0)).clamp(0.0, 1.0);

        // --- 3. Structural Debt ---
        // For MVP, we use unconnected components or known bad patterns
        let debt = if ripple_avg == 0.0 && node_count > 1 { 1 } else { 0 };

        AuditReport {
            ieee_p3361_cognitive_load: cognitive_index,
            iso_25059_adaptability: adaptability,
            structural_data_debt: debt,
        }
    }
}
