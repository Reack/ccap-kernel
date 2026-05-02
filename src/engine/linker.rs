use crate::engine::extractor::FileFeatures;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct Linker {
    graph: DiGraph<String, f32>,
    nodes: HashMap<String, NodeIndex>,
}

impl Linker {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn build_graph(&mut self, analysis_results: &[(String, FileFeatures)]) {
        let mut export_index = HashMap::new();
        for (path, feat) in analysis_results {
            let idx = self.graph.add_node(path.clone());
            self.nodes.insert(path.clone(), idx);

            for export in &feat.exports {
                export_index.insert(export.id.clone(), idx);
            }
        }

        for (path, feat) in analysis_results {
            let source_idx = *self.nodes.get(path).unwrap();
            for ref_id in &feat.references {
                for (exp_id, &target_idx) in &export_index {
                    if source_idx == target_idx { continue; }
                    if exp_id == ref_id || exp_id.starts_with(ref_id) {
                        if !self.graph.contains_edge(source_idx, target_idx) {
                            self.graph.add_edge(source_idx, target_idx, 1.0);
                        }
                    }
                }
            }
        }
        println!("🔗  Topological Bonding: Detected {} active relationships.", self.graph.edge_count());
    }

    pub fn export_edges(&self) -> Vec<(usize, usize, f32)> {
        self.graph
            .edge_indices()
            .map(|e| {
                let (u, v) = self.graph.edge_endpoints(e).unwrap();
                let weight = *self.graph.edge_weight(e).unwrap();
                (u.index(), v.index(), weight)
            })
            .collect()
    }

    pub fn get_node_paths(&self) -> Vec<String> {
        let mut paths = vec![String::new(); self.graph.node_count()];
        for idx in self.graph.node_indices() {
            paths[idx.index()] = self.graph[idx].clone();
        }
        paths
    }
}
