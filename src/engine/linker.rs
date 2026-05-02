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
        println!("\n🏗️  Topological Analysis (BOND Discovery):");

        for (path, _) in analysis_results {
            let idx = self.graph.add_node(path.clone());
            self.nodes.insert(path.clone(), idx);
        }

        for (path, feat) in analysis_results {
            let source_idx = *self.nodes.get(path).unwrap();
            
            for imp in &feat.imports {
                let imp_normalized = imp.replace(".", "/").to_lowercase();
                
                for (other_path, _) in analysis_results {
                    let other_normalized = other_path.replace("\\", "/").to_lowercase();
                    let other_stem = other_normalized.strip_suffix(".py").unwrap_or(&other_normalized);

                    if other_stem == &imp_normalized || other_stem.ends_with(&format!("/{}", imp_normalized)) {
                        let target_idx = *self.nodes.get(other_path).unwrap();
                        if !self.graph.contains_edge(source_idx, target_idx) {
                            self.graph.add_edge(source_idx, target_idx, 0.9);
                            println!("    🔗 BOND: {} -> {} (via '{}')", path, other_path, imp);
                        }
                    }
                }
            }
        }

        println!("✅ Topology Mapping Finished. {} active links found.", self.graph.edge_count());
    }

    /// Exports the graph structure for mathematical processing.
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
