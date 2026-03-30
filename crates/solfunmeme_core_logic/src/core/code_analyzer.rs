use crate::core::{
    declaration_splitter::{Declaration, DeclarationSplitter, DeclarationType},
    duplicate_detector::{DuplicateDetector, DuplicateReport},
    vectorization::{CodeVector, CodeVectorizer},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use syn_serde::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub file_path: String,
    pub declarations: Vec<Declaration>,
    pub vectors: Vec<CodeVector>,
    pub json_ast: String,
    pub duplicate_report: Option<DuplicateReport>,
    pub metrics: CodeMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub total_lines: usize,
    pub function_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub trait_count: usize,
    pub impl_count: usize,
    pub complexity_score: f32,
}

pub struct CodeAnalyzer {
    splitter: DeclarationSplitter,
    vectorizer: CodeVectorizer,
    duplicate_detector: DuplicateDetector,
}

impl CodeAnalyzer {
    pub fn new(vector_dimensions: usize, similarity_threshold: f32) -> Self {
        Self {
            splitter: DeclarationSplitter::new(),
            vectorizer: CodeVectorizer::new(vector_dimensions),
            duplicate_detector: DuplicateDetector::new(vector_dimensions, similarity_threshold),
        }
    }

    pub fn analyze_file(
        &mut self,
        content: &str,
        file_path: String,
    ) -> Result<CodeAnalysis, Box<dyn std::error::Error>> {
        // Parse and split declarations
        self.splitter = DeclarationSplitter::new();
        self.splitter.split_file(content, Some(file_path.clone()))?;

        // Generate vectors for each declaration
        let vectors: Vec<CodeVector> = self
            .splitter
            .declarations
            .iter()
            .map(|decl| self.vectorizer.vectorize(&decl.content))
            .collect();

        // Generate JSON AST
        let json_ast = self.generate_json_ast(content)?;

        // Calculate metrics
        let metrics = self.calculate_metrics(content, &self.splitter.declarations);

        // Detect duplicates
        let duplicate_report = if self.splitter.declarations.len() > 1 {
            Some(
                self.duplicate_detector
                    .detect_duplicates(&self.splitter.declarations),
            )
        } else {
            None
        };

        Ok(CodeAnalysis {
            file_path,
            declarations: self.splitter.declarations.clone(),
            vectors,
            json_ast,
            duplicate_report,
            metrics,
        })
    }

    pub fn analyze_multiple_files(
        &mut self,
        files: HashMap<String, String>,
    ) -> Result<Vec<CodeAnalysis>, Box<dyn std::error::Error>> {
        let mut analyses = Vec::new();

        for (file_path, content) in files {
            match self.analyze_file(&content, file_path.clone()) {
                Ok(analysis) => analyses.push(analysis),
                Err(e) => eprintln!("Failed to analyze {}: {}", file_path, e),
            }
        }

        Ok(analyses)
    }

    pub fn find_cross_file_duplicates(&self, analyses: &[CodeAnalysis]) -> DuplicateReport {
        let all_declarations: Vec<Declaration> = analyses
            .iter()
            .flat_map(|analysis| analysis.declarations.iter().cloned())
            .collect();

        self.duplicate_detector.detect_duplicates(&all_declarations)
    }

    fn generate_json_ast(&self, content: &str) -> Result<String, syn::Error> {
        let syntax_tree = syn::parse_file(content)?;
        Ok(json::to_string_pretty(&syntax_tree))
    }

    fn calculate_metrics(&self, content: &str, declarations: &[Declaration]) -> CodeMetrics {
        let total_lines = content.lines().count();

        let mut function_count = 0;
        let mut struct_count = 0;
        let mut enum_count = 0;
        let mut trait_count = 0;
        let mut impl_count = 0;

        for decl in declarations {
            match decl.declaration_type {
                DeclarationType::Function => function_count += 1,
                DeclarationType::Struct => struct_count += 1,
                DeclarationType::Enum => enum_count += 1,
                DeclarationType::Trait => trait_count += 1,
                DeclarationType::Impl => impl_count += 1,
                _ => {}
            }
        }

        let complexity_score = self.calculate_complexity_score(content);

        CodeMetrics {
            total_lines,
            function_count,
            struct_count,
            enum_count,
            trait_count,
            impl_count,
            complexity_score,
        }
    }

