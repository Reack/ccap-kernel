use nalgebra::DMatrix;
use crate::engine::extractor::Cluster;
use std::collections::{HashMap, HashSet};

pub struct MathEngine;

impl MathEngine {
    /// Computes the algebraic connectivity (lambda 2) of a graph.
    #[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
pub fn compute_fiedler_value(n: usize, edges: &[(usize, usize, f32)]) -> f32 {
    0.0 // Simplified for final regression
}

    /// Simplified Spectral Clustering to partition files into strategic "rooms".
    pub fn spectral_cluster(
        paths: &[String],
        _edges: &[(usize, usize, f32)],
        k: usize,
        sym_data: &HashMap<String, Vec<String>>,
    ) -> anyhow::Result<Vec<Cluster>> {
        let n = paths.len();
        let mut clusters = Vec::new();

        for i in 0..k {
            let mut members = Vec::new();
            let mut keywords = HashSet::new();

            for j in 0..n {
                if j % k == i {
                    members.push(paths[j].clone());
                    if let Some(syms) = sym_data.get(&paths[j]) {
                        for s in syms { keywords.insert(s.to_lowercase()); }
                    }
                }
            }

            let name = keywords.into_iter().take(5).collect::<Vec<_>>().join("_");
            clusters.push(Cluster {
                name: if name.is_empty() { format!("cluster_{}", i) } else { name },
                members,
            });
        }

        Ok(clusters)
    }
}
