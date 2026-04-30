use std::fs;
use std::path::{Path, PathBuf};
use crate::engine::extractor::FileFeatures;

pub struct Storage {
    root: PathBuf,
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

        Ok(Self { root })
    }

    pub fn get_map_dir(&self) -> PathBuf {
        self.root.join("maps")
    }

    pub fn save_map(&self, rel_path: &str, telegram: &str, features: &FileFeatures) -> anyhow::Result<()> {
        // Create a safe directory name from the relative path
        let safe_path = rel_path.replace("\\", "/").replace(":", "_");
        let map_dir = self.get_map_dir().join(&safe_path);
        
        fs::create_dir_all(&map_dir)?;

        // 1. Write _MAP.md (Semantic Telegram - SCA)
        let md_path = map_dir.join("_MAP.md");
        fs::write(md_path, telegram)?;

        // 2. Write _MAP.meta.json (Physical Metadata - VNM)
        let meta_path = map_dir.join("_MAP.meta.json");
        let meta_json = serde_json::to_string_pretty(features)?;
        fs::write(meta_path, meta_json)?;

        Ok(())
    }
}
