//! Turtle (RDF) serialization and parsing using Sophia
//! 
//! This module provides functionality for working with Turtle format RDF data,
//! including parsing, serialization, and manipulation.

use sophia::api::{graph::Graph, triple::Triple, term::Term};
use sophia::inmem::graph::FastGraph;
use sophia::turtle::{parser::turtle, serializer::turtle::TurtleSerializer};
use std::io::{BufReader, BufWriter, Write};
use std::fs::File;
use std::path::Path;

use crate::{SemWebResult, SemWebError};

/// Parse Turtle data and add to graph
pub fn parse_and_add(graph: &mut FastGraph, turtle_data: &str) -> SemWebResult<()> {
    turtle::parse_str(turtle_data)
        .in_graph(graph)
        .map_err(|e| SemWebError::RdfParse(e.to_string()))?;
    
    Ok(())
}

/// Parse Turtle from file and add to graph
pub fn parse_file_and_add(graph: &mut FastGraph, file_path: &Path) -> SemWebResult<()> {
    let file = File::open(file_path)
        .map_err(|e| SemWebError::Io(e))?;
    let reader = BufReader::new(file);
    
    turtle::parse_bufread(reader)
        .in_graph(graph)
        .map_err(|e| SemWebError::RdfParse(e.to_string()))?;
    
    Ok(())
}

/// Serialize graph to Turtle format
pub fn serialize_graph(graph: &FastGraph) -> SemWebResult<String> {
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);
    
    TurtleSerializer::new(&mut writer)
        .serialize_graph(graph)
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    String::from_utf8(buffer)
        .map_err(|e| SemWebError::Serialization(format!("UTF-8 error: {}", e)))
}

/// Serialize graph to Turtle file
pub fn serialize_to_file(graph: &FastGraph, file_path: &Path) -> SemWebResult<()> {
    let file = File::create(file_path)
        .map_err(|e| SemWebError::Io(e))?;
    let mut writer = BufWriter::new(file);
    
    TurtleSerializer::new(&mut writer)
        .serialize_graph(graph)
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    Ok(())
}

/// Create Turtle with custom prefixes
pub fn serialize_with_prefixes(
    graph: &FastGraph,
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
    TurtleSerializer::new(&mut writer)
        .serialize_graph(graph)
        .map_err(|e| SemWebError::Serialization(e.to_string()))?;
    
    String::from_utf8(buffer)
        .map_err(|e| SemWebError::Serialization(format!("UTF-8 error: {}", e)))
}

/// Convert graph to Turtle with emoji annotations
pub fn serialize_with_emoji_annotations(
    graph: &FastGraph,
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
    for triple in graph.triples() {
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
    let mut triples = Vec::new();
    
    for triple in turtle::parse_str(turtle_data) {
        let triple = triple.map_err(|e| SemWebError::RdfParse(e.to_string()))?;
        triples.push(triple);
    }
    
    Ok(triples)
}

/// Create Turtle from triples
pub fn triples_to_turtle(triples: &[Triple]) -> SemWebResult<String> {
    let mut graph = FastGraph::new();
    
    for triple in triples {
        graph.insert(triple.s(), triple.p(), triple.o())
            .map_err(|e| SemWebError::Graph(e.to_string()))?;
    }
    
    serialize_graph(&graph)
}

/// Merge multiple Turtle strings into one
pub fn merge_turtle_strings(turtle_strings: &[String]) -> SemWebResult<String> {
    let mut merged_graph = FastGraph::new();
    
    for turtle_string in turtle_strings {
        parse_and_add(&mut merged_graph, turtle_string)?;
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
    match turtle::parse_str(turtle_data).collect::<Result<Vec<_>, _>>() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Count triples in Turtle data
pub fn count_triples(turtle_data: &str) -> SemWebResult<usize> {
    let count = turtle::parse_str(turtle_data).count();
    Ok(count)
}

/// Extract unique subjects from Turtle data
pub fn extract_subjects(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut subjects = std::collections::HashSet::new();
    
    for triple in turtle::parse_str(turtle_data) {
        let triple = triple.map_err(|e| SemWebError::RdfParse(e.to_string()))?;
        subjects.insert(triple.s().to_string());
    }
    
    Ok(subjects)
}

/// Extract unique predicates from Turtle data
pub fn extract_predicates(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut predicates = std::collections::HashSet::new();
    
    for triple in turtle::parse_str(turtle_data) {
        let triple = triple.map_err(|e| SemWebError::RdfParse(e.to_string()))?;
        predicates.insert(triple.p().to_string());
    }
    
    Ok(predicates)
}

/// Extract unique objects from Turtle data
pub fn extract_objects(turtle_data: &str) -> SemWebResult<std::collections::HashSet<String>> {
    let mut objects = std::collections::HashSet::new();
    
    for triple in turtle::parse_str(turtle_data) {
        let triple = triple.map_err(|e| SemWebError::RdfParse(e.to_string()))?;
        objects.insert(triple.o().to_string());
    }
    
    Ok(objects)
} 