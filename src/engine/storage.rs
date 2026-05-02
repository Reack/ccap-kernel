use std::fs;
use std::path::{Path, PathBuf};
use crate::engine::extractor::FileFeatures;
use crate::engine::security::SecurityEngine;

pub struct Storage {
    root: PathBuf,
    security: Option<SecurityEngine>,
}

impl Storage {
    pub fn init(repo_path: &str) -> anyhow::Result<Self> {
        let root = Path::new(repo_path).join(".ccap");
        
        if !root.exists() {
            fs::create_dir_all(&root)?;
            println!("🛡️  Shadow directory created at: {:?}", root);
        }

        fs::create_dir_all(root.join("maps"))?;
        fs::create_dir_all(root.join("snapshots"))?;

        let gitignore_path = root.join(".gitignore");
        if !gitignore_path.exists() {
            fs::write(gitignore_path, "*\n!.gitignore")?;
        }

        Ok(Self { root, security: None })
    }

    pub fn with_security(mut self, key: &str) -> Self {
        self.security = Some(SecurityEngine::new(key));
        self
    }

    pub fn get_map_dir(&self) -> PathBuf {
        self.root.join("maps")
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.root.join("index.db")
    }

    pub fn save_map(&self, rel_path: &str, telegram: &str, features: &FileFeatures) -> anyhow::Result<()> {
        let safe_path = rel_path.replace("\\", "/").replace(":", "_");
        let map_dir = self.get_map_dir().join(&safe_path);
        fs::create_dir_all(&map_dir)?;

        let mut final_features = features.clone();

        if let Some(_) = &self.security {
            final_features.top_symbols = final_features.top_symbols
                .iter()
                .map(|s| SecurityEngine::obfuscate_symbol(s))
                .collect();
            
            let mut vec = [
                final_features.control_flow_score,
                final_features.data_density_score,
                final_features.io_density_score
            ];
            SecurityEngine::perturb_vector(&mut vec, 0.02);
            final_features.control_flow_score = vec[0];
            final_features.data_density_score = vec[1];
            final_features.io_density_score = vec[2];
        }

        let md_content = telegram.as_bytes();
        let md_path = map_dir.join("_MAP.md");
        let meta_json = serde_json::to_string_pretty(&final_features)?;
        let meta_content = meta_json.as_bytes();
        let meta_path = map_dir.join("_MAP.meta.json");

        if let Some(sec) = &self.security {
            let enc_md = sec.encrypt(md_content)?;
            let enc_meta = sec.encrypt(meta_content)?;
            fs::write(md_path.with_extension("md.enc"), enc_md)?;
            fs::write(meta_path.with_extension("json.enc"), enc_meta)?;
            let _ = fs::remove_file(&md_path);
            let _ = fs::remove_file(&meta_path);
        } else {
            fs::write(md_path, md_content)?;
            fs::write(meta_path, meta_content)?;
        }

        let telemetry_path = self.root.join("telemetry.log");
        let log_entry = format!("FILE:{}|RAW_SIZE:{}|MAP_SIZE:{}\n", rel_path, features.symbol_count * 100, md_content.len());
        let _ = fs::OpenOptions::new().create(true).append(true).open(telemetry_path).map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "{}", log_entry);
        });

        Ok(())
    }
}
