use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Glossary {
    pub aliases: HashMap<String, String>, // SCIP_ID -> Human-readable Name
}

pub struct GlossaryEngine;

impl GlossaryEngine {
    pub fn load(repo_root: &str) -> anyhow::Result<Glossary> {
        let path = std::path::Path::new(repo_root).join(".ccap").join("glossary.json");
        if path.exists() {
            let json = fs::read_to_string(path)?;
            let glossary: Glossary = serde_json::from_str(&json)?;
            Ok(glossary)
        } else {
            Ok(Glossary::default())
        }
    }

    pub fn save(repo_root: &str, glossary: &Glossary) -> anyhow::Result<()> {
        let dir = std::path::Path::new(repo_root).join(".ccap");
        fs::create_dir_all(&dir)?;
        let path = dir.join("glossary.json");
        let json = serde_json::to_string_pretty(glossary)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn set_alias(repo_root: &str, id: &str, alias: &str) -> anyhow::Result<()> {
        let mut glossary = Self::load(repo_root)?;
        glossary.aliases.insert(id.to_string(), alias.to_string());
        Self::save(repo_root, &glossary)?;
        println!("📝  Glossary: Alias set for {} -> {}", id, alias);
        Ok(())
    }
}
