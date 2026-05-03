use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProjectWikiData {
    pub project_name: String,
    pub total_files: usize,
    pub native_docs: Option<String>,
    pub project_soul: Option<String>,
    pub rooms: Vec<RoomData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RoomData {
    pub id: usize,
    pub label: String,
    pub value: usize,
    pub keywords: Vec<String>,
    pub soul: String,
    pub members: Vec<MemberData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemberData {
    pub path: String,
    pub label: String,
    pub is_hub: bool,
    pub complexity: f32,
}

pub mod generator;
pub mod template;

use crate::engine::{extractor::FileFeatures as InternalFeatures, glossary::Glossary, Linker};
pub use generator::WikiGenerator;
pub use template::WikiTemplate;

pub struct WikiProxy;

impl WikiProxy {
    pub fn generate_markdown(target_path: &str, features: &InternalFeatures, glossary: &Glossary) -> String {
        WikiGenerator::generate_markdown(target_path, features, glossary)
    }

    pub fn generate_project_index(repo_root: &str, results: &[(String, InternalFeatures)], linker: &Linker, glossary: &Glossary) -> String {
        WikiGenerator::generate_project_index(repo_root, results, linker, glossary)
    }

    pub fn generate_ai_enrich_prompt(target_path: &str, features: &InternalFeatures) -> String {
        WikiGenerator::generate_ai_enrich_prompt(target_path, features)
    }

    pub fn generate_global_ai_package(results: &[(String, InternalFeatures)]) -> String {
        WikiGenerator::generate_global_ai_package(results)
    }

    pub fn generate_html_wiki(repo_root: &str, results: &[(String, InternalFeatures)], linker: &Linker, glossary: &Glossary) -> anyhow::Result<String> {
        let data = WikiGenerator::build_project_data(repo_root, results, linker, glossary);
        Ok(WikiTemplate::render_project_wiki(&data))
    }
}
