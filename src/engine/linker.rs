use std::collections::HashSet;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use crate::engine::extractor::FileFeatures;
use petgraph::prelude::DiGraph;
use serde::{Serialize, Deserialize};
use log::debug;

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

    /// 核心正規化邏輯：確保全平台、全語法下的路徑一致性 (v0.2.0 Gold Baseline)
    pub fn normalize_path(path: &str) -> String {
        path.replace("\\", "/")
            .replace("crate::", "")
            .replace("::", "/")
            .trim_start_matches("./")
            .trim_start_matches("../")
            .split('.')
            .next() // 移除副檔名
            .unwrap_or(path)
            .to_lowercase()
    }

    pub fn build_graph(&mut self, results: &[(String, FileFeatures)]) {
        self.graph.clear();
        self.path_to_idx.clear();

        // 1. Add all nodes using ORIGINAL paths
        for (path, _) in results {
            let idx = self.graph.add_node(path.clone());
            self.path_to_idx.insert(path.clone(), idx);
        }

        // 2. Add edges with Normalization-on-Demand
        for (path, features) in results {
            if let Some(&u) = self.path_to_idx.get(path) {
                for ref_id in &features.references {
                    let target_raw = ref_id.replace("ccap . . ", "");
                    let v_norm = Self::normalize_path(&target_raw);
                    
                    // 尋找匹配的原始路徑
                    let found = self.path_to_idx.iter().find(|(p_orig, _)| {
                        Self::normalize_path(p_orig) == v_norm
                    });

                    if let Some((_, &v)) = found {
                        if u != v {
                            // Ghost Link 偵測：是否有符號級調用 (#)
                            let is_solid = ref_id.contains('#');
                            let weight = if is_solid { 1.0 } else { 0.1 };
                            
                            if !self.graph.contains_edge(u, v) {
                                self.graph.add_edge(u, v, weight);
                            }
                        }
                    }
                }
            }
        }

        // 3. 拓樸孤島偵測 (Debug Only)
        self.detect_islands();
    }

    fn detect_islands(&self) {
        let node_count = self.graph.node_count();
        if node_count == 0 { return; }

        let mut connected = HashSet::new();
        for edge in self.graph.edge_indices() {
            let (u, v) = self.graph.edge_endpoints(edge).unwrap();
            connected.insert(u);
            connected.insert(v);
        }
        
        if connected.len() < node_count {
            debug!("Topological Audit: Found {} isolated files (islands).", node_count - connected.len());
            for i in 0..node_count {
                let idx = NodeIndex::new(i);
                if !connected.contains(&idx) {
                    debug!("  Island detected: {}", self.graph[idx]);
                }
            }
        }
    }

    pub fn calculate_impact(&self, target_path: &str) -> Option<ImpactReport> {
        // 使用正規化查找
        let target_norm = Self::normalize_path(target_path);
        let root_idx = self.path_to_idx.iter()
            .find(|(p, _)| Self::normalize_path(p) == target_norm)
            .map(|(_, &idx)| idx)?;

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
