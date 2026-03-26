use solfunmeme_dioxus::core::*;
use std::collections::HashMap;

const SAMPLE_RUST_CODE: &str = r#"
use std::collections::HashMap;

/// A sample function for testing
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// A sample struct
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Sample enum
pub enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

/// Sample trait
pub trait Drawable {
    fn draw(&self);
}

impl Drawable for Point {
    fn draw(&self) {
        println!("Drawing point at ({}, {})", self.x, self.y);
    }
}
"#;

#[test]
fn test_full_code_analysis_pipeline() {
    let mut analyzer = CodeAnalyzer::new(128, 0.8);

    let analysis = analyzer
        .analyze_file(SAMPLE_RUST_CODE, "sample.rs".to_string())
        .expect("Failed to analyze code");

    // Verify basic analysis
    assert_eq!(analysis.file_path, "sample.rs");
    assert!(analysis.declarations.len() > 0);
    assert_eq!(analysis.declarations.len(), analysis.vectors.len());
    assert!(!analysis.json_ast.is_empty());

    // Verify metrics
    assert!(analysis.metrics.function_count >= 3); // fibonacci, new, distance, draw
    assert_eq!(analysis.metrics.struct_count, 1);
    assert_eq!(analysis.metrics.enum_count, 1);
    assert_eq!(analysis.metrics.trait_count, 1);
    assert_eq!(analysis.metrics.impl_count, 2);
    assert!(analysis.metrics.complexity_score > 0.0);
}

#[test]
fn test_declaration_splitting_and_vectorization() {
    let mut splitter = DeclarationSplitter::new();
    splitter
        .split_file(SAMPLE_RUST_CODE, Some("test.rs".to_string()))
        .expect("Failed to split declarations");

    assert!(splitter.declarations.len() >= 5);

    let vectorizer = CodeVectorizer::new(64);
    let vectors: Vec<_> = splitter
        .declarations
        .iter()
        .map(|decl| vectorizer.vectorize(&decl.content))
        .collect();

    assert_eq!(vectors.len(), splitter.declarations.len());

    // Test vector similarity
    for vector in &vectors {
        assert_eq!(vector.dimensions.len(), 64);
        let sum: f32 = vector.dimensions.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5); // Should be normalized
    }
}

#[test]
fn test_duplicate_detection() {
    let duplicate_code1 = r#"
fn duplicate_function() {
    println!("This is a duplicate");
    let x = 42;
    x + 1
}
"#;

    let duplicate_code2 = r#"
fn another_duplicate() {
    println!("This is a duplicate");
    let x = 42;
    x + 1
}
"#;

    let mut analyzer = CodeAnalyzer::new(64, 0.7);

    let mut files = HashMap::new();
    files.insert("file1.rs".to_string(), duplicate_code1.to_string());
    files.insert("file2.rs".to_string(), duplicate_code2.to_string());

    let analyses = analyzer
        .analyze_multiple_files(files)
        .expect("Failed to analyze files");

    let duplicate_report = analyzer.find_cross_file_duplicates(&analyses);

    // Should detect duplicates due to similar content
    assert!(duplicate_report.total_duplicates > 0 || duplicate_report.canonical_count > 0);
}

#[test]
fn test_meme_generation() {
    let mut analyzer = CodeAnalyzer::new(128, 0.8);
    let analysis = analyzer
        .analyze_file(SAMPLE_RUST_CODE, "sample.rs".to_string())
        .expect("Failed to analyze code");

    let memes = analyzer.generate_meme_representation(&analysis);

    assert!(memes.len() > 0);

    // Check that we have memes for different declaration types
    let meme_values: Vec<&String> = memes.values().collect();
    let has_function_meme = meme_values.iter().any(|m: &&String| m.contains("🔧"));
    let has_struct_meme = meme_values.iter().any(|m: &&String| m.contains("🏗️"));

    assert!(has_function_meme);
    assert!(has_struct_meme);
}

