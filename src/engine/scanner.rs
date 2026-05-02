use crate::engine::{Extractor, Storage, Linker, Mapper, MathEngine};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::path::Path;
use std::fs;

pub struct Scanner;

impl Scanner {
    pub fn scan_project(root: &str, _deep: bool) -> anyhow::Result<()> {
        let start = Instant::now();
        let storage = Arc::new(Storage::init(root)?);
        
        println!("🚀 Initiating Semantic Sync at: {}", root);
        
        let files: Vec<_> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                !path.to_string_lossy().contains(".ccap") &&
                path.is_file() && 
                path.extension().map_or(false, |ext| ext == "py")
            })
            .collect();

        println!("📂 Detected {} Python files.", files.len());

        let results = Arc::new(Mutex::new(Vec::new()));
        let root_path = Path::new(root).to_path_buf();

        files.par_iter().for_each(|entry| {
            let mut extractor = Extractor::new();
            let file_path = entry.path();
            
            match extractor.analyze_python(file_path.to_str().unwrap()) {
                Ok(features) => {
                    let rel_path = pathdiff::diff_paths(file_path, &root_path)
                        .unwrap_or_else(|| file_path.to_path_buf());
                    let rel_path_str = rel_path.to_string_lossy().to_string();

                    let telegram = Mapper::to_telegram(&rel_path_str, &features);
                    let _ = storage.save_map(&rel_path_str, &telegram, &features);

                    let mut res = results.lock().unwrap();
                    res.push((rel_path_str, features));
                },
                Err(e) => {
                    eprintln!("❌ Error analyzing {:?}: {}", file_path, e);
                }
            }
        });

        let duration = start.elapsed();
        let analysis_results = results.lock().unwrap().clone();
        
        println!("\n✅ Feature Extraction complete. Analyzed {} files.", analysis_results.len());

        // 4. Phase 2: Math & Topology
        if !analysis_results.is_empty() {
            let mut linker = Linker::new();
            linker.build_graph(&analysis_results);

            let edges = linker.export_edges();
            let paths = linker.get_node_paths();
            
            // Execute Spectral Clustering (Example: group into 3 semantic rooms)
            if let Ok(clusters) = MathEngine::spectral_cluster(&paths, &edges, 3) {
                println!("📦  Semantic Clustering: Identified {} functional areas.", clusters.values().collect::<std::collections::HashSet<_>>().len());
                
                // 5. Generate AAAK Skeleton Index
                let mut skeleton = format!("@ROOT[{}]\n", root);
                for (path, cid) in clusters {
                    skeleton.push_str(&format!("@ROOM[cluster_{}] NODE:{}\n", cid, path));
                }
                
                let skeleton_path = storage.get_map_dir().join("root.st.aaak");
                fs::write(skeleton_path, skeleton)?;
                println!("🦴  AAAK Skeleton Index generated at root.st.aaak");
            }
        }

        println!("🛡️  All maps synchronized in .ccap/ (Total Duration: {:?})", duration);

        Ok(())
    }
}
