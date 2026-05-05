use crate::engine::{extractor::FileFeatures, glossary::Glossary, Linker, MathEngine, Mapper};
use super::{ProjectWikiData, RoomData, MemberData};
use std::collections::HashMap;
use serde_json::json;
use regex::Regex;

pub struct WikiGenerator;

impl WikiGenerator {
    /// Cleans HTML tags from a string to prevent layout breaking and unwanted navigation.
    fn sanitize_text(text: &str) -> String {
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(text, "").to_string()
    }

    pub fn build_project_data(
        repo_root: &str,
        results: &[(String, FileFeatures)],
        linker: &Linker,
        glossary: &Glossary,
    ) -> ProjectWikiData {
        let project_name = std::path::Path::new(repo_root).file_name().unwrap_or_default().to_string_lossy().to_string();
        
        let mut data = ProjectWikiData {
            project_name,
            total_files: results.len(),
            ..Default::default()
        };

        // 1. Native Docs with Sanitization
        for name in &["README.md", "readme.md", "PROJECT_SUMMARY.md"] {
            let path = std::path::Path::new(repo_root).join(name);
            if let Ok(content) = std::fs::read_to_string(path) {
                let raw_summary = content.lines().take(10).collect::<Vec<_>>().join(" ");
                data.native_docs = Some(Self::sanitize_text(&raw_summary));
                break;
            }
        }

        // 2. Project Soul
        let soul_path = std::path::Path::new(repo_root).join(".ccap").join("project_soul.st");
        data.project_soul = std::fs::read_to_string(soul_path).ok();

        // 3. Cluster Analysis
        let edges = linker.export_edges();
        let paths = linker.get_node_paths();
        let mut sym_data = HashMap::new();
        for (path, features) in results { sym_data.insert(path.clone(), features.top_symbols.clone()); }

        let k_rooms = 3.max(results.len() / 50).min(10);
        let clusters = MathEngine::spectral_cluster(&paths, &edges, k_rooms, &sym_data).unwrap_or_default();

        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.members.is_empty() { continue; }
            
            let mut kw_count: HashMap<String, usize> = HashMap::new();
            for m in &cluster.members {
                if let Some(syms) = sym_data.get(m) {
                    for s in syms { *kw_count.entry(s.to_lowercase()).or_insert(0) += 1; }
                }
            }
            let mut skw: Vec<_> = kw_count.into_iter().collect();
            skw.sort_by(|a, b| b.1.cmp(&a.1));

            let mut soul = String::new();
            let rs_path = std::path::Path::new(repo_root).join(".ccap").join(format!("room_soul_{}.st", i));
            if let Ok(s) = std::fs::read_to_string(rs_path) { soul = s.trim().to_string(); }

            let members: Vec<_> = cluster.members.iter()
                .filter(|m| !m.contains("docs"))
                .take(30)
                .map(|m| {
                    let feat = results.iter().find(|(p, _)| p == m).map(|x| &x.1);
                    
                    // 1. 計算引力 (基於入度)
                    let gravity = edges.iter()
                        .filter(|(_, v_idx, _)| {
                            let v_path = &paths[*v_idx];
                            v_path == m
                        })
                        .count() as f32;

                    // 2. 計算冗餘債務 (權重 0.1 的邊)
                    let ghost_debt = edges.iter()
                        .filter(|(u_idx, _, w)| {
                            let u_path = &paths[*u_idx];
                            u_path == m && *w < 0.5
                        })
                        .count() as u32;

                    MemberData {
                        path: m.clone(),
                        label: glossary.aliases.get(m).cloned().unwrap_or(m.clone()),
                        is_hub: feat.map_or(false, |f| f.symbol_count > 10 || gravity > 5.0),
                        complexity: feat.map_or(0.0, |f| f.control_flow_score),
                        gravity,
                        ghost_debt,
                    }
                }).collect();

            data.rooms.push(RoomData {
                id: i,
                label: cluster.name.clone(),
                value: cluster.members.len(),
                keywords: skw.iter().take(10).map(|x| x.0.clone()).collect(),
                soul,
                members,
            });
        }
        data
    }

    pub fn generate_project_index(
        repo_root: &str,
        results: &[(String, FileFeatures)],
        linker: &Linker,
        glossary: &Glossary,
    ) -> String {
        let data = Self::build_project_data(repo_root, results, linker, glossary);
        let mut wiki = String::new();
        wiki.push_str(&format!("# 🌌 專案戰略地圖：{} \n\n", data.project_name));

        if let Some(docs) = &data.native_docs {
            wiki.push_str("### 📖 專案原生文檔 (Native Documentation)\n");
            wiki.push_str(&format!("> {}\n\n", docs));
        }

        if let Some(soul) = &data.project_soul {
            wiki.push_str("### 🎭 專案靈魂故事 (AI Project Soul)\n");
            wiki.push_str(&format!("> {}\\n\\n", soul.trim()));
        }

        wiki.push_str("### 🏗️ 互動式戰略導航 (Hierarchical Map Container)\n");
        wiki.push_str("<div id='mynetwork' style='height: 600px; border: 2px solid #AEB98F; background-color: #ffffff; border-radius: 16px; margin-bottom: 30px;'></div>\n\n");
        
        let graph_data_json = json!({ "rooms": data.rooms }).to_string();
        wiki.push_str(&format!("@ROOM_DATA_START@{}@ROOM_DATA_END@\n", graph_data_json));

        wiki.push_str("<div id='highlight-container'></div>\n");
        wiki
    }

    pub fn generate_markdown(target_path: &str, features: &FileFeatures, glossary: &Glossary) -> String {
        let mut wiki = String::new();
        let title = glossary.aliases.get(target_path).unwrap_or(&target_path.to_string()).clone();
        
        // 判定角色與引力
        let role = if features.symbol_count > 10 { "核心樞紐 (HUB)" } else { "功能節點 (LEAF)" };
        
        wiki.push_str(&format!("# 📄 {} \n\n", title));
        wiki.push_str(&format!("**物理路徑**: `{}`\n", target_path));
        wiki.push_str(&format!("**架構角色**: `{}`\n", role));
        
        wiki.push_str("\n### ⚖️ 幾何指標 (Spectral Metrics)\n");
        wiki.push_str(&format!("*   **邏輯複雜度**: `{:.2}`\n", features.control_flow_score));
        
        // 這裡暫時無法從 features 拿到 gravity，我們使用 symbol_count 作為啟發式替代
        // 在 build_project_data 中已經有了完整的 gravity 數據，這裡如果是 Wiki 靜態頁，
        // 我們應考慮未來將 Linker 狀態傳入。
        
        wiki.push_str("\n### 🔗 SCIP Exports\n");
        for sym in &features.exports {
            wiki.push_str(&format!("*   `{}` (L{})\n", sym.name, sym.line));
        }
        
        wiki.push_str("\n---\n*Generated by CCAP v0.2.0-dev | Research-Backed Diagnostic Engine*");
        wiki
    }

    pub fn generate_ai_enrich_prompt(target_path: &str, features: &FileFeatures) -> String {
        let telegram = Mapper::to_telegram(target_path, features);
        format!("[CCAP AEP] TARGET: {}\nTELEGRAM: {}\nTASK: Professional summary.", target_path, telegram)
    }

    pub fn generate_global_ai_package(results: &[(String, FileFeatures)]) -> String {
        let mut p = String::from("[CCAP GLOBAL SYNTHESIS]\n");
        for (path, feat) in results { p.push_str(&format!("{}\n", Mapper::to_telegram(path, feat))); }
        p
    }
}
