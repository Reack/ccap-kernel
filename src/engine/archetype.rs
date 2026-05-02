use std::collections::{HashMap, HashSet};

pub struct ArchetypeEngine;

impl ArchetypeEngine {
    /// Infers an engineering role for a cluster based on its symbol distribution.
    pub fn infer_cluster_name(keywords: &HashSet<String>, cluster_id: usize) -> String {
        let patterns = [
            (vec!["auth", "jwt", "login", "security", "permission"], "Security_Gateway"),
            (vec!["db", "sql", "model", "schema", "entity", "repository"], "Data_Persistence_Layer"),
            (vec!["api", "route", "controller", "endpoint", "http", "request"], "API_Interface_Hub"),
            (vec!["test", "mock", "stub", "spec", "bench"], "Quality_Assurance_Zone"),
            (vec!["util", "helper", "common", "tool", "format"], "Utility_Toolbox"),
            (vec!["service", "logic", "engine", "core", "handler"], "Business_Logic_Core"),
            (vec!["ui", "view", "component", "style", "css", "html"], "Frontend_Presentation"),
        ];

        for (hints, role) in patterns {
            if hints.iter().any(|h| keywords.contains(&h.to_string())) {
                return format!("{}_{}", role, cluster_id);
            }
        }

        format!("Semantic_Room_{}", cluster_id)
    }
}
