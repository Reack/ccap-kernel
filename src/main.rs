use clap::{Parser, Subcommand};
mod engine;

use crate::engine::{Extractor, Mapper, Scanner};

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
        /// Repository root path (default: current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Force full deep scan of all modules
        #[arg(short, long)]
        deep: bool,
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
        Commands::Init { path, deep } => {
            Scanner::scan_project(path, *deep)?;
        }
        Commands::Analyze { path } => {
            let mut extractor = Extractor::new();
            let features = extractor.analyze_python(path)?;
            let telegram = Mapper::to_telegram(path, &features);
            println!("{}", telegram);
        }
    }

    Ok(())
}
