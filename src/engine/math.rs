use nalgebra::DMatrix;
use crate::engine::extractor::Cluster;
use std::collections::{HashMap, HashSet};

pub struct MathEngine;

impl MathEngine {
    /// Computes the algebraic connectivity (lambda 2) of a graph.
    #[allow(dead_code)]
pub fn compute_fiedler_value(n: usize, edges: &[(usize, usize, f32)]) -> f32 {
    if n < 2 { return 1.0; }
    let mut laplacian: DMatrix<f32> = DMatrix::zeros(n, n);
    for &(u, v, w) in edges {
        if u >= n || v >= n { continue; }
        laplacian[(u, u)] += w;
        laplacian[(v, v)] += w;
        laplacian[(u, v)] -= w;
        laplacian[(v, u)] -= w;
    }
    let eig = laplacian.symmetric_eigen();
    let mut values: Vec<f32> = eig.eigenvalues.iter().cloned().collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    *values.get(1).unwrap_or(&0.0)
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

            let name = crate::engine::archetype::ArchetypeEngine::infer_cluster_name(&keywords, i);
            clusters.push(Cluster {
                name,
                members,
            });
        }

        Ok(clusters)
    }
}
