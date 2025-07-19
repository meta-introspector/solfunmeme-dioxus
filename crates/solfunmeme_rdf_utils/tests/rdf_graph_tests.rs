'''
use solfunmeme_rdf_utils::rdf_graph::{RdfGraph, GraphBuilder};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_rdf_graph_creation_and_serialization() {
    let mut graph = RdfGraph::new();
    graph.namespaces.add_namespace("ex", "http://example.org/").unwrap();
    graph.add_triple("http://example.org/subject", "http://example.org/predicate", "http://example.org/object").unwrap();
    let turtle_string = graph.serialize_to_turtle_string().unwrap();
    assert!(turtle_string.contains("@prefix ex: <http://example.org/> ."));
    assert!(turtle_string.contains("<http://example.org/subject> <http://example.org/predicate> <http://example.org/object> ."));
}

#[test]
fn test_graph_builder() {
    let graph = GraphBuilder::new()
        .with_namespace("ex", "http://example.org/").unwrap()
        .add_triple("http://example.org/subject", "http://example.org/predicate", "http://example.org/object").unwrap()
        .build();
    let turtle_string = graph.serialize_to_turtle_string().unwrap();
    assert!(turtle_string.contains("@prefix ex: <http://example.org/> ."));
    assert!(turtle_string.contains("<http://example.org/subject> <http://example.org/predicate> <http://example.org/object> ."));
}

#[test]
fn test_file_serialization() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.ttl");
    let mut graph = RdfGraph::new();
    graph.namespaces.add_namespace("ex", "http://example.org/").unwrap();
    graph.add_triple("http://example.org/subject", "http://example.org/predicate", "http://example.org/object").unwrap();
    graph.serialize_to_turtle(&file_path).unwrap();
    
    let loaded_graph = RdfGraph::from_file(&file_path).unwrap();
    let turtle_string = loaded_graph.serialize_to_turtle_string().unwrap();
    assert!(turtle_string.contains("<http://example.org/subject> <http://example.org/predicate> <http://example.org/object> ."));
}
''