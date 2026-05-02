use crate::engine::{Linker, Verifier, extractor::FileFeatures};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModificationContract {
    pub target_id: String,
    pub action: String,
    pub code_snippet: String,
}

pub struct ContractGuard;

impl ContractGuard {
    /// Simulates a modification and checks if it violates structural integrity.
    pub fn verify_modification(
        contract: &ModificationContract,
        current_results: &[(String, FileFeatures)],
    ) -> anyhow::Result<f32> {
        let mut shadow_results = current_results.to_vec();
        
        let mut found = false;
        for (path, feat) in shadow_results.iter_mut() {
            // FIX: Use .as_str() to satisfy the Pattern trait
            if contract.target_id.contains(path.as_str()) {
                feat.control_flow_score += 0.1; 
                found = true;
                break;
            }
        }

        if !found {
            return Err(anyhow::anyhow!("Contract Violation: Target ID not found in map."));
        }

        let mut shadow_linker = Linker::new();
        shadow_linker.build_graph(&shadow_results);
        let edges = shadow_linker.export_edges();
        let shadow_fidelity = Verifier::calculate_fidelity(shadow_results.len(), &edges);
        
        Ok(shadow_fidelity)
    }
}
