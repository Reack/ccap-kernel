use crate::engine::{Extractor, Storage, Linker, Mapper};
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::path::Path;

pub struct Scanner;

impl Scanner {
    pub fn scan_project(root: &str, deep: bool) -> anyhow::Result<()> {
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

        if !analysis_results.is_empty() {
            let mut linker = Linker::new();
            linker.build_graph(&analysis_results);
        }

        println!("🛡️  All maps synchronized in .ccap/ (Duration: {:?})", duration);

        Ok(())
    }
}
