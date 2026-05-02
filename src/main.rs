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
    /// Displays local token savings statistics.
    Stats {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
    },
    /// Exports the full project atlas to a standard JSON format for 3rd-party KGs.
    Export {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Output file path (e.g., atlas.json)
        #[arg(short, long, default_value = "ccap-atlas.json")]
        output: String,
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
    /// Runs a command and filters its output through the proxy engine.
    Run {
        /// Command to run
        command: String,
        /// Arguments for the command
        args: Vec<String>,
        /// Repository root path for normalization
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Validates a modification contract (shadow execution).
    Contract {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Target SCIP ID
        target: String,
        /// New code snippet for validation
        #[arg(short, long)]
        code: String,
    },
    /// Manages the semantic glossary (aliases).
    Glossary {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// SCIP ID to alias
        #[arg(short, long)]
        id: String,
        /// Alias name
        #[arg(short, long)]
        alias: String,
    },
    /// Generates a semantic Wiki page for a given target.
    Wiki {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Target file path to view
        target: String,
    },
    /// Runs the physical proofs verification suite (Chapter 14).
    Prove {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
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
                        crate::engine::Patcher::apply_patch(path, file, sym, code)?;
                        return Ok(());
                    }
                }
            }
            println!("❌  Symbol ID not found for patching.");
        }
        Commands::Audit { path } => {
            println!("🔬  Audit: Initiating Standards-Compliant Quality Assessment for: {}", path);
            let results = Scanner::scan_for_verification(path)?;
            let mut linker = Linker::new();
            linker.build_graph(&results);
            
            let report = crate::engine::Evaluator::perform_audit(&linker, results.len());

            println!("\n====================================================");
            println!("🏛️   CCAP SCIENTIFIC AUDIT REPORT");
            println!("====================================================");
            println!("📘  IEEE P3361 Cognitive Index:  {:.4}", report.ieee_p3361_cognitive_load);
            println!("📐  ISO 25059 Adaptability:     {:.4}", report.iso_25059_adaptability);
            println!("📊  Structural Data Debt:       {}", report.structural_data_debt);
            
            println!("\n--- [V2P TRANSITION ANALYSIS] ---");
            if report.ieee_p3361_cognitive_load > 0.8 && report.iso_25059_adaptability > 0.7 {
                println!("✅  Product Ready: HIGH (Strong structure for AI scale-up)");
            } else {
                println!("⚠️   Vibe Only:     MEDIUM (Requires structural refactoring)");
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
            }
        }
        Commands::Stats { path } => {
            let storage = crate::engine::Storage::init(path)?;
            let log_path = storage.get_map_dir().parent().unwrap().join("telemetry.log");
            
            if log_path.exists() {
                let content = fs::read_to_string(log_path)?;
                let mut total_raw = 0;
                let mut total_map = 0;
                let mut file_count = 0;

                for line in content.lines() {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() == 3 {
                        let raw: usize = parts[1].split(':').nth(1).unwrap_or("0").parse().unwrap_or(0);
                        let map: usize = parts[2].split(':').nth(1).unwrap_or("0").parse().unwrap_or(0);
                        total_raw += raw;
                        total_map += map;
                        file_count += 1;
                    }
                }

                println!("\n====================================================");
                println!("📈  CCAP TOKEN SAVINGS STATS (Local Audit)");
                println!("====================================================");
                println!("📂  Files Tracked:    {}", file_count);
                println!("📄  Raw Size Est:     {} (Bytes)", total_raw);
                println!("🛰️   CCAP Map Size:    {} (Bytes)", total_map);
                
                if total_raw > 0 {
                    let savings = (1.0 - (total_map as f64 / total_raw as f64)) * 100.0;
                    println!("💰  Total Savings:     {:.2}%", savings);
                }
                println!("====================================================\n");
            }
        }
        Commands::Export { path, output } => {
            println!("📂  Export: Compiling Project Atlas for 3rd-party compatibility...");
            let results = Scanner::scan_for_verification(path)?;
            let mut linker = Linker::new();
            linker.build_graph(&results);
            
            let atlas = crate::engine::Exporter::export_atlas(path, &results, &linker)?;
            crate::engine::Exporter::save_to_file(&atlas, output)?;
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
                    let safe_member = member.replace("\\", "/").replace(":", "_");
                    let map_path = storage.get_map_dir().join(safe_member).join("_MAP.md");
                    if let Ok(telegram) = fs::read_to_string(map_path) {
                        println!("  {}", telegram);
                    }
                }
            }
        }
        Commands::Run { command, args, path } => {
            let actual_path = if path == "." {
                std::env::current_dir()?.to_string_lossy().to_string()
            } else {
                path.clone()
            };

            let output = std::process::Command::new(command)
                .args(args)
                .output()
                .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let cleaned = crate::engine::ProxyEngine::clean_output(&stdout, &actual_path);
            println!("{}", cleaned);
        }
        Commands::Contract { path, target, code } => {
            println!("📑  Contract: Initiating Shadow Execution for mutation on: {}", target);
            let results = Scanner::scan_for_verification(path)?;

            let contract = crate::engine::contract::ModificationContract {
                target_id: target.clone(),
                action: "PATCH".to_string(),
                code_snippet: code.clone(),
            };

            match crate::engine::ContractGuard::verify_modification(&contract, &results) {
                Ok(shadow_fidelity) => {
                    println!("\n--- [CONTRACT VERIFICATION RESULT] ---");
                    println!("✅  Integrity Check: PASSED");
                    println!("🧮  Predicted Fidelity: {:.4}", shadow_fidelity);
                    println!("🚀  Ready for Surgical Patch.");
                },
                Err(e) => {
                    println!("\n--- [CONTRACT VERIFICATION RESULT] ---");
                    println!("❌  Integrity Check: FAILED");
                    println!("⚠️   Error: {}", e);
                }
            }
        }
        Commands::Glossary { path, id, alias } => {
            crate::engine::GlossaryEngine::set_alias(path, id, alias)?;
        }
        Commands::Wiki { path, target } => {
            let results = Scanner::scan_for_verification(path)?;
            let glossary = crate::engine::GlossaryEngine::load(path)?;

            for (p, feat) in results {
                if p == *target {
                    let wiki_page = crate::engine::WikiProxy::generate_page(path, &p, &feat, &glossary);
                    println!("{}", wiki_page);
                    return Ok(());
                }
            }
            println!("❌  Target '{}' not found in map.", target);
        }
        Commands::Prove { path } => {
            let results = Scanner::scan_for_verification(path)?;
            crate::engine::CCAPProver::run_physical_proofs(path, &results)?;
        }
    }

    Ok(())
}
