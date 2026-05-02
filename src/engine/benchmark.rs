use crate::engine::{Storage, Scanner, Verifier, Linker};
use tiktoken_rs::cl100k_base;
use std::fs;
use walkdir::WalkDir;

pub struct Benchmark;

impl Benchmark {
    pub fn run(root: &str) -> anyhow::Result<()> {
        println!("📊  CCAP V4.0: Initiating Integrated Integrity & Efficiency Benchmark...");
        println!("📂  Target Project: {}", root);
        
        let analysis_results = Scanner::scan_for_verification(root)?;
        let v_report = Verifier::verify_scip_ids(&results_to_slice(&analysis_results));
        
        let mut linker = Linker::new();
        linker.build_graph(&analysis_results);
        let edges = linker.export_edges();
        let fidelity = Verifier::calculate_fidelity(analysis_results.len(), &edges);
        let accuracy = Verifier::verify_topological_accuracy(&linker, &analysis_results);

        let bpe = cl100k_base()?;
        let mut total_raw_tokens = 0;
        let mut total_map_tokens = 0;

        for (path_str, _) in &analysis_results {
            let abs_path = std::path::Path::new(root).join(path_str);
            if let Ok(content) = fs::read_to_string(abs_path) {
                total_raw_tokens += bpe.encode_with_special_tokens(&content).len();
            }
        }

        let storage = Storage::init(root)?;
        if storage.get_map_dir().exists() {
            for entry in WalkDir::new(storage.get_map_dir())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().file_name().map_or(false, |n| n == "_MAP.md"))
            {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    total_map_tokens += bpe.encode_with_special_tokens(&content).len();
                }
            }
        }

        println!("\n====================================================");
        println!("🏆  CCAP V4.0 INTEGRATED BENCHMARK REPORT");
        println!("====================================================");
        println!("--- [1. EFFICIENCY MATRIX] ---");
        println!("📄  Raw Source Tokens:     {}", total_raw_tokens);
        println!("🛰️  CCAP Map Tokens (ST):  {}", total_map_tokens);
        if total_map_tokens > 0 {
            println!("📉  Compression Ratio:     {:.2}x", total_raw_tokens as f64 / total_map_tokens as f64);
            println!("💰  Token Savings:         {:.2}%", (1.0 - (total_map_tokens as f64 / total_raw_tokens as f64)) * 100.0);
        }

        println!("\n--- [2. FIDELITY MATRIX] ---");
        println!("🎯  Topological Accuracy:    {:.2}%", accuracy * 100.0);
        println!("🧮  Algebraic Connectivity:  {:.4}", fidelity);
        println!("✅  SCIP Status:            {}", if v_report.scip_parity_passed { "PASSED" } else { "FAILED" });
        println!("====================================================\n");

        Ok(())
    }
}

fn results_to_slice(v: &[(String, crate::engine::extractor::FileFeatures)]) -> &[(String, crate::engine::extractor::FileFeatures)] {
    v
}
