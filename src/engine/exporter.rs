use serde::{Serialize, Deserialize};
use crate::engine::extractor::FileFeatures;
use crate::engine::Linker;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct AtlasExport {
    pub project_name: String,
    pub directed: bool,
    pub multigraph: bool,
    pub nodes: Vec<NodeExport>,
    pub edges: Vec<EdgeExport>, // NetworkX standard
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeExport {
    pub id: String, 
    pub features: FileFeatures,
    pub inferred_role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeExport {
    pub source: String,
    pub target: String,
    pub weight: f32,
}

pub struct Exporter;

impl Exporter {
    /// Aggregates all kernel data into a standard JSON export (NetworkX Gold Standard).
    pub fn export_atlas(
        repo_root: &str,
        analysis_results: &[(String, FileFeatures)],
        linker: &Linker,
    ) -> anyhow::Result<AtlasExport> {
        let mut nodes = Vec::new();
        let edges_raw = linker.export_edges();
        let paths = linker.get_node_paths();

        for (path, features) in analysis_results {
            nodes.push(NodeExport {
                id: path.clone(),
                features: features.clone(),
                inferred_role: "PENDING_INFERENCE".to_string(),
            });
        }

        let mut edges = Vec::new();
        for (u, v, w) in edges_raw {
            if u < paths.len() && v < paths.len() {
                edges.push(EdgeExport {
                    source: paths[u].clone(),
                    target: paths[v].clone(),
                    weight: w,
                });
            }
        }

        let export = AtlasExport {
            project_name: Path::new(repo_root)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            directed: true,
            multigraph: false,
            nodes,
            edges,
        };

        Ok(export)
    }

    pub fn save_to_file(export: &AtlasExport, output_path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(export)?;
        fs::write(output_path, json)?;
        println!("🚀  Export: CCAP Atlas (NetworkX-Ready) saved to {}", output_path);
        Ok(())
    }
}
