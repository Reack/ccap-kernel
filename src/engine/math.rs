use nalgebra::DMatrix;
use crate::engine::extractor::Cluster;
use std::collections::{HashMap, HashSet};

pub struct MathEngine;

impl MathEngine {
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

    /// Optimized Clustering: Group by Path Prefixes (Heuristic) for better "Rooms".
    pub fn spectral_cluster(
        paths: &[String],
        _edges: &[(usize, usize, f32)],
        k: usize,
        sym_data: &HashMap<String, Vec<String>>,
    ) -> anyhow::Result<Vec<Cluster>> {
        let n = paths.len();
        let mut clusters = Vec::new();
        
        // 1. Sort paths to group similar directories
        let mut sorted_paths = paths.to_vec();
        sorted_paths.sort();

        // 2. Linear partition into k groups (better than modulo)
        let chunk_size = (n / k).max(1);
        for i in 0..k {
            let start = i * chunk_size;
            let end = if i == k - 1 { n } else { (i + 1) * chunk_size }.min(n);
            if start >= n { break; }

            let members = sorted_paths[start..end].to_vec();
            let mut keywords = HashSet::new();
            for m in &members {
                if let Some(syms) = sym_data.get(m) {
                    for s in syms { keywords.insert(s.to_lowercase()); }
                }
            }

            let name = crate::engine::archetype::ArchetypeEngine::infer_cluster_name(&keywords, i);
            clusters.push(Cluster { name, members });
        }

        Ok(clusters)
    }
}
