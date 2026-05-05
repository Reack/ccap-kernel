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
    pub gravity: f32,    // 幾何引力 (特徵中心度)
    pub ghost_debt: u32, // 冗餘引用計數
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_data_serialization() {
        let data = ProjectWikiData {
            project_name: "TestProject".to_string(),
            total_files: 1,
            rooms: vec![RoomData {
                id: 0,
                label: "Core".to_string(),
                value: 1,
                keywords: vec!["logic".to_string()],
                soul: "Brain".to_string(),
                members: vec![MemberData {
                    path: "src/main.rs".to_string(),
                    label: "main".to_string(),
                    is_hub: true,
                    complexity: 0.5,
                }],
            }],
            ..Default::default()
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("TestProject"));
        assert!(json.contains("Core"));
        assert!(json.contains("src/main.rs"));
    }

    #[test]
    fn test_html_rendering_basics() {
        let data = ProjectWikiData {
            project_name: "MockProject".to_string(),
            ..Default::default()
        };
        let html = WikiTemplate::render_project_wiki(&data);
        assert!(html.contains("MockProject"));
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("vis-network"));
        assert!(html.contains("AEB98F")); // Our signature color
    }
}
