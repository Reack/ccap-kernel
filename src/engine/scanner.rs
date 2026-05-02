use crate::engine::{Extractor, Storage, Linker, Mapper, MathEngine};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::path::Path;
use std::fs;

pub struct Scanner;

impl Scanner {
    pub fn scan_project(root: &str, _deep: bool, key: Option<&str>) -> anyhow::Result<()> {
        let start = Instant::now();
        let mut storage = Storage::init(root)?;
        if let Some(k) = key {
            storage = storage.with_security(k);
            println!("🔒  Security: Encrypted Semantic Vault Enabled.");
        }
        let storage = Arc::new(storage);
        
        println!("🚀  CSK: Initiating Industrial Semantic Compilation...");
        
        let extensions = ["py", "js", "jsx", "ts", "tsx"];
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

        println!("📂  Detected {} source files (Polyglot). Quantizing...", files.len());

        let results = Arc::new(Mutex::new(Vec::new()));
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
                res.push((rel_path_str, features));
            }
        });

        let duration = start.elapsed();
        let analysis_results = results.lock().unwrap().clone();
        
        println!("✅  Feature Extraction: 100% Deterministic (0 Token Cost).");

        if !analysis_results.is_empty() {
            let mut linker = Linker::new();
            linker.build_graph(&analysis_results);

            let edges = linker.export_edges();
            let paths = linker.get_node_paths();
            
            if let Ok(clusters) = MathEngine::spectral_cluster(&paths, &edges, 3) {
                let mut skeleton = format!("@ROOT[{}]\n", root);
                for (path, cid) in clusters {
                    skeleton.push_str(&format!("@ROOM[cluster_{}] NODE:{}\n", cid, path));
                }
                let skeleton_path = storage.get_map_dir().join("root.st.aaak");
                fs::write(skeleton_path, skeleton)?;
                println!("🦴  Topology: High-entropy Skeleton Index generated.");
            }
        }

        println!("🛡️   Atlas Synchronization Complete. [Duration: {:?}]", duration);
        Ok(())
    }
}
