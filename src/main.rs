use clap::{Parser, Subcommand};
use std::fs;
mod engine;


use crate::engine::{Extractor, Mapper, Scanner, Benchmark};

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
    /// Verifies the correctness and parity of the generated maps.
    Verify {
        /// Repository root path
        #[arg(default_value = ".")]
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
    /// Analyzes a single file and outputs the ST-AAAK telegram.

    Analyze {
        /// Path to the file to analyze
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path, deep, key } => {
            Scanner::scan_project(path, *deep, key.as_deref())?;
        }
        Commands::Benchmark { path } => {
            Benchmark::run(path)?;
        }
        Commands::Verify { path } => {
            println!("🔍  Verification: Initiating Formal Parity Check for: {}", path);
            let results = Scanner::scan_for_verification(path)?;
            let mut report = crate::engine::Verifier::verify_scip_ids(&results);

            // Calculate Topological Fidelity
            let mut linker = crate::engine::Linker::new();
            linker.build_graph(&results);
            let edges = linker.export_edges();
            let n = results.len();
            report.algebraic_connectivity = crate::engine::Verifier::calculate_fidelity(n, &edges);

            println!("\n====================================================");
            println!("🛡️   CCAP FORMAL VERIFICATION REPORT");
            println!("====================================================");
            println!("✅  SCIP Format Consistency: {}", if report.scip_parity_passed { "PASSED" } else { "FAILED" });
            println!("🛑  Symbol Collisions Found: {}", report.symbol_collisions);
            println!("🧮  Algebraic Connectivity:  {:.4}", report.algebraic_connectivity);
            
            if report.algebraic_connectivity > 0.0 {
                println!("✨  Structural Integrity:   SECURE (Graph is connected)");
            } else {
                println!("⚠️   Structural Integrity:   FRAGMENTED (Graph has isolated islands)");
            }
            
            if !report.format_errors.is_empty() {
                println!("\n❌  DETAILED ERRORS:");
                for err in report.format_errors.iter().take(10) {
                    println!("    - {}", err);
                }
            } else {
                println!("\n✨  Perfect Parity: All symbols are 100% SCIP compliant.");
            }
            println!("====================================================\n");
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
            } else {
                println!("❌  Room '{}' not found. Available rooms: {:?}", room, registry.keys());
            }
        }
        Commands::Analyze { path } => {

            let mut extractor = Extractor::new();
            let features = extractor.analyze_file(path, path)?;
            let telegram = Mapper::to_telegram(path, &features);
            println!("{}", telegram);
        }
    }

    Ok(())
}
