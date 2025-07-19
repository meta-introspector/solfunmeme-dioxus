//! Ontology management and loading
//! 
//! This module provides functionality for working with ontologies,
//! including loading from Turtle files and managing ontology structures.

use async_trait::async_trait;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::sophia_api::triple::Triple;
use solfunmeme_rdf_utils::sophia_api::term::Term;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use serde::{Deserialize, Serialize};

use crate::{SemWebResult, SemWebError, turtle};

/// Ontology representation with enhanced functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    pub uri: String,
    pub prefix: String,
    pub title: String,
    pub description: Option<String>,
    pub version: String,
    #[serde(skip)] // Skip serialization for RdfGraph
    pub graph: RdfGraph,
    pub classes: HashMap<String, Class>,
    pub properties: HashMap<String, Property>,
    pub individuals: HashMap<String, Individual>,
    pub metadata: OntologyMetadata,
}

/// Enhanced class representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub uri: String,
    pub label: String,
    pub comment: Option<String>,
    pub sub_class_of: Vec<String>,
    pub equivalent_to: Vec<String>,
    pub disjoint_with: Vec<String>,
    pub instances: Vec<String>,
}

/// Enhanced property representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub uri: String,
    pub label: String,
    pub comment: Option<String>,
    pub domain: Option<String>,
    pub range: Option<String>,
    pub sub_property_of: Vec<String>,
    pub equivalent_to: Vec<String>,
    pub inverse_of: Option<String>,
    pub is_functional: bool,
    pub is_inverse_functional: bool,
    pub is_transitive: bool,
    pub is_symmetric: bool,
    pub is_asymmetric: bool,
    pub is_reflexive: bool,
    pub is_irreflexive: bool,
}

/// Enhanced individual representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    pub uri: String,
    pub label: String,
    pub comment: Option<String>,
    pub types: Vec<String>,
    pub same_as: Vec<String>,
    pub different_from: Vec<String>,
    pub properties: HashMap<String, Vec<String>>,
}

/// Ontology metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyMetadata {
    pub creator: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub license: Option<String>,
    pub keywords: Vec<String>,
    pub namespace: String,
    pub prefix: String,
}

impl Ontology {
    /// Create a new ontology
    pub fn new(uri: String, prefix: String, title: String) -> Self {
        Self {
            uri: uri.clone(),
            prefix: prefix.clone(),
            title,
            description: None,
            version: "1.0.0".to_string(),
            graph: RdfGraph::new(),
            classes: HashMap::new(),
            properties: HashMap::new(),
            individuals: HashMap::new(),
            metadata: OntologyMetadata {
                creator: None,
                created: None,
                modified: None,
                license: None,
                keywords: Vec::new(),
                namespace: uri,
                prefix,
            },
        }
    }
    
    /// Load ontology from Turtle file
    pub async fn from_turtle_file(path: &str) -> SemWebResult<Self> {
        let content = fs::read_to_string(path).await
            .map_err(|e| SemWebError::Io(e))?;
        
        Self::from_turtle_string(&content)
    }
    
    /// Create ontology from Turtle string
    pub fn from_turtle_string(turtle_data: &str) -> SemWebResult<Self> {
        let mut graph = RdfGraph::new();
        graph.add_turtle_str(turtle_data)?;
        
        let mut ontology = Self::new(
            "http://example.org/ontology".to_string(),
            "ex".to_string(),
            "Generated Ontology".to_string(),
        );
        ontology.graph = graph;
        
        // Extract classes, properties, and individuals from the graph
        ontology.extract_entities()?;
        
        Ok(ontology)
    }
    
    /// Extract entities from the graph
    fn extract_entities(&mut self) -> SemWebResult<()> {
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                let subject = triple.s().to_string();
                let predicate = triple.p().to_string();
                let object = triple.o().to_string();
                
                // Extract classes
                if predicate == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" &&
                   object == "http://www.w3.org/2000/01/rdf-schema#Class" {
                    let class = Class {
                        uri: subject.clone(),
                        label: self.extract_label(&subject).unwrap_or_else(|| subject.clone()),
                        comment: self.extract_comment(&subject),
                        sub_class_of: self.extract_sub_classes(&subject),
                        equivalent_to: Vec::new(),
                        disjoint_with: Vec::new(),
                        instances: Vec::new(),
                    };
                    self.classes.insert(subject, class);
                }
                
                // Extract properties
                if predicate == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" &&
                   object == "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property" {
                    let property = Property {
                        uri: subject.clone(),
                        label: self.extract_label(&subject).unwrap_or_else(|| subject.clone()),
                        comment: self.extract_comment(&subject),
                        domain: self.extract_domain(&subject),
                        range: self.extract_range(&subject),
                        sub_property_of: Vec::new(),
                        equivalent_to: Vec::new(),
                        inverse_of: None,
                        is_functional: false,
                        is_inverse_functional: false,
                        is_transitive: false,
                        is_symmetric: false,
                        is_asymmetric: false,
                        is_reflexive: false,
                        is_irreflexive: false,
                    };
                    self.properties.insert(subject, property);
                }
                
                // Extract individuals
                if predicate == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" &&
                   !object.starts_with("http://www.w3.org/") {
                    let individual = Individual {
                        uri: subject.clone(),
                        label: self.extract_label(&subject).unwrap_or_else(|| subject.clone()),
                        comment: self.extract_comment(&subject),
                        types: vec![object],
                        same_as: Vec::new(),
                        different_from: Vec::new(),
                        properties: HashMap::new(),
                    };
                    self.individuals.insert(subject, individual);
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract label for a URI
    fn extract_label(&self, uri: &str) -> Option<String> {
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                if triple.s().to_string() == uri &&
                   triple.p().to_string() == "http://www.w3.org/2000/01/rdf-schema#label" {
                    return Some(triple.o().to_string().trim_matches('"').to_string());
                }
            }
        }
        None
    }
    
