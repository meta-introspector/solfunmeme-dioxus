use solfunmeme_tarot::TarotEngine;
use solfunmeme_lean4::OntologyResolver as Lean4OntologyResolver;
use std::path::PathBuf;

#[test]
fn test_tarot_engine_with_lean4_ontology() {
    let lean4_ontology_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("docs")
        .join("lean4_emoji_ontology.ttl");

    let lean4_resolver = Lean4OntologyResolver::load(&lean4_ontology_path)
        .expect("Failed to load Lean4 ontology");

    let emoji_ontology_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("ontologies")
        .join("zos")
        .join("v1.ttl");

    let tarot_engine = TarotEngine::from_ontology(&emoji_ontology_path, Some(lean4_resolver))
        .expect("Failed to create TarotEngine");

    // Test a Lean4 concept that should have an emoji from the lean4_emoji_ontology.ttl
    let simple_expr_uri = "http://example.org/lean4_code#SimpleExpr";
    let simple_expr_entity = tarot_engine.get_entity(simple_expr_uri)
        .expect("SimpleExpr entity not found");
    assert_eq!(simple_expr_entity.emoji, "🏗️🌳");
    assert_eq!(simple_expr_entity.label, "SimpleExpr");

    // Test a non-Lean4 concept that should have an emoji from the v1.ttl
    let task_uri = "http://example.org/emoji#task";
    let task_entity = tarot_engine.get_entity(task_uri)
        .expect("Task entity not found");
    assert_eq!(task_entity.emoji, "✅📋");
    assert_eq!(task_entity.label, "task");

    println!("TarotEngine with Lean4 ontology test passed!");
}
