use serde::{Serialize, Deserialize};
use crate::engine::extractor::FileFeatures;
use crate::engine::Linker;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct AtlasExport {
    pub project_name: String,
    pub nodes: Vec<NodeExport>,
    pub edges: Vec<EdgeExport>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeExport {
    pub id: String, // Relative path or SCIP root
    pub features: FileFeatures,
    pub inferred_role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeExport {
    pub from: String,
    pub to: String,
    pub weight: f32,
}

pub struct Exporter;

impl Exporter {
    /// Aggregates all kernel data into a standard JSON export.
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
                inferred_role: "PENDING_INFERENCE".to_string(), // Placeholder for future logic
            });
        }

        let mut edges = Vec::new();
        for (u, v, w) in edges_raw {
            edges.push(EdgeExport {
                from: paths[u].clone(),
                to: paths[v].clone(),
                weight: w,
            });
        }

        let export = AtlasExport {
            project_name: Path::new(repo_root)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            nodes,
            edges,
        };

        Ok(export)
    }

    pub fn save_to_file(export: &AtlasExport, output_path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(export)?;
        fs::write(output_path, json)?;
        println!("🚀  Export: CCAP Atlas saved to {}", output_path);
        Ok(())
    }
}
