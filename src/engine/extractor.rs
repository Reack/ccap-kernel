use tree_sitter::{Parser, Node};
use std::fs;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HalsteadMetrics {
    pub n1: usize, // unique operators
    pub n2: usize, // unique operands
    pub n1_total: usize, // total operators
    pub n2_total: usize, // total operands
    pub volume: f32,
    pub effort: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileFeatures {
    pub control_flow_score: f32,
    pub data_density_score: f32,
    pub io_density_score: f32,
    pub symbol_count: usize,
    pub top_symbols: Vec<String>,
    pub exports: Vec<ScipSymbol>,
    pub references: Vec<String>,
    pub halstead: HalsteadMetrics, // v0.2.0: Information Theory Suite
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ScipSymbol {
    pub id: String,
    pub name: String,
    pub line: usize,
    pub range: (usize, usize),
    pub end_line: usize,
    pub gravity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Cluster {
    pub name: String,
    pub members: Vec<String>,
}

pub struct Extractor {
    parser: Parser,
    py_def_regex: Regex,
}

impl Extractor {
    pub fn new() -> Self {
        Self { 
            parser: Parser::new(),
            py_def_regex: Regex::new(r"(?m)^\s*(?:async\s+)?(?:class|def)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap(),
        }
    }

    pub fn analyze_file(&mut self, file_path: &str, rel_path: &str) -> anyhow::Result<FileFeatures> {
        let extension = self.get_extension(file_path);
        let language = match extension.as_str() {
            "py" => tree_sitter_python::language(),
            "js" | "jsx" | "ts" | "tsx" => tree_sitter_javascript::language(),
            "rs" => tree_sitter_rust::language(),
            "go" => tree_sitter_go::language(),
            "java" => tree_sitter_java::language(),
            "cs" => tree_sitter_c_sharp::language(),
            _ => tree_sitter_c::language(),
        };

        let mod_path = rel_path.replace("\\", "/").trim_start_matches("../").trim_start_matches("./").to_string();
        self.parser.set_language(language)?;
        let code = fs::read_to_string(file_path)?;
        let tree = self.parser.parse(&code, None).ok_or_else(|| anyhow::anyhow!("Parse failed"))?;
        
        let mut f = FileFeatures::default();
        let mut raw_io_count = 0;
        let mut scope_stack = vec![mod_path];
        
        let mut u_ops = HashSet::new();
        let mut u_params = HashSet::new();

        self.traverse_node(tree.root_node(), &code, &mut f, &mut raw_io_count, &extension, &mut scope_stack, &mut u_ops, &mut u_params);

        f.io_density_score = (raw_io_count as f32 / 5.0).min(1.0);
        f.control_flow_score = (f.control_flow_score / 15.0).min(1.0);
        f.data_density_score = (f.data_density_score / 3.0).min(1.0);

        // Calculate Halstead
        f.halstead.n1 = u_ops.len();
        f.halstead.n2 = u_params.len();
        let vocab = (f.halstead.n1 + f.halstead.n2) as f32;
        let len = (f.halstead.n1_total + f.halstead.n2_total) as f32;
        if vocab > 0.0 {
            f.halstead.volume = len * vocab.log2();
            let diff = if f.halstead.n2 > 0 { (f.halstead.n1 as f32 / 2.0) * (f.halstead.n2_total as f32 / f.halstead.n2 as f32) } else { 1.0 };
            f.halstead.effort = diff * f.halstead.volume;
        }

        Ok(f)
    }

    fn get_extension(&self, path: &str) -> String {
        std::path::Path::new(path).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase()
    }

    fn traverse_node(
        &self, 
        node: Node, 
        code: &str, 
        f: &mut FileFeatures, 
        io_count: &mut i32, 
        ext: &str, 
        scope: &mut Vec<String>,
        u_ops: &mut HashSet<String>,
        u_params: &mut HashSet<String>
    ) {
        let (current_symbol, is_trusted) = self.detect_symbol_enhanced(node, code, ext);

        self.collect_halstead(node, code, f, io_count, u_ops, u_params);

        if let Some(name) = current_symbol {
            let scip_id = format!("ccap . . {}#{}#", scope.join("#"), name);
            let mut gravity = 0.1;
            let range = node.byte_range();
            let body_text = &code[range.clone()];
            gravity += (body_text.lines().count() as f32 / 100.0).min(0.4);
            if body_text.contains("sql") || body_text.contains("api") || body_text.contains("http") || body_text.contains("db") { gravity += 0.3; }

            f.exports.push(ScipSymbol {
                id: scip_id,
                name: name.clone(),
                line: node.start_position().row + 1,
                range: (range.start, range.end),
                end_line: node.end_position().row + 1,
                gravity: gravity.min(1.0),
            });

            if f.top_symbols.len() < 5 && !name.starts_with('_') { f.top_symbols.push(name.clone()); }

            if is_trusted {
                scope.push(name);
                self.traverse_children(node, code, f, io_count, ext, scope, u_ops, u_params);
                scope.pop();
                return;
            }
        }
        
        self.traverse_children(node, code, f, io_count, ext, scope, u_ops, u_params);
    }

    fn traverse_children(
        &self, 
        node: Node, 
        code: &str, 
        f: &mut FileFeatures, 
        io_count: &mut i32, 
        ext: &str, 
        scope: &mut Vec<String>,
        u_ops: &mut HashSet<String>,
        u_params: &mut HashSet<String>
    ) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "identifier" {
                self.traverse_node(child, code, f, io_count, ext, scope, u_ops, u_params);
            }
        }
    }

    fn collect_halstead(&self, node: Node, code: &str, f: &mut FileFeatures, io_count: &mut i32, u_ops: &mut HashSet<String>, u_params: &mut HashSet<String>) {
        let kind = node.kind();
        if kind == "if_statement" || kind == "for_statement" || kind == "while_statement" || kind == "try_statement" {
            f.control_flow_score += 1.0;
        }
        if kind == "call" || kind == "call_expression" {
            let text = &code[node.byte_range()].to_lowercase();
            if text.contains("open") || text.contains("print") || text.contains("fetch") || 
               text.contains("socket") || text.contains("http") || text.contains("io") {
                *io_count += 1;
            }
        }

        let is_op = kind.contains("operator") || kind.contains("keyword") || ["+", "-", "*", "/", "=", "==", "!=", "(", ")", "[", "]", "{", "}", ";"].contains(&kind);
        let is_param = kind == "identifier" || kind.contains("literal") || kind.contains("string") || kind == "number";

        if is_op {
            f.halstead.n1_total += 1;
            u_ops.insert(kind.to_string());
        } else if is_param {
            f.halstead.n2_total += 1;
            u_params.insert(code[node.byte_range()].to_string());
        }
    }

    fn detect_symbol_enhanced(&self, node: Node, code: &str, ext: &str) -> (Option<String>, bool) {
        let kind = node.kind();
        if kind == "class_definition" || kind == "function_definition" || kind == "method_definition" ||
           kind == "struct_item" || kind == "enum_item" || kind == "type_declaration" {
            return (self.extract_name_robust(node, code), true);
        }
        if ext == "py" && kind == "ERROR" {
            let range = node.byte_range();
            if range.end > range.start {
                let text = &code[range];
                if let Some(cap) = self.py_def_regex.captures(text) {
                    return (Some(cap.get(1).unwrap().as_str().to_string()), true);
                }
            }
        }
        (None, false)
    }

    fn extract_name_robust(&self, node: Node, code: &str) -> Option<String> {
        if let Some(name_node) = node.child_by_field_name("name") { return Some(code[name_node.byte_range()].to_string()); }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) { if child.kind() == "identifier" { return Some(code[child.byte_range()].to_string()); } }
        None
    }
}
