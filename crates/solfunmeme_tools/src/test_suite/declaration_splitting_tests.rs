use solfunmeme_core_logic::core::{DeclarationSplitter, DeclarationType};

pub fn run_declaration_splitting_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔪 Testing Declaration Splitting...");

    let test_code = include_str!("../../tests/fixtures/declaration_splitting_test_code.rs.fixture");

    let mut splitter = DeclarationSplitter::new();
    splitter.split_file(test_code, Some("test.rs".to_string()))?;

    assert!(
        splitter.declarations.len() >= 5,
        "Should find at least 5 declarations"
    );

    let functions = splitter.get_declarations_by_type(DeclarationType::Function);
    let structs = splitter.get_declarations_by_type(DeclarationType::Struct);
    let enums = splitter.get_declarations_by_type(DeclarationType::Enum);
    let traits = splitter.get_declarations_by_type(DeclarationType::Trait);
    let impls = splitter.get_declarations_by_type(DeclarationType::Impl);

    assert!(functions.len() >= 2, "Should find at least 2 functions");
    assert_eq!(structs.len(), 1, "Should find 1 struct");
    assert_eq!(enums.len(), 1, "Should find 1 enum");
    assert_eq!(traits.len(), 1, "Should find 1 trait");
    assert_eq!(impls.len(), 1, "Should find 1 impl");

    println!("   ✅ Declaration splitting tests passed");
    Ok(())
}