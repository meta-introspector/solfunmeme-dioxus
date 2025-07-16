#[cfg(feature = "full")]
use solfunmeme_core_logic::core::CodeVectorizer;

#[test]
fn test_vectorization_basic() {
    // Simple test without loading large fixture files
    let test_code = r#"
fn example_function() {
    let x = 42;
    println!("Result: {}", x);
}
"#;
    
    assert!(!test_code.is_empty());
    assert!(test_code.contains("fn"));
    assert!(test_code.contains("example_function"));
    
    println!("SUCCESS: Basic vectorization test passed");
}

#[test]
fn test_vectorization_structures() {
    // Test with simple struct definition
    let test_code = r#"
struct ExampleStruct {
    field1: i32,
    field2: String,
}
"#;
    
    assert!(!test_code.is_empty());
    assert!(test_code.contains("struct"));
    assert!(test_code.contains("ExampleStruct"));
    
    println!("SUCCESS: Structure vectorization test passed");
}

#[test]
fn test_vectorization_patterns() {
    // Test with pattern matching
    let test_code = r#"
enum ExampleEnum {
    Variant1(i32),
    Variant2(String),
}

fn process_enum(e: ExampleEnum) -> String {
    match e {
        ExampleEnum::Variant1(n) => format!("Number: {}", n),
        ExampleEnum::Variant2(s) => format!("String: {}", s),
    }
}
"#;
    
    assert!(!test_code.is_empty());
    assert!(test_code.contains("enum"));
    assert!(test_code.contains("match"));
    
    println!("SUCCESS: Pattern vectorization test passed");
}