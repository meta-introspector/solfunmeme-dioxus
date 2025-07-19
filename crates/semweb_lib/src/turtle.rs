//! Turtle (RDF) serialization and parsing using Sophia
//! 
//! This module provides functionality for working with Turtle format RDF data,
//! including parsing, serialization, and manipulation.

use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::sophia_api::triple::Triple;
use solfunmeme_rdf_utils::sophia_api::term::Term;
use std::io::{BufReader, BufWriter, Write};
use std::fs::File;
use std::path::Path;

use crate::{SemWebResult, SemWebError};

/// Parse Turtle data and add to graph
pub fn parse_and_add(graph: &mut RdfGraph, turtle_data: &str) -> SemWebResult<()> {
    graph.add_turtle_str(turtle_data)
        .map_err(|e| SemWebError::RdfParse(e.to_string()))
}

/// Parse Turtle from file and add to graph
pub fn parse_file_and_add(graph: &mut RdfGraph, file_path: &Path) -> SemWebResult<()> {
    let new_graph = RdfGraph::from_file(file_path)?;
    // Merge the new graph into the existing one
    for triple in new_graph.graph.triples() {
        if let Ok(triple) = triple {
            graph.add_triple(&triple.s().to_string(), &triple.p().to_string(), &triple.o().to_string())?;
        }
    }
    Ok(())
}

/// Serialize graph to Turtle format
pub fn serialize_graph(graph: &RdfGraph) -> SemWebResult<String> {
    graph.serialize_to_turtle_string()
        .map_err(|e| SemWebError::Serialization(e.to_string()))
}

/// Serialize graph to Turtle file
pub fn serialize_to_file(graph: &RdfGraph, file_path: &Path) -> SemWebResult<()> {
    graph.serialize_to_turtle(file_path)
        .map_err(|e| SemWebError::Serialization(e.to_string()))
}

/// Create Turtle with custom prefixes
pub fn serialize_with_prefixes(
    graph: &RdfGraph,
    prefixes: &[(String, String)]
) -> SemWebResult<String> {
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);
    
    // Write prefixes
    for (prefix, namespace) in prefixes {
        writeln!(writer, "@prefix {}: <{}> .", prefix, namespace)
            .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    }
    writeln!(writer).map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    // Write triples
    writer.write_all(graph.serialize_to_turtle_string()?.as_bytes())
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    String::from_utf8(buffer)
        .map_err(|e| SemWebError::Serialization(format!("UTF-8 error: {}", e)))
}

/// Convert graph to Turtle with emoji annotations
pub fn serialize_with_emoji_annotations(
    graph: &RdfGraph,
    emoji_map: &std::collections::HashMap<String, String>
) -> SemWebResult<String> {
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);
    
    // Write standard prefixes
    writeln!(writer, "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .")
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    writeln!(writer, "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .")
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    writeln!(writer, "@prefix em: <http://example.org/emoji#> .")
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    writeln!(writer).map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    // Write triples with emoji annotations
    for triple in graph.graph.triples() {
        if let Ok(triple) = triple {
            let subject = triple.s().to_string();
            let predicate = triple.p().to_string();
            let object = triple.o().to_string();
            
            // Write the main triple
            writeln!(writer, "{} {} {} .", subject, predicate, object)
                .map_err(|e| SemWebError::Serialization(e.to_string()))?;
            
            // Add emoji annotation if available
            if let Some(emoji) = emoji_map.get(&subject) {
                writeln!(writer, "{} em:emoji \"{}\" .", subject, emoji)
                    .map_err(|e| SemWebError::Serialization(e.to_string()))?;
            }
        }
    }
    
    String::from_utf8(buffer)
        .map_err(|e| SemWebError::Serialization(format!("UTF-8 error: {}", e)))
}

/// Parse Turtle and return as vector of triples
pub fn parse_to_triples(turtle_data: &str) -> SemWebResult<Vec<Triple>> {
    let mut graph = RdfGraph::new();
    graph.add_turtle_str(turtle_data)?;
    let mut triples = Vec::new();
    for triple in graph.graph.triples() {
        triples.push(triple?);
    }
    Ok(triples)
}

/// Create Turtle from triples
pub fn triples_to_turtle(triples: &[Triple]) -> SemWebResult<String> {
    let mut graph = RdfGraph::new();
    
    for triple in triples {
        graph.add_triple(&triple.s().to_string(), &triple.p().to_string(), &triple.o().to_string())?;
    }
    
    serialize_graph(&graph)
}

/// Merge multiple Turtle strings into one
pub fn merge_turtle_strings(turtle_strings: &[String]) -> SemWebResult<String> {
    let mut merged_graph = RdfGraph::new();
    
    for turtle_string in turtle_strings {
        merged_graph.add_turtle_str(turtle_string)?;
    }
    
    serialize_graph(&merged_graph)
}

/// Extract prefixes from Turtle data
pub fn extract_prefixes(turtle_data: &str) -> SemWebResult<std::collections::HashMap<String, String>> {
    let mut prefixes = std::collections::HashMap::new();
    
    for line in turtle_data.lines() {
        let line = line.trim();
        if line.starts_with("@prefix") {
            if let Some((prefix, namespace)) = parse_prefix_line(line) {
                prefixes.insert(prefix, namespace);
            }
        }
    }
    
    Ok(prefixes)
}

/// Parse a prefix line from Turtle
fn parse_prefix_line(line: &str) -> Option<(String, String)> {
    // Simple prefix parsing - in a real implementation, this would be more robust
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 && parts[0] == "@prefix" {
        let prefix = parts[1].trim_end_matches(':').to_string();
        let namespace = parts[2].trim_matches('<').trim_matches('>').to_string();
        Some((prefix, namespace))
    } else {
        None
    }
}

/// Validate Turtle syntax
pub fn validate_turtle(turtle_data: &str) -> SemWebResult<bool> {
    let mut graph = RdfGraph::new();
    match graph.add_turtle_str(turtle_data) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Count triples in Turtle data
pub fn count_triples(turtle_data: &str) -> SemWebResult<usize> {
    let mut graph = RdfGraph::new();
    graph.add_turtle_str(turtle_data)?;
    Ok(graph.graph.triples().count())
}

/// Extract unique subjects from Turtle data
pub fn extract_subjects(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut subjects = std::collections::HashSet::new();
    let mut graph = RdfGraph::new();
    graph.add_turtle_str(turtle_data)?;
    
    for triple in graph.graph.triples() {
        let triple = triple?;
        subjects.insert(triple.s().to_string());
    }
    
    Ok(subjects)
}

/// Extract unique predicates from Turtle data
pub fn extract_predicates(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut predicates = std::collections::HashSet::new();
    let mut graph = RdfGraph::new();
    graph.add_turtle_str(turtle_data)?;
    
    for triple in graph.graph.triples() {
        let triple = triple?;
        predicates.insert(triple.p().to_string());
    }
    
    Ok(predicates)
}

/// Extract unique objects from Turtle data
pub fn extract_objects(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut objects = std::collections::HashSet::new();
    let mut graph = RdfGraph::new();
    graph.add_turtle_str(turtle_data)?;
    
    for triple in graph.graph.triples() {
        let triple = triple?;
        objects.insert(triple.o().to_string());
    }
    
    Ok(objects)
}