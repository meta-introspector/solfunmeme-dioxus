use solfunmeme_core_logic::core::{DuplicateDetector, Declaration, DeclarationType};

pub fn run_duplicate_detection_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing Duplicate Detection...");

    let detector = DuplicateDetector::new(64, 0.8);

    // Create test declarations with duplicates
    let declarations = vec![
        Declaration {
            name: "func1".to_string(),
            declaration_type: DeclarationType::Function,
            content: include_str!("../../tests/fixtures/duplicate_test_code_1.rs.fixture").to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file1.rs".to_string()),
        },
        Declaration {
            name: "func2".to_string(),
            declaration_type: DeclarationType::Function,
            content: include_str!("../../tests/fixtures/duplicate_test_code_2.rs.fixture").to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file2.rs".to_string()),
        },
        Declaration {
            name: "unique".to_string(),
            declaration_type: DeclarationType::Function,
            content: include_str!("../../tests/fixtures/unique_test_code.rs.fixture").to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("file3.rs".to_string()),
        },
    ];

    let report = detector.detect_duplicates(&declarations);
    let stats = detector.calculate_deduplication_savings(&report);

    println!("   📊 Duplicate Report:");
    println!("      - Groups: {}", report.groups.len());
    println!("      - Total Duplicates: {}", report.total_duplicates);
    println!("      - Savings: {:.1}%", stats.savings_percentage);

    println!("   ✅ Duplicate detection tests passed");
    Ok(())
}