use super::language_trait::LanguageImpl;
use tree_sitter::{Language as TSLanguage, Node};

/// Implementation of LanguageImpl for Go
pub struct GoLanguage;

impl Default for GoLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl GoLanguage {
    pub fn new() -> Self {
        GoLanguage
    }
}

impl LanguageImpl for GoLanguage {
    fn get_tree_sitter_language(&self) -> TSLanguage {
        tree_sitter_go::LANGUAGE.into()
    }

    fn get_extension(&self) -> &'static str {
        "go"
    }

    fn is_acceptable_parent(&self, node: &Node) -> bool {
        matches!(
            node.kind(),
            "function_declaration" |
            "method_declaration" |
            "type_declaration" |
            "struct_type" |
            "interface_type" |
            "const_declaration" |
            "var_declaration" |
            "const_spec" |
            "var_spec" |
            "short_var_declaration" |
            "type_spec" // Added for type definitions
        )
    }

    fn is_test_node(&self, node: &Node, source: &[u8]) -> bool {
        let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "1";
        let node_type = node.kind();

        // Go: Check function_declaration nodes with names starting with Test
        if node_type == "function_declaration" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    let name = child.utf8_text(source).unwrap_or("");
                    if name.starts_with("Test") {
                        if debug_mode {
                            println!("DEBUG: Test node detected (Go): Test function");
                        }
                        return true;
                    }
                }
            }
        }

        false
    }

    fn find_parent_function<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        let debug_mode = std::env::var("DEBUG").unwrap_or_default() == "1";

        if debug_mode {
            println!(
                "DEBUG: Finding parent function for {node_kind}",
                node_kind = node.kind()
            );
        }

        let mut current = node;

        while let Some(parent) = current.parent() {
            if parent.kind() == "function_declaration" || parent.kind() == "method_declaration" {
                if debug_mode {
                    println!(
                        "DEBUG: Found parent function: {parent_kind}",
                        parent_kind = parent.kind()
                    );
                }
                return Some(parent);
            }
            current = parent;
        }

        if debug_mode {
            println!(
                "DEBUG: No parent function found for {node_kind}",
                node_kind = node.kind()
            );
        }

        None
    }

    fn get_symbol_signature(&self, node: &Node, source: &[u8]) -> Option<String> {
        match node.kind() {
            "function_declaration" => {
                // Extract function signature without body
                // Find block node and extract everything before it
                if let Some(block) = node.child_by_field_name("body") {
                    let sig_end = block.start_byte();
                    let sig = &source[node.start_byte()..sig_end];
                    let sig_str = String::from_utf8_lossy(sig).trim().to_string();
                    // Remove trailing { if present
                    Some(sig_str.trim_end_matches('{').trim().to_string())
                } else {
                    // For function declarations without body
                    let sig = &source[node.start_byte()..node.end_byte()];
                    Some(String::from_utf8_lossy(sig).trim().to_string())
                }
            }
            "type_declaration" => {
                // Extract type signature
                // Go type_declaration has a type_spec child containing name
                // Try to find name by traversing children
                let mut cursor = node.walk();
                let mut found_spec = None;

                for child in node.children(&mut cursor) {
                    if child.kind() == "type_spec" {
                        found_spec = Some(child);
                        break;
                    }
                }

                if let Some(type_spec) = found_spec {
                    if let Some(name) = type_spec.child_by_field_name("name") {
                        let mut sig = String::new();
                        sig.push_str("type ");
                        let name_text = &source[name.start_byte()..name.end_byte()];
                        sig.push_str(&String::from_utf8_lossy(name_text));

                        // Add type parameters if present (in type_spec)
                        if let Some(params) = type_spec.child_by_field_name("type_parameters") {
                            let params_text = &source[params.start_byte()..params.end_byte()];
                            sig.push_str(&String::from_utf8_lossy(params_text));
                        }

                        // Add type if present (in type_spec)
                        if let Some(type_node) = type_spec.child_by_field_name("type") {
                            sig.push_str(" = ");
                            let type_text = &source[type_node.start_byte()..type_node.end_byte()];
                            sig.push_str(&String::from_utf8_lossy(type_text));
                        }

                        Some(sig)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "const_declaration" => {
                // Extract const signature
                // Go const_declaration has a const_spec child containing name
                if let Some(const_spec) = node.child_by_field_name("const_spec") {
                    if let Some(name) = const_spec.child_by_field_name("name") {
                        let mut sig = String::new();
                        sig.push_str("const ");
                        let name_text = &source[name.start_byte()..name.end_byte()];
                        sig.push_str(&String::from_utf8_lossy(name_text));

                        // Add type if present (in const_spec)
                        if let Some(type_node) = const_spec.child_by_field_name("type") {
                            sig.push_str(" ");
                            let type_text = &source[type_node.start_byte()..type_node.end_byte()];
                            sig.push_str(&String::from_utf8_lossy(type_text));
                        }

                        // Add value if present (in const_spec)
                        if let Some(value) = const_spec.child_by_field_name("value") {
                            sig.push_str(" = ");
                            let value_text = &source[value.start_byte()..value.end_byte()];
                            sig.push_str(&String::from_utf8_lossy(value_text));
                        }

                        Some(sig)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
