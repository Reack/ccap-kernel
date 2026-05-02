use std::collections::HashSet;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use crate::engine::extractor::FileFeatures;
use petgraph::prelude::DiGraph;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImpactReport {
    pub radius: usize,
    pub dependents: Vec<String>,
    pub risk_score: f32,
}

pub struct Linker {
    graph: DiGraph<String, f32>,
    path_to_idx: std::collections::HashMap<String, NodeIndex>,
}

impl Linker {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            path_to_idx: std::collections::HashMap::new(),
        }
    }

    pub fn build_graph(&mut self, results: &[(String, FileFeatures)]) {
        // 1. Add all nodes
        for (path, _) in results {
            let idx = self.graph.add_node(path.clone());
            self.path_to_idx.insert(path.clone(), idx);
        }

        // 2. Add edges based on SCIP references with SMART MAPPING
        for (path, features) in results {
            if let Some(&u) = self.path_to_idx.get(path) {
                for ref_id in &features.references {
                    let target_base = ref_id.split('#').nth(0).unwrap_or("").replace("ccap . . ", "");
                    
                    // TRY 1: Exact match
                    if let Some(&v) = self.path_to_idx.get(&target_base) {
                        self.graph.add_edge(u, v, 1.0);
                        continue;
                    }

                    // TRY 2: Fuzzy match with extensions (.py, .rs, etc)
                    for (node_path, &v) in &self.path_to_idx {
                        if node_path.starts_with(&target_base) && (node_path.ends_with(".py") || node_path.ends_with(".rs") || node_path.ends_with(".c")) {
                            self.graph.add_edge(u, v, 1.0);
                        }
                    }
                }
            }
        }
    }

    pub fn calculate_impact(&self, target_path: &str) -> Option<ImpactReport> {
        let root_idx = *self.path_to_idx.get(target_path)?;
        let mut victims = HashSet::new();
        let mut current_layer = vec![root_idx];
        let mut radius = 0;

        while !current_layer.is_empty() && radius < 5 {
            let mut next_layer = Vec::new();
            for &idx in &current_layer {
                let mut neighbors = self.graph.neighbors_directed(idx, Direction::Incoming);
                while let Some(neighbor) = neighbors.next() {
                    if victims.insert(neighbor) {
                        next_layer.push(neighbor);
                    }
                }
            }
            if !next_layer.is_empty() { radius += 1; }
            current_layer = next_layer;
        }

        let dependents: Vec<String> = victims.iter().map(|&idx| self.graph[idx].clone()).collect();
        Some(ImpactReport { radius, dependents, risk_score: (radius as f32 / 5.0).min(1.0) })
    }

    pub fn export_edges(&self) -> Vec<(usize, usize, f32)> {
        self.graph.edge_indices()
            .map(|e| {
                let (u, v) = self.graph.edge_endpoints(e).unwrap();
                (u.index(), v.index(), *self.graph.edge_weight(e).unwrap())
            })
            .collect()
    }

    pub fn get_node_paths(&self) -> Vec<String> {
        (0..self.graph.node_count()).map(|i| self.graph[NodeIndex::new(i)].clone()).collect()
    }
}
