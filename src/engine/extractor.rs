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
            .unwrap_or("")
            .to_lowercase();

        let language = match extension.as_str() {
            "py" => tree_sitter_python::language(),
            "js" | "jsx" | "ts" | "tsx" => tree_sitter_javascript::language(),
            "c" | "h" => tree_sitter_c::language(),
            "cpp" | "hpp" | "cc" | "hh" => tree_sitter_cpp::language(),
            "rs" => tree_sitter_rust::language(),
            "go" => tree_sitter_go::language(),
            "java" => tree_sitter_java::language(),
            "cs" => tree_sitter_c_sharp::language(),
            _ => return Err(anyhow::anyhow!("Unsupported language: .{}", extension)),
        };

        // Normalize module path for SCIP
        let mod_path = rel_path.replace("\\", "/");
        let mod_path = mod_path.split('.').next().unwrap_or(&mod_path).to_string();

        self.parser.set_language(language)?;
        let code = fs::read_to_string(file_path)?;
        let tree = self.parser.parse(&code, None).ok_or_else(|| anyhow::anyhow!("Parse failed"))?;
        
        let mut f = FileFeatures::default();
        let mut raw_io_count = 0;
        let mut scope_stack = vec![mod_path];

        self.traverse_node(tree.root_node(), &code, &mut f, &mut raw_io_count, &extension, &mut scope_stack);

        f.io_density_score = (raw_io_count as f32 / 5.0).min(1.0);
        f.control_flow_score = (f.control_flow_score / 15.0).min(1.0);
        f.data_density_score = (f.data_density_score / 3.0).min(1.0);

        Ok(f)
    }

    fn traverse_node(&self, node: Node, code: &str, f: &mut FileFeatures, io_count: &mut i32, ext: &str, scope: &mut Vec<String>) {
        let kind = node.kind();
        let mut current_symbol = None;

        // --- UNIVERSAL SEMANTIC MAPPING ENGINE ---
        match (ext, kind) {
            // 1. DATA / CLASS DEFINITIONS
            (_, "class_definition") | (_, "class_declaration") | (_, "struct_specifier") |
            ("rs", "struct_item") | ("rs", "enum_item") | ("go", "type_declaration") => {
                f.data_density_score += 1.0;
                if let Some(name_node) = node.child_by_field_name("name") {
                    current_symbol = Some(code[name_node.byte_range()].to_string());
                }
            },

            // 2. LOGIC / FUNCTION DEFINITIONS
            (_, "function_definition") | (_, "function_declaration") | (_, "method_definition") |
            ("rs", "function_item") | ("go", "function_declaration") | ("cs", "method_declaration") => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    current_symbol = Some(code[name_node.byte_range()].to_string());
                }
                f.symbol_count += 1;
            },

            // 3. CONTROL FLOW
            (_, "if_statement") | (_, "for_statement") | (_, "while_statement") | (_, "try_statement") |
            ("rs", "if_expression") | ("rs", "for_expression") | ("rs", "match_expression") => {
                f.control_flow_score += 1.0;
            },

            // 4. IMPORTS / REFERENCES (SCIP Normalization)
            ("py", "import_statement") | ("py", "import_from_statement") |
            ("rs", "use_declaration") | ("go", "import_declaration") | ("java", "import_declaration") => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind().contains("name") || child.kind().contains("path") {
                        let name = code[child.byte_range()].replace(".", "/").replace("::", "/");
                        f.references.push(format!("ccap . . {}#", name.trim_matches(':')));
                        break;
                    }
                }
            },
            ("js", "import_statement") | ("ts", "import_statement") => {
                if let Some(source) = node.child_by_field_name("source") {
                    let path = code[source.byte_range()].trim_matches(|c| c == '"' || c == '\'' || c == '.').to_string();
                    f.references.push(format!("ccap . . {}#", path));
                }
            },
            ("c", "preproc_include") | ("cpp", "preproc_include") => {
                if let Some(path) = node.child(1) {
                    let name = code[path.byte_range()].trim_matches(|c| c == '<' || c == '>' || c == '"').to_string();
                    f.references.push(format!("ccap . . {}#", name));
                }
            },

            // 5. I/O HEURISTICS
            (_, "call") | (_, "call_expression") | ("rs", "call_expression") => {
                let text = &code[node.byte_range()].to_lowercase();
                if text.contains("open") || text.contains("print") || text.contains("fetch") || 
                   text.contains("socket") || text.contains("http") || text.contains("io") {
                    *io_count += 1;
                }
            },
            _ => {}
        }

        // --- SCOPE & SCIP ID GENERATION ---
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
