use tree_sitter::{Parser, Node};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileFeatures {
    pub control_flow_score: f32,
    pub data_density_score: f32,
    pub io_density_score: f32,
    pub symbol_count: usize,
    pub top_symbols: Vec<String>,
    pub imports: Vec<String>,
}

pub struct Extractor {
    parser: Parser,
}

impl Extractor {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        let language = tree_sitter_python::language();
        parser.set_language(language).expect("Error loading Python grammar");
        Self { parser }
    }

    pub fn analyze_python(&mut self, file_path: &str) -> anyhow::Result<FileFeatures> {
        let code = fs::read_to_string(file_path)?;
        let tree = self.parser.parse(&code, None).ok_or_else(|| anyhow::anyhow!("Parse failed"))?;
        
        let mut features = FileFeatures::default();
        let mut raw_io_count = 0;
        
        self.traverse_node(tree.root_node(), &code, &mut features, &mut raw_io_count);

        // Normalize scores
        features.io_density_score = (raw_io_count as f32 / 5.0).min(1.0);
        features.control_flow_score = (features.control_flow_score / 15.0).min(1.0);
        features.data_density_score = (features.data_density_score / 3.0).min(1.0);

        Ok(features)
    }

    fn traverse_node(&self, node: Node, code: &str, f: &mut FileFeatures, io_count: &mut i32) {
        let kind = node.kind();
        
        match kind {
            "if_statement" | "for_statement" | "while_statement" | "try_statement" => {
                f.control_flow_score += 1.0;
            },
            "class_definition" => {
                f.data_density_score += 1.0;
            },
            "function_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &code[name_node.byte_range()];
                    if !name.starts_with('_') && f.top_symbols.len() < 5 {
                        f.top_symbols.push(name.to_string());
                    }
                }
                f.symbol_count += 1;
            },
            "import_statement" => {
                // Find all dotted names in this statement
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "dotted_name" {
                        f.imports.push(code[child.byte_range()].to_string());
                    }
                }
            },
            "import_from_statement" => {
                // The module being imported from is usually the first dotted_name
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "dotted_name" {
                        f.imports.push(code[child.byte_range()].to_string());
                        break; // Only take the module source
                    }
                }
            },
            "call" => {
                let text = &code[node.byte_range()];
                if text.contains("open") || text.contains("print") || text.contains("read") {
                    *io_count += 1;
                }
            }
            _ => {}
        }

        // Recursive traversal
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.traverse_node(child, code, f, io_count);
        }
    }
}