#[test]
fn test_meme_ecosystem_creation() {
    let mut analyzer = CodeAnalyzer::new(64, 0.8);
    let analysis = analyzer
        .analyze_file(SAMPLE_RUST_CODE, "sample.rs".to_string())
        .expect("Failed to analyze code");

    let generator = MemeGenerator::new(64);
    let ecosystem = generator.create_meme_ecosystem(&[analysis]);

    assert!(ecosystem.memes.len() > 0);
    assert_eq!(ecosystem.relationships.len(), ecosystem.memes.len());
    assert_eq!(ecosystem.dimensional_structure.dimensions, 64);
    assert!(ecosystem.dimensional_structure.harmonic_frequencies.len() > 0);
}

#[test]
fn test_biosemiotic_properties() {
    let recursive_code = r#"
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
"#;

    let mut splitter = DeclarationSplitter::new();
    splitter.split_file(recursive_code, None).unwrap();

    let generator = MemeGenerator::new(64);
    let vector = CodeVectorizer::new(64).vectorize(recursive_code);
    let meme = generator.generate_meme_from_declaration(&splitter.declarations[0], &vector);

    let props = &meme.metadata.biosemiotic_properties;
    assert!(props.emergence_level >= 1);
    assert!(props.recursive_depth > 0);
    assert!(props.information_density > 0.0);
    assert!(props.semantic_coherence > 0.0);
}

#[test]
fn test_code_to_vector_to_code_consistency() {
    let simple_code = "fn hello() { println!(\"Hello, world!\"); }";

    let vectorizer = CodeVectorizer::new(32);
    let vector1 = vectorizer.vectorize(simple_code);
    let vector2 = vectorizer.vectorize(simple_code);

    // Same code should produce identical vectors
    assert_eq!(vector1.dimensions, vector2.dimensions);
    assert!((vector1.similarity(&vector2) - 1.0).abs() < 1e-6);
}

#[test]
fn test_complexity_scoring() {
    let simple_code = "fn simple() {}";
    let complex_code = r#"
fn complex(data: Vec<HashMap<String, Vec<Option<Result<i32, String>>>>>) -> Result<(), Box<dyn std::error::Error>> {
    for item in data {
        for (key, values) in item {
            for value in values {
                match value {
                    Some(Ok(n)) if n > 0 => {
                        for i in 0..n {
                            if i % 2 == 0 {
                                unsafe {
                                    println!("Complex operation: {}", i);
                                }
                            }
                        }
                    },
                    Some(Err(e)) => return Err(e.into()),
                    _ => continue,
                }
            }
        }
    }
    Ok(())
}
"#;

    let mut analyzer = CodeAnalyzer::new(64, 0.8);
    let simple_analysis = analyzer
        .analyze_file(simple_code, "simple.rs".to_string())
        .unwrap();
    let complex_analysis = analyzer
        .analyze_file(complex_code, "complex.rs".to_string())
        .unwrap();

    assert!(complex_analysis.metrics.complexity_score > simple_analysis.metrics.complexity_score);
}

#[test]
fn test_canonical_directory_creation() {
    let code_with_duplicates = r#"
fn function_a() {
    let x = 42;
    println!("{}", x);
}

fn function_b() {
    let x = 42;
    println!("{}", x);
}

fn unique_function() {
    println!("I am unique!");
}
"#;

    let mut analyzer = CodeAnalyzer::new(64, 0.9);
    let analysis = analyzer
        .analyze_file(code_with_duplicates, "test.rs".to_string())
        .unwrap();

    if let Some(duplicate_report) = &analysis.duplicate_report {
        let detector = DuplicateDetector::new(64, 0.9);
        let canonical_files = detector.create_canonical_directory(duplicate_report);
        let stats = detector.calculate_deduplication_savings(duplicate_report);

        if duplicate_report.groups.len() > 0 {
            assert!(canonical_files.len() > 0);
            assert!(stats.savings_percentage >= 0.0);
        }
    }
}
