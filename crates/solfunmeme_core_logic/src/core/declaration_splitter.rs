use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn::{spanned::Spanned, File, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub declaration_type: DeclarationType,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeclarationType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Use,
    Const,
    Static,
    Type,
    Macro,
}

pub struct DeclarationSplitter {
    pub declarations: Vec<Declaration>,
}

impl DeclarationSplitter {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
        }
    }

    pub fn split_file(
        &mut self,
        content: &str,
        file_path: Option<String>,
    ) -> Result<(), syn::Error> {
        let syntax_tree = syn::parse_file(content)?;
        self.extract_declarations(&syntax_tree, content, file_path);
        Ok(())
    }

    fn extract_declarations(&mut self, file: &File, content: &str, file_path: Option<String>) {
        let lines: Vec<&str> = content.lines().collect();

        for item in &file.items {
            let declaration = self.item_to_declaration(item, &lines, file_path.clone());
            if let Some(decl) = declaration {
                self.declarations.push(decl);
            }
        }
    }

    fn item_to_declaration(
        &self,
        item: &Item,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Option<Declaration> {
        match item {
            Item::Fn(item_fn) => Some(self.function_to_declaration(item_fn, lines, file_path)),
            Item::Struct(item_struct) => {
                Some(self.struct_to_declaration(item_struct, lines, file_path))
            }
            Item::Enum(item_enum) => Some(self.enum_to_declaration(item_enum, lines, file_path)),
            Item::Trait(item_trait) => {
                Some(self.trait_to_declaration(item_trait, lines, file_path))
            }
            Item::Impl(item_impl) => Some(self.impl_to_declaration(item_impl, lines, file_path)),
            _ => None,
        }
    }

    fn function_to_declaration(
        &self,
        item_fn: &ItemFn,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Declaration {
        let name = item_fn.sig.ident.to_string();
        let (start, end) = self.get_span_lines(&item_fn.span());
        let content = self.extract_content(lines, start, end);

        Declaration {
            name,
            declaration_type: DeclarationType::Function,
            content,
            line_start: start,
            line_end: end,
            file_path,
        }
    }

    fn struct_to_declaration(
        &self,
        item_struct: &ItemStruct,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Declaration {
        let name = item_struct.ident.to_string();
        let (start, end) = self.get_span_lines(&item_struct.span());
        let content = self.extract_content(lines, start, end);

        Declaration {
            name,
            declaration_type: DeclarationType::Struct,
            content,
            line_start: start,
            line_end: end,
            file_path,
        }
    }

    fn enum_to_declaration(
        &self,
        item_enum: &ItemEnum,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Declaration {
        let name = item_enum.ident.to_string();
        let (start, end) = self.get_span_lines(&item_enum.span());
        let content = self.extract_content(lines, start, end);

        Declaration {
            name,
            declaration_type: DeclarationType::Enum,
            content,
            line_start: start,
            line_end: end,
            file_path,
        }
    }

    fn trait_to_declaration(
        &self,
        item_trait: &ItemTrait,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Declaration {
        let name = item_trait.ident.to_string();
        let (start, end) = self.get_span_lines(&item_trait.span());
        let content = self.extract_content(lines, start, end);

        Declaration {
            name,
            declaration_type: DeclarationType::Trait,
            content,
            line_start: start,
            line_end: end,
            file_path,
        }
    }

    fn impl_to_declaration(
        &self,
        item_impl: &ItemImpl,
        lines: &[&str],
        file_path: Option<String>,
    ) -> Declaration {
        let name = format!("impl_{}", item_impl.self_ty.span().start().line);
        let (start, end) = self.get_span_lines(&item_impl.span());
        let content = self.extract_content(lines, start, end);

        Declaration {
            name,
            declaration_type: DeclarationType::Impl,
            content,
            line_start: start,
            line_end: end,
            file_path,
        }
    }

    fn get_span_lines(&self, span: &syn::__private::Span) -> (usize, usize) {
        (span.start().line, span.end().line)
    }

    fn extract_content(&self, lines: &[&str], start: usize, end: usize) -> String {
        if start == 0 || start > lines.len() || end > lines.len() {
            return String::new();
        }

        lines[(start - 1)..end].join("\n")
    }

    pub fn get_declarations_by_type(&self, decl_type: DeclarationType) -> Vec<&Declaration> {
        self.declarations
            .iter()
            .filter(|d| d.declaration_type == decl_type)
            .collect()
    }

    pub fn save_declarations_to_files(
        &self,
        base_path: &str,
    ) -> Result<HashMap<String, String>, std::io::Error> {
        let mut file_map = HashMap::new();

        for declaration in &self.declarations {
            let filename = format!(
                "{}_{}.rs",
                declaration.declaration_type.to_string().to_lowercase(),
                declaration.name.replace("::", "_")
            );
            let full_path = format!("{}/{}", base_path, filename);
            file_map.insert(full_path, declaration.content.clone());
        }

        Ok(file_map)
    }
}

impl ToString for DeclarationType {
    fn to_string(&self) -> String {
        match self {
            DeclarationType::Function => "function".to_string(),
            DeclarationType::Struct => "struct".to_string(),
            DeclarationType::Enum => "enum".to_string(),
            DeclarationType::Trait => "trait".to_string(),
            DeclarationType::Impl => "impl".to_string(),
            DeclarationType::Module => "module".to_string(),
            DeclarationType::Use => "use".to_string(),
            DeclarationType::Const => "const".to_string(),
            DeclarationType::Static => "static".to_string(),
            DeclarationType::Type => "type".to_string(),
            DeclarationType::Macro => "macro".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CODE: &str = r#"
fn main() {
    println!("Hello, world!");
}

struct Point {
    x: f64,
    y: f64,
}

enum Color {
    Red,
    Green,
    Blue,
}
"#;

    #[test]
    fn test_split_file() {
        let mut splitter = DeclarationSplitter::new();
        let result = splitter.split_file(TEST_CODE, Some("test.rs".to_string()));
        assert!(result.is_ok());
        assert_eq!(splitter.declarations.len(), 3);
    }

    #[test]
    fn test_function_extraction() {
        let mut splitter = DeclarationSplitter::new();
        splitter.split_file(TEST_CODE, None).unwrap();

        let functions = splitter.get_declarations_by_type(DeclarationType::Function);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "main");
    }

    #[test]
    fn test_struct_extraction() {
        let mut splitter = DeclarationSplitter::new();
        splitter.split_file(TEST_CODE, None).unwrap();

        let structs = splitter.get_declarations_by_type(DeclarationType::Struct);
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "Point");
    }

    #[test]
    fn test_enum_extraction() {
        let mut splitter = DeclarationSplitter::new();
        splitter.split_file(TEST_CODE, None).unwrap();

        let enums = splitter.get_declarations_by_type(DeclarationType::Enum);
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].name, "Color");
    }

    #[test]
    fn test_save_declarations_to_files() {
        let mut splitter = DeclarationSplitter::new();
        splitter.split_file(TEST_CODE, None).unwrap();

        let file_map = splitter.save_declarations_to_files("/tmp").unwrap();
        assert_eq!(file_map.len(), 3);
        assert!(file_map.contains_key("/tmp/function_main.rs"));
        assert!(file_map.contains_key("/tmp/struct_Point.rs"));
        assert!(file_map.contains_key("/tmp/enum_Color.rs"));
    }
}