    fn calculate_complexity_score(&self, content: &str) -> f32 {
        let mut score = 0.0;

        // Simple complexity metrics
        score += content.matches("if ").count() as f32 * 1.0;
        score += content.matches("match ").count() as f32 * 2.0;
        score += content.matches("for ").count() as f32 * 1.5;
        score += content.matches("while ").count() as f32 * 1.5;
        score += content.matches("loop ").count() as f32 * 2.0;
        score += content.matches("unsafe ").count() as f32 * 3.0;

        // Normalize by lines of code
        let lines = content.lines().count().max(1) as f32;
        score / lines
    }

    pub fn generate_meme_representation(&self, analysis: &CodeAnalysis) -> HashMap<String, String> {
        let mut memes = HashMap::new();

        for (i, declaration) in analysis.declarations.iter().enumerate() {
            let emoji = self.get_emoji_for_declaration(&declaration.declaration_type);
            let vector_summary = if i < analysis.vectors.len() {
                format!("Vector[{}]", analysis.vectors[i].dimensions.len())
            } else {
                "Vector[?]".to_string()
            };

            let meme = format!(
                "{} {} - {} (Lines: {}-{})",
                emoji,
                declaration.name,
                vector_summary,
                declaration.line_start,
                declaration.line_end
            );

            memes.insert(declaration.name.clone(), meme);
        }

        memes
    }

    fn get_emoji_for_declaration(&self, decl_type: &DeclarationType) -> &'static str {
        match decl_type {
            DeclarationType::Function => "🔧",
            DeclarationType::Struct => "🏗️",
            DeclarationType::Enum => "🎲",
            DeclarationType::Trait => "✨",
            DeclarationType::Impl => "🔌",
            DeclarationType::Module => "📦",
            DeclarationType::Use => "📤",
            DeclarationType::Const => "💎",
            DeclarationType::Static => "🗿",
            DeclarationType::Type => "📜",
            DeclarationType::Macro => "🔮",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CODE: &str = r#"
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    if true {
        for i in 0..10 {
            println!("{}", i);
        }
    }
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

trait Display {
    fn display(&self);
}

impl Display for Point {
    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}
"#;

    #[test]
    fn test_analyze_file() {
        let mut analyzer = CodeAnalyzer::new(100, 0.8);
        let result = analyzer.analyze_file(TEST_CODE, "test.rs".to_string());

        assert!(result.is_ok());
        let analysis = result.unwrap();

        assert_eq!(analysis.file_path, "test.rs");
        assert!(analysis.declarations.len() > 0);
        assert_eq!(analysis.vectors.len(), analysis.declarations.len());
        assert!(!analysis.json_ast.is_empty());
    }

    #[test]
    fn test_code_metrics() {
        let mut analyzer = CodeAnalyzer::new(100, 0.8);
        let analysis = analyzer
            .analyze_file(TEST_CODE, "test.rs".to_string())
            .unwrap();

        assert!(analysis.metrics.total_lines > 0);
        assert_eq!(analysis.metrics.function_count, 2); // main + display
        assert_eq!(analysis.metrics.struct_count, 1);
        assert_eq!(analysis.metrics.enum_count, 1);
        assert_eq!(analysis.metrics.trait_count, 1);
        assert_eq!(analysis.metrics.impl_count, 1);
        assert!(analysis.metrics.complexity_score > 0.0);
    }

    #[test]
    fn test_meme_representation() {
        let mut analyzer = CodeAnalyzer::new(100, 0.8);
        let analysis = analyzer
            .analyze_file(TEST_CODE, "test.rs".to_string())
            .unwrap();

        let memes = analyzer.generate_meme_representation(&analysis);
        assert!(memes.len() > 0);
        assert!(memes.contains_key("main"));
        assert!(memes.get("main").unwrap().contains("🔧"));
    }

    #[test]
    fn test_analyze_multiple_files() {
        let mut analyzer = CodeAnalyzer::new(100, 0.8);
        let mut files = HashMap::new();
        files.insert("test1.rs".to_string(), TEST_CODE.to_string());
        files.insert("test2.rs".to_string(), "fn hello() {}".to_string());

        let analyses = analyzer.analyze_multiple_files(files).unwrap();
        assert_eq!(analyses.len(), 2);
    }

    #[test]
    fn test_cross_file_duplicates() {
        let mut analyzer = CodeAnalyzer::new(100, 0.8);
        let duplicate_code = "fn duplicate() { println!(\"same\"); }";

        let mut files = HashMap::new();
        files.insert("file1.rs".to_string(), duplicate_code.to_string());
        files.insert("file2.rs".to_string(), duplicate_code.to_string());

        let analyses = analyzer.analyze_multiple_files(files).unwrap();
        let cross_file_report = analyzer.find_cross_file_duplicates(&analyses);

        assert!(cross_file_report.groups.len() > 0);
    }
}
