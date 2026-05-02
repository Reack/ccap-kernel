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
    pub exports: Vec<ScipSymbol>,
    pub references: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScipSymbol {
    pub id: String,
    pub name: String,
    pub line: usize,
}

pub struct Extractor {
    parser: Parser,
}

impl Extractor {
    pub fn new() -> Self {
        Self { parser: Parser::new() }
    }

    pub fn analyze_file(&mut self, file_path: &str, rel_path: &str) -> anyhow::Result<FileFeatures> {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let language = match extension {
            "py" => tree_sitter_python::language(),
            "js" | "jsx" | "ts" | "tsx" => tree_sitter_javascript::language(),
            _ => return Err(anyhow::anyhow!("Unsupported language: .{}", extension)),
        };

        // Normalize module path (remove extensions for SCIP compatibility)
        let mod_path = rel_path.replace("\\", "/")
            .trim_end_matches(".py")
            .trim_end_matches(".js")
            .trim_end_matches(".ts")
            .to_string();

        self.parser.set_language(language)?;
        let code = fs::read_to_string(file_path)?;
        let tree = self.parser.parse(&code, None).ok_or_else(|| anyhow::anyhow!("Parse failed"))?;
        
        let mut f = FileFeatures::default();
        let mut raw_io_count = 0;
        let mut scope_stack = vec![mod_path];

        self.traverse_node(tree.root_node(), &code, &mut f, &mut raw_io_count, extension, &mut scope_stack);

        f.io_density_score = (raw_io_count as f32 / 5.0).min(1.0);
        f.control_flow_score = (f.control_flow_score / 15.0).min(1.0);
        f.data_density_score = (f.data_density_score / 3.0).min(1.0);

        Ok(f)
    }

    fn traverse_node(&self, node: Node, code: &str, f: &mut FileFeatures, io_count: &mut i32, ext: &str, scope: &mut Vec<String>) {
        let kind = node.kind();
        let mut current_symbol = None;

        match (ext, kind) {
            (_, "class_definition") | ("js", "class_declaration") => {
                f.data_density_score += 1.0;
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = code[name_node.byte_range()].to_string();
                    current_symbol = Some(name);
                }
            },
            (_, "function_definition") | ("js", "function_declaration") | ("js", "method_definition") => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = code[name_node.byte_range()].to_string();
                    current_symbol = Some(name);
                }
                f.symbol_count += 1;
            },
            ("py", "import_statement") | ("py", "import_from_statement") => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "dotted_name" {
                        let name = code[child.byte_range()].replace(".", "/");
                        f.references.push(format!("ccap . . {}#", name));
                    }
                }
            },
            ("js", "import_statement") => {
                if let Some(source) = node.child_by_field_name("source") {
                    let path = code[source.byte_range()]
                        .trim_matches(|c| c == '"' || c == '\'' || c == '.')
                        .trim_start_matches('/')
                        .to_string();
                    f.references.push(format!("ccap . . {}#", path));
                }
            },
            (_, "call") | ("js", "call_expression") => {
                let text = &code[node.byte_range()];
                if text.contains("open") || text.contains("print") || text.contains("fetch") {
                    *io_count += 1;
                }
            },
            _ => {}
        }

        if let Some(name) = current_symbol {
            let scip_id = format!("ccap . . {}#{}#", scope.join("#"), name);
            
            f.exports.push(ScipSymbol {
                id: scip_id,
                name: name.clone(),
                line: node.start_position().row + 1,
            });

            if f.top_symbols.len() < 5 && !name.starts_with('_') {
                f.top_symbols.push(name.clone());
            }

            scope.push(name);
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.traverse_node(child, code, f, io_count, ext, scope);
            }
            scope.pop();
        } else {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.traverse_node(child, code, f, io_count, ext, scope);
            }
        }
    }
}
