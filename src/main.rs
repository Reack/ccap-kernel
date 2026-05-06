use clap::{Parser, Subcommand};
use std::fs;

use ccap_kernel::engine::{Mapper, Scanner, Benchmark, Linker, Verifier};


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
    /// Verifies the correctness and parity.
    Verify {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// External SCIP index path (optional)
        #[arg(long)]
        scip: Option<String>,
    },
    /// Traces dependencies or calculates modification impact.
    Trace {
        /// Symbol or file path to trace
        target: String,
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
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
        /// External atlas JSON path to audit instead of local path
        #[arg(long)]
        from_json: Option<String>,
    },
    /// Performs a scientific benchmark of information density (MDL).
    Benchmark {
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
    /// Generates a semantic Wiki page for the project or a given target.
    Wiki {
        /// Repository root path
        #[arg(default_value = ".")]
        path: String,
        /// Target file path to view (optional). If omitted, generates project index.
        #[arg(short, long)]
        target: Option<String>,
        /// Generate and open HTML view
        #[arg(long)]
        html: bool,
        /// Generate a high-entropy prompt for AI enrichment (Chapter 15)
        #[arg(long)]
        ai_enrich: bool,
        /// Package the entire project's semantic data for global AI synthesis
        #[arg(long)]
        ai_enrich_all: bool,
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
        Commands::Verify { path, scip } => {
            println!("🔍  Verification: Initiating Formal Parity Check for: {}", path);
            let mut results = Scanner::scan_for_verification(path)?;

            if let Some(s) = scip {
                println!("🧬  SCIP Injection: Using external index from: {}", s);
                ccap_kernel::engine::ScipConsumer::inject_semantics(&s, &mut results)?;
            }

            let mut linker = Linker::new();
            linker.build_graph(&results);
            let edges = linker.export_edges();
            let n = results.len();
            
            let report = Verifier::verify_scip_ids(path, &results);
            let fidelity = Verifier::calculate_fidelity(n, &edges);
            let accuracy = Verifier::verify_topological_accuracy(&linker, &results);

            println!("\n====================================================");
            println!("🛡️   CCAP FORMAL VERIFICATION REPORT");
            println!("====================================================");
            println!("📊  Semantic Confidence:     {:.1}%", report.confidence_score * 100.0);
            println!("⚖️   SCIP Syntax Format:      {}", if report.format_passed { "PASSED" } else { "FAILED" });
            println!("🆔  Symbol Uniqueness:       {}", if report.symbol_collisions == 0 { "PASSED" } else { "FAILED" });
            
            if report.symbol_collisions > 0 {
                println!("🛑  Symbol Ambiguities:      {}", report.symbol_collisions);
                for detail in report.collision_details.iter().take(5) {
                    println!("    ⚠️   {}", detail);
                }
                if report.symbol_collisions > 5 {
                    println!("    ... and {} more ambiguities.", report.symbol_collisions - 5);
                }

                println!("\n💡  SEMANTIC REPAIR GUIDE:");
                println!("    Topological ambiguities detected. For professional surgical precision,");
                println!("    please provide a language-specific SCIP index:");
                for tool in &report.recommended_tools {
                    println!("    🛠️   {}", tool);
                }
            }

            println!("\n🧮  Algebraic Connectivity:  {:.4}", fidelity);
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
                        ccap_kernel::engine::Patcher::apply_patch(path, file, sym, code)?;
                        return Ok(());
                    }
                }
            }
            println!("❌  Symbol ID not found for patching.");
        }
        Commands::Audit { path, from_json } => {
            println!("🔬  Audit: Initiating Standards-Compliant Quality Assessment for: {}", path);
            
            let (results, n) = if let Some(json_path) = from_json {
                println!("📂  JSON Audit: Loading external atlas from: {}", json_path);
                let json_data = fs::read_to_string(json_path)?;
                let atlas: serde_json::Value = serde_json::from_str(&json_data)?;
                
                let mut mock_results = Vec::new();
                if let Some(nodes) = atlas.get("nodes").and_then(|n| n.as_array()) {
                    for node in nodes {
                        if let Some(id) = node.get("id").and_then(|i| i.as_str()) {
                            let mut feat = ccap_kernel::engine::extractor::FileFeatures::default();
                            if let Some(exports) = node.get("features").and_then(|f| f.get("exports")).and_then(|e| e.as_array()) {
                                for exp in exports {
                                    if let Some(s_id) = exp.get("id").and_then(|s| s.as_str()) {
                                        feat.exports.push(ccap_kernel::engine::extractor::ScipSymbol {
                                            id: s_id.to_string(),
                                            name: exp.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                                            ..Default::default()
                                        });
                                    }
                                }
                            }
                            if let Some(refs) = node.get("features").and_then(|f| f.get("references")).and_then(|r| r.as_array()) {
                                for r in refs {
                                    if let Some(r_str) = r.as_str() {
                                        let linker_friendly = if !r_str.contains("ccap . . ") { format!("ccap . . {}", r_str) } else { r_str.to_string() };
                                        feat.references.push(linker_friendly);
                                    }
                                }
                            }
                            mock_results.push((id.to_string(), feat));
                        }
                    }
                }
                let count = mock_results.len();
                (mock_results, count)
            } else {
                let r = Scanner::scan_for_verification(path)?;
                let count = r.len();
                (r, count)
            };

            let mut linker = Linker::new();
            linker.build_graph(&results);
            let report = ccap_kernel::engine::Evaluator::perform_audit(&linker, n);

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
            let storage = ccap_kernel::engine::Storage::init(path)?;
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
            
            let atlas = ccap_kernel::engine::Exporter::export_atlas(path, &results, &linker)?;
            ccap_kernel::engine::Exporter::save_to_file(&atlas, output)?;
        }
        Commands::Analyze { path } => {
            let mut extractor = ccap_kernel::engine::Extractor::new();
            let features = extractor.analyze_file(path, path)?;
            let telegram = Mapper::to_telegram(path, &features);
            println!("{}", telegram);
        }
        Commands::InspectRoom { path, room } => {
            let storage = ccap_kernel::engine::Storage::init(path)?;
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
            let cleaned = ccap_kernel::engine::ProxyEngine::clean_output(&stdout, &actual_path);
            println!("{}", cleaned);
        }
        Commands::Contract { path, target, code } => {
            println!("📑  Contract: Initiating Shadow Execution for mutation on: {}", target);
            let results = Scanner::scan_for_verification(path)?;

            let contract = ccap_kernel::engine::contract::ModificationContract {
                target_id: target.clone(),
                action: "PATCH".to_string(),
                code_snippet: code.clone(),
            };

            match ccap_kernel::engine::ContractGuard::verify_modification(&contract, &results) {
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
            ccap_kernel::engine::GlossaryEngine::set_alias(path, id, alias)?;
        }
        Commands::Wiki { path, target, html, ai_enrich, ai_enrich_all } => {
            let results = Scanner::scan_for_verification(path)?;
            let glossary = ccap_kernel::engine::GlossaryEngine::load(path)?;

            if *ai_enrich_all {
                let package = ccap_kernel::engine::WikiProxy::generate_global_ai_package(&results);
                println!("{}", package);
                return Ok(());
            }

            let mut linker = Linker::new();
            linker.build_graph(&results);

            let markdown = if let Some(t) = target {
                let mut found = None;
                for (p, feat) in &results {
                    if p == t {
                        if *ai_enrich {
                            let prompt = ccap_kernel::engine::WikiProxy::generate_ai_enrich_prompt(p, feat);
                            println!("{}", prompt);
                            return Ok(());
                        }
                        found = Some(ccap_kernel::engine::WikiProxy::generate_markdown(p, feat, &glossary));
                        break;
                    }
                }
                if let Some(md) = found { md } else {
                    println!("❌  Target '{}' not found in map.", t);
                    return Ok(());
                }
            } else {
                ccap_kernel::engine::WikiProxy::generate_project_index(path, &results, &linker, &glossary)
            };

            if *html {
                let html_content = ccap_kernel::engine::WikiProxy::generate_html_wiki(path, &results, &linker, &glossary)?;
                let tmp_path = std::env::temp_dir().join("ccap_wiki.html");

                fs::write(&tmp_path, html_content)?;
                println!("🌐  Wiki: Opening beautiful HTML view at {:?}", tmp_path);
                let _ = std::process::Command::new("cmd").args(["/c", "start", tmp_path.to_str().unwrap()]).spawn();
            } else {
                println!("{}", markdown);
            }
            return Ok(());
        }
        Commands::Prove { path } => {
            let results = Scanner::scan_for_verification(path)?;
            ccap_kernel::engine::CCAPProver::run_physical_proofs(path, &results)?;
        }
    }

    Ok(())
}
