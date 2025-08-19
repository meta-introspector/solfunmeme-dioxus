use solfunmeme_core_logic::core::CodeAnalyzer;

pub fn run_code_analysis_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Testing Code Analysis...");

    let mut analyzer = CodeAnalyzer::new(128, 0.8);

    let test_code = include_str!("../../tests/fixtures/code_analysis_test_code.rs.fixture");

    let analysis = analyzer.analyze_file(test_code, "test.rs".to_string())?;

    assert_eq!(analysis.file_path, "test.rs");
    assert!(analysis.declarations.len() > 0);
    assert_eq!(analysis.declarations.len(), analysis.vectors.len());
    assert!(!analysis.json_ast.is_empty());

    println!("   📈 Analysis Results:");
    println!("      - Declarations: {}", analysis.declarations.len());
    println!("      - Functions: {}", analysis.metrics.function_count);
    println!("      - Structs: {}", analysis.metrics.struct_count);
    println!(
        "      - Complexity: {:.2}",
        analysis.metrics.complexity_score
    );

    println!("   ✅ Code analysis tests passed");
    Ok(())
}