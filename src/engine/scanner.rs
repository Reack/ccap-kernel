use crate::engine::{Extractor, Storage, Linker, Mapper, MathEngine};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

pub struct Scanner;

impl Scanner {
    pub fn scan_project(root: &str, _deep: bool, key: Option<&str>) -> anyhow::Result<()> {
        let start = Instant::now();
        let mut storage = Storage::init(root)?;
        if let Some(k) = key {
            storage = storage.with_security(k);
        }
        let storage = Arc::new(storage);
        
        println!("🚀  CSK: Starting Hierarchical Semantic Compilation...");
        
        let extensions = ["py", "js", "jsx", "ts", "tsx", "c", "h", "cpp", "hpp", "cc", "hh", "rs", "go", "java", "cs"];
        let files: Vec<_> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                !path.to_string_lossy().contains(".ccap") &&
                path.is_file() && 
                extensions.iter().any(|ext| path.extension().map_or(false, |e| e == *ext))
            })
            .collect();

        let results = Arc::new(Mutex::new(Vec::new()));
        let node_symbols = Arc::new(Mutex::new(HashMap::new()));
        let root_path = Path::new(root).to_path_buf();

        files.par_iter().for_each(|entry| {
            let mut extractor = Extractor::new();
            let file_path = entry.path();
            let rel_path = pathdiff::diff_paths(file_path, &root_path)
                .unwrap_or_else(|| file_path.to_path_buf());
            let rel_path_str = rel_path.to_string_lossy().to_string();

            if let Ok(features) = extractor.analyze_file(file_path.to_str().unwrap(), &rel_path_str) {
                let telegram = Mapper::to_telegram(&rel_path_str, &features);
                let _ = storage.save_map(&rel_path_str, &telegram, &features);
                
                let mut res = results.lock().unwrap();
                res.push((rel_path_str.clone(), features.clone()));
                
                let mut sym_map = node_symbols.lock().unwrap();
                sym_map.insert(rel_path_str, features.top_symbols);
            }
        });

        let analysis_results = results.lock().unwrap().clone();
        let sym_data = node_symbols.lock().unwrap().clone();

        if !analysis_results.is_empty() {
            let mut linker = Linker::new();
            linker.build_graph(&analysis_results);

            let edges = linker.export_edges();
            let paths = linker.get_node_paths();
            
            // Generate Semantic Clusters (3 main rooms for strategic view)
            if let Ok(clusters) = MathEngine::spectral_cluster(&paths, &edges, 3, &sym_data) {
                let mut skeleton = format!("@ROOT[{}]\n", root);
                let mut room_registry = HashMap::new();

                for cluster in clusters {
                    skeleton.push_str(&format!(
                        "@ROOM[{}] Size:{} | Keywords:[{}]\n", 
                        cluster.name, cluster.members.len(), cluster.name.replace("_", "|")
                    ));
                    room_registry.insert(cluster.name, cluster.members);
                }
                
                // Save high-level skeleton
                let skeleton_path = storage.get_map_dir().join("root.st.aaak");
                fs::write(skeleton_path, skeleton)?;

                // Save detailed room registry for 'inspect-room'
                let registry_path = storage.get_map_dir().join("room_registry.json");
                fs::write(registry_path, serde_json::to_string_pretty(&room_registry)?)?;
                
                println!("🦴  Hierarchy: Strategic Skeleton Index generated.");
            }
        }

        println!("🛡️   Synchronization Complete. [Duration: {:?}]", start.elapsed());
        Ok(())
    }

    /// Scans the project specifically for verification purposes, returning raw results.
    pub fn scan_for_verification(root: &str) -> anyhow::Result<Vec<(String, crate::engine::extractor::FileFeatures)>> {
        let extensions = ["py", "js", "jsx", "ts", "tsx", "c", "h", "cpp", "hpp", "cc", "hh", "rs", "go", "java", "cs"];
        let files: Vec<_> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                !path.to_string_lossy().contains(".ccap") &&
                path.is_file() && 
                extensions.iter().any(|ext| path.extension().map_or(false, |e| e == *ext))
            })
            .collect();

        let results = Arc::new(Mutex::new(Vec::new()));
        let root_path = Path::new(root).to_path_buf();

        files.par_iter().for_each(|entry| {
            let mut extractor = Extractor::new();
            let file_path = entry.path();
            let rel_path = pathdiff::diff_paths(file_path, &root_path)
                .unwrap_or_else(|| file_path.to_path_buf());
            let rel_path_str = rel_path.to_string_lossy().to_string();

            if let Ok(features) = extractor.analyze_file(file_path.to_str().unwrap(), &rel_path_str) {
                let mut res = results.lock().unwrap();
                res.push((rel_path_str, features));
            }
        });

        let res = results.lock().unwrap().clone();
        Ok(res)
    }
}


