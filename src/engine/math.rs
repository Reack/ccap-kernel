use nalgebra::{DMatrix, SymmetricEigen};
use std::collections::HashMap;

pub struct MathEngine;

#[derive(Debug)]
pub struct ClusterSummary {
    pub name: String,
    pub members: Vec<String>,
}

impl MathEngine {
    pub fn compute_fiedler_value(n: usize, edges: &[(usize, usize, f32)]) -> f32 {
        if n < 2 { return 0.0; }
        
        let mut a = DMatrix::zeros(n, n);
        for &(u, v, w) in edges {
            if u < n && v < n {
                a[(u, v)] = w;
                a[(v, u)] = w;
            }
        }

        let mut d = DMatrix::zeros(n, n);
        for i in 0..n {
            d[(i, i)] = a.row(i).sum();
        }

        let l = d - a;
        let eig = SymmetricEigen::new(l);
        let mut values: Vec<f32> = eig.eigenvalues.iter().cloned().collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        if values.len() > 1 { values[1] } else { values[0] }
    }

    pub fn spectral_cluster(
        node_paths: &[String],
        _edges: &[(usize, usize, f32)],
        k: usize,
        node_symbols: &HashMap<String, Vec<String>>,
    ) -> anyhow::Result<Vec<ClusterSummary>> {
        let n = node_paths.len();
        if n == 0 { return Ok(Vec::new()); }

        let mut cluster_map: HashMap<usize, Vec<String>> = HashMap::new();
        for (i, path) in node_paths.iter().enumerate() {
            let cid = i % k; 
            cluster_map.entry(cid).or_insert_with(Vec::new).push(path.clone());
        }

        let mut summaries = Vec::new();
        for (cid, members) in cluster_map {
            let mut symbol_counts: HashMap<String, usize> = HashMap::new();
            for member in &members {
                // SAFETY: Using if let to avoid unwrap() panic
                if let Some(symbols) = node_symbols.get(member) {
                    for sym in symbols {
                        *symbol_counts.entry(sym.clone()).or_insert(0) += 1;
                    }
                }
            }

            let mut top_symbols: Vec<_> = symbol_counts.into_iter().collect();
            top_symbols.sort_by(|a, b| b.1.cmp(&a.1));
            
            let name = top_symbols.into_iter()
                .take(3)
                .map(|(s, _)| s)
                .collect::<Vec<_>>()
                .join("_");

            summaries.push(ClusterSummary {
                name: if name.is_empty() { format!("ZONE_{}", cid) } else { name },
                members,
            });
        }

        Ok(summaries)
    }
}
