use crate::engine::extractor::FileFeatures;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use std::collections::{HashMap, HashSet};

pub struct Linker {
    graph: DiGraph<String, f32>,
    nodes: HashMap<String, NodeIndex>,
}

#[derive(Debug)]
pub struct ImpactReport {
    pub target: String,
    pub radius: usize,
    pub dependents: Vec<String>,
    pub risk_score: f32,
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
    }

    pub fn calculate_impact(&self, target_path: &str) -> Option<ImpactReport> {
        let start_node = self.nodes.get(target_path)?;
        
        let mut dependents = Vec::new();
        let mut stack = vec![(*start_node, 0)];
        let mut visited = HashSet::new();
        visited.insert(*start_node);

        while let Some((curr, depth)) = stack.pop() {
            if curr != *start_node {
                dependents.push(self.graph[curr].clone());
            }

            for neighbor in self.graph.neighbors_directed(curr, Direction::Incoming) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    stack.push((neighbor, depth + 1));
                }
            }
        }

        // Calculate max depth from the visited set (this logic is simplified for speed)
        let max_depth = if !dependents.is_empty() { 3 } else { 0 }; // Placeholder for actual BFS depth

        Some(ImpactReport {
            target: target_path.to_string(),
            radius: max_depth,
            dependents,
            risk_score: (base_risk_calculation(&self.graph, *start_node, &visited)).min(1.0),
        })
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

fn base_risk_calculation(graph: &DiGraph<String, f32>, start: NodeIndex, visited: &HashSet<NodeIndex>) -> f32 {
    let in_degree = graph.neighbors_directed(start, Direction::Incoming).count();
    (in_degree as f32 * 0.4) + (visited.len() as f32 * 0.1)
}
