use clap::{Parser, Subcommand};
use std::fs;
mod engine;

use crate::engine::{Extractor, Mapper, Scanner, Benchmark, Linker, Patcher, Verifier};

#[derive(Parser)]
#[command(name = "ccap-kernel")]
#[command(about = "Industrial grade semantic compiler for 1M+ line codebases.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the project CCAP map.
    Init {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Force full deep scan
        #[arg(short, long)]
        deep: bool,
        /// Encryption key
        #[arg(short, long)]
        key: Option<String>,
    },
    /// Runs a token efficiency benchmark.
    Benchmark {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
    },
    /// Verifies the correctness and parity.
    Verify {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
    },
    /// Traces dependencies or calculates modification impact.
    Trace {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Symbol or file path to trace
        target: String,
        /// Calculate the impact (blast radius)
        #[arg(short, long)]
        impact: bool,
    },
    /// Surgically patches a specific symbol in a file.
    Patch {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Relative path to the file
        file: String,
        /// SCIP ID of the symbol to replace
        symbol_id: String,
        /// New code content for the symbol
        #[arg(short, long)]
        code: String,
    },
    /// Performs a scientific quality audit based on ISO 25010.
    Audit {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
    },
    /// Quotes the estimated token cost and risk for a modification.

    Quote {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Target symbol/file to modify
        target: String,
    },
    /// Analyzes a single file.
    Analyze {
        /// Path to the file to analyze
        path: String,
    },
    /// Inspects the contents of a specific semantic room.
    InspectRoom {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Room name to inspect
        room: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path, deep: _, key } => {
            Scanner::scan_project(path, false, key.as_deref())?;
        }
        Commands::Benchmark { path } => {
            Benchmark::run(path)?;
        }
        Commands::Verify { path } => {
            println!("🔍  Verification: Initiating Formal Parity Check for: {}", path);
            let results = Scanner::scan_for_verification(path)?;
            let mut linker = Linker::new();
            linker.build_graph(&results);
            let edges = linker.export_edges();
            let n = results.len();
            
            let report = Verifier::verify_scip_ids(&results);
            let fidelity = Verifier::calculate_fidelity(n, &edges);
            let accuracy = Verifier::verify_topological_accuracy(&linker, &results);

            println!("\n====================================================");
            println!("🛡️   CCAP FORMAL VERIFICATION REPORT");
            println!("====================================================");
            println!("✅  SCIP Format Consistency: {}", if report.scip_parity_passed { "PASSED" } else { "FAILED" });
            println!("🛑  Symbol Collisions Found: {}", report.symbol_collisions);
            println!("🧮  Algebraic Connectivity:  {:.4}", fidelity);
            println!("🎯  Topological Accuracy:    {:.2}%", accuracy * 100.0);
            println!("====================================================\n");
        }
        Commands::Trace { path, target, impact } => {
            if *impact {
                let results = Scanner::scan_for_verification(path)?;
                let mut linker = Linker::new();
                linker.build_graph(&results);
                let norm_target = target.replace("\\", "/");
                
                if let Some(report) = linker.calculate_impact(&norm_target) {
                    println!("\n====================================================");
                    println!("🚀  CCAP PRE-FLIGHT RISK BRIEFING");
                    println!("====================================================");
                    println!("🎯  Target:          {}", norm_target);
                    println!("📏  Blast Radius:    {}", report.radius);
                    println!("🔥  Risk Score:      {:.2}", report.risk_score);
                    println!("⚠️   Potential Victims ({}):", report.dependents.len());
                    for dep in report.dependents.iter().take(5) {
                        println!("    - {}", dep);
                    }
                    if report.dependents.len() > 5 {
                        println!("    ... and {} others.", report.dependents.len() - 5);
                    }
                    println!("====================================================\n");
                }
            }
        }
        Commands::Patch { path, file, symbol_id, code } => {
            let results = Scanner::scan_for_verification(path)?;
            let norm_file = file.replace("\\", "/");
            for (p, feat) in results {
                if p == norm_file {
                    if let Some(sym) = feat.exports.iter().find(|s| s.id == *symbol_id) {
                        Patcher::apply_patch(path, file, sym, code)?;
                        return Ok(());
                    }
                }
            }
            println!("❌  Symbol ID not found for patching.");
        }
        Commands::Audit { path } => {
            println!("🔬  Audit: Initiating ISO 25010 Quality Assessment for: {}", path);
            let results = Scanner::scan_for_verification(path)?;
            let mut linker = Linker::new();
            linker.build_graph(&results);
            
            let report = crate::engine::Evaluator::perform_audit(&linker, results.len());

            println!("\n====================================================");
            println!("🏛️   CCAP SCIENTIFIC AUDIT REPORT (ISO/IEC 25010)");
            println!("====================================================");
            println!("🧩  Modularity Score:      {:.4} (CCR)", report.modularity_score);
            println!("👁️   Analyzability Index:   {:.4}", report.analyzability_index);
            println!("🌊  Ripple Effect Avg:     {:.2} files", report.ripple_effect_avg);
            
            println!("\n--- [ISO CLASSIFICATION] ---");
            if report.modularity_score > 0.1 {
                println!("✅  Maintainability:       HIGH");
            } else {
                println!("⚠️   Maintainability:       MONOLITHIC / LOW");
            }
            println!("====================================================\n");
        }
        Commands::Quote { path, target } => {

            let results = Scanner::scan_for_verification(path)?;
            let mut linker = Linker::new();
            linker.build_graph(&results);
            let norm_target = target.replace("\\", "/");
            
            if let Some(report) = linker.calculate_impact(&norm_target) {
                let token_est = (report.dependents.len() * 1500) + 2000;
                println!("\n====================================================");
                println!("🧾  CCAP REFACTORING QUOTATION");
                println!("====================================================");
                println!("🎯  Target:       {}", norm_target);
                println!("💰  Est. Tokens:  {} (~${:.2})", token_est, (token_est as f32 / 1000.0) * 0.015);
                println!("🔥  Fragility:    {:.2}", report.risk_score);
                println!("🛠️   Complexity:   {} victims impacted.", report.dependents.len());
                println!("====================================================\n");
            } else {
                println!("❌  Target '{}' not found. Ensure the path is correct.", norm_target);
            }
        }
        Commands::Analyze { path } => {
            let mut extractor = Extractor::new();
            let features = extractor.analyze_file(path, path)?;
            let telegram = Mapper::to_telegram(path, &features);
            println!("{}", telegram);
        }
        Commands::InspectRoom { path, room } => {
            let storage = crate::engine::Storage::init(path)?;
            let registry_path = storage.get_map_dir().join("room_registry.json");
            let registry_json = fs::read_to_string(registry_path)?;
            let registry: std::collections::HashMap<String, Vec<String>> = serde_json::from_str(&registry_json)?;

            if let Some(members) = registry.get(room) {
                println!("🏠  Room: [{}] | Total Files: {}", room, members.len());
                for member in members {
                    let map_path = storage.get_map_dir().join(member.replace("\\", "/").replace(":", "_")).join("_MAP.md");
                    if let Ok(telegram) = fs::read_to_string(map_path) {
                        println!("  {}", telegram);
                    }
                }
            }
        }
    }

    Ok(())
}