    /// Extract comment for a URI
    fn extract_comment(&self, uri: &str) -> Option<String> {
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                if triple.s().to_string() == uri &&
                   triple.p().to_string() == "http://www.w3.org/2000/01/rdf-schema#comment" {
                    return Some(triple.o().to_string().trim_matches('"').to_string());
                }
            }
        }
        None
    }
    
    /// Extract sub-classes for a URI
    fn extract_sub_classes(&self, uri: &str) -> Vec<String> {
        let mut sub_classes = Vec::new();
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                if triple.s().to_string() == uri &&
                   triple.p().to_string() == "http://www.w3.org/2000/01/rdf-schema#subClassOf" {
                    sub_classes.push(triple.o().to_string());
                }
            }
        }
        sub_classes
    }
    
    /// Extract domain for a property
    fn extract_domain(&self, uri: &str) -> Option<String> {
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                if triple.s().to_string() == uri &&
                   triple.p().to_string() == "http://www.w3.org/2000/01/rdf-schema#domain" {
                    return Some(triple.o().to_string());
                }
            }
        }
        None
    }
    
    /// Extract range for a property
    fn extract_range(&self, uri: &str) -> Option<String> {
        for triple in self.graph.graph.triples() {
            if let Ok(triple) = triple {
                if triple.s().to_string() == uri &&
                   triple.p().to_string() == "http://www.w3.org/2000/01/rdf-schema#range" {
                    return Some(triple.o().to_string());
                }
            }
        }
        None
    }
    
    /// Add a class to the ontology
    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.uri.clone(), class);
    }
    
    /// Add a property to the ontology
    pub fn add_property(&mut self, property: Property) {
        self.properties.insert(property.uri.clone(), property);
    }
    
    /// Add an individual to the ontology
    pub fn add_individual(&mut self, individual: Individual) {
        self.individuals.insert(individual.uri.clone(), individual);
    }
    
    /// Get a class by URI
    pub fn get_class(&self, uri: &str) -> Option<&Class> {
        self.classes.get(uri)
    }
    
    /// Get a property by URI
    pub fn get_property(&self, uri: &str) -> Option<&Property> {
        self.properties.get(uri)
    }
    
    /// Get an individual by URI
    pub fn get_individual(&self, uri: &str) -> Option<&Individual> {
        self.individuals.get(uri)
    }
    
    /// Export ontology to Turtle format
    pub fn to_turtle(&self) -> SemWebResult<String> {
        self.graph.serialize_to_turtle_string()
            .map_err(|e| SemWebError::Serialization(e.to_string()))
    }
    
    /// Save ontology to file
    pub async fn save_to_file(&self, path: &str) -> SemWebResult<()> {
        let turtle_data = self.to_turtle()?;
        fs::write(path, turtle_data).await
            .map_err(|e| SemWebError::Io(e))?;
        Ok(())
    }
    
    /// Merge with another ontology
    pub fn merge(&mut self, other: &Ontology) -> SemWebResult<()> {
        // Merge graphs
        for triple in other.graph.graph.triples() {
            if let Ok(triple) = triple {
                self.graph.add_triple(&triple.s().to_string(), &triple.p().to_string(), &triple.o().to_string())?;
            }
        }
        
        // Merge classes
        for (uri, class) in &other.classes {
            self.classes.insert(uri.clone(), class.clone());
        }
        
        // Merge properties
        for (uri, property) in &other.properties {
            self.properties.insert(uri.clone(), property.clone());
        }
        
        // Merge individuals
        for (uri, individual) in &other.individuals {
            self.individuals.insert(uri.clone(), individual.clone());
        }
        
        Ok(())
    }
    
    /// Get all classes
    pub fn get_all_classes(&self) -> Vec<&Class> {
        self.classes.values().collect()
    }
    
    /// Get all properties
    pub fn get_all_properties(&self) -> Vec<&Property> {
        self.properties.values().collect()
    }
    
    /// Get all individuals
    pub fn get_all_individuals(&self) -> Vec<&Individual> {
        self.individuals.values().collect()
    }
    
    /// Search for entities by label
    pub fn search_by_label(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        // Search in classes
        for class in self.classes.values() {
            if class.label.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    uri: class.uri.clone(),
                    label: class.label.clone(),
                    entity_type: EntityType::Class,
                });
            }
        }
        
        // Search in properties
        for property in self.properties.values() {
            if property.label.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    uri: property.uri.clone(),
                    label: property.label.clone(),
                    entity_type: EntityType::Property,
                });
            }
        }
        
        // Search in individuals
        for individual in self.individuals.values() {
            if individual.label.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    uri: individual.uri.clone(),
                    label: individual.label.clone(),
                    entity_type: EntityType::Individual,
                });
            }
        }
        
        results
    }
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub uri: String,
    pub label: String,
    pub entity_type: EntityType,
}

/// Entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Class,
    Property,
    Individual,
}