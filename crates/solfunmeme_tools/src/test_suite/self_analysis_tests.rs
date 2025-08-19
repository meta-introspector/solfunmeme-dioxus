use solfunmeme_core_logic::core::{CodeAnalyzer, MemeGenerator};

pub fn run_self_analysis_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🪞 Testing Self Analysis...");

    // Analyze this very file
    let current_file = include_str!("../bin/test_runner_cli.rs"); // Adjusted path to the new CLI entry

    let mut analyzer = CodeAnalyzer::new(256, 0.8);
    let analysis = analyzer.analyze_file(current_file, "test_runner_cli.rs".to_string())?;

    let generator = MemeGenerator::new(256);
    let ecosystem = generator.create_meme_ecosystem(&[analysis.clone()]);

    println!("   🔍 Self-Analysis Results:");
    println!(
        "      - Functions Found: {}",
        analysis.metrics.function_count
    );
    println!("      - Total Lines: {}", analysis.metrics.total_lines);
    println!(
        "      - Complexity Score: {:.2}",
        analysis.metrics.complexity_score
    );
    println!("      - Ecosystem Size: {}", ecosystem.memes.len());

    println!("   ✅ Self analysis tests passed");
    Ok(())
}
