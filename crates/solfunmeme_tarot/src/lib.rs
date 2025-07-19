use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use libloading::{Library, Symbol};
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

/// Represents a generic semiotic entity loaded from the ontology.
/// This can be a task, a concept, or eventually, a Tarot card,
/// identified by its unique URI.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SemioticEntity {
    pub uri: String,
    pub label: String, // The human-readable part of the URI, e.g., "task"
    pub emoji: String,
    pub category: String,
}

/// A trait for dynamically loaded meme objects.
pub trait ExecutableMeme: Send + Sync {
    fn execute(&self) -> String;
    fn get_name(&self) -> &str;
}

/// Represents the binding of a SemioticEntity to a dynamic function.
#[derive(Debug, Clone)]
pub struct FunctionBinding {
    pub entity: SemioticEntity,
    pub library_path: String,
    pub function_symbol: String,
}

/// Manages the ontology and the bindings between entities and functions.
#[derive(Debug)]
pub struct TarotEngine<'a> {
    entities: HashMap<String, SemioticEntity>, // Maps URI to Entity
    bindings: HashMap<String, FunctionBinding>,
    graph: RdfGraph<'a>,
}

impl<'a> TarotEngine<'a> {
    /// Loads entities from a Turtle ontology file.
    pub fn from_ontology(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut graph = RdfGraph::from_file(path)?;
        graph.namespaces.add_namespace("concept", "https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#vibe:")?;
        graph.namespaces.add_namespace("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;
        graph.namespaces.add_namespace("emoji", "https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#")?;

        let mut entities = HashMap::new();
        let concept_term = graph.namespaces.get_term("concept", "Concept")?;
        let type_term = graph.namespaces.get_term("rdf", "type")?;

        let subjects = graph.get_subjects_with_property(&type_term, &concept_term)?;

        for subject in subjects {
            let uri = subject.iri().unwrap().as_str().to_string();
            let label = uri.split('#').last().unwrap_or("").to_string();

            let emoji_prop = graph.namespaces.get_term("emoji", "emoji")?;
            let category_prop = graph.namespaces.get_term("emoji", "vibe:category")?;

            let emoji = graph.get_property_value(&subject, &emoji_prop)?.unwrap_or_default();
            let category = graph.get_property_value(&subject, &category_prop)?.unwrap_or_default();

            let entity = SemioticEntity {
                uri: uri.clone(),
                label,
                emoji,
                category,
            };
            entities.insert(uri, entity);
        }

        Ok(TarotEngine {
            entities,
            bindings: HashMap::new(),
            graph,
        })
    }

    /// Binds a function to a semiotic entity by its URI.
    pub fn add_binding(&mut self, uri: &str, library_path: &str, function_symbol: &str) -> Result<(), String> {
        if let Some(entity) = self.entities.get(uri) {
            let binding = FunctionBinding {
                entity: entity.clone(),
                library_path: library_path.to_string(),
                function_symbol: function_symbol.to_string(),
            };
            self.bindings.insert(uri.to_string(), binding);
            Ok(())
        } else {
            Err(format!("Entity with URI '{}' not found in ontology.", uri).into())
        }
    }

    pub fn get_entity(&self, uri: &str) -> Option<&SemioticEntity> {
        self.entities.get(uri)
    }

    pub fn get_binding(&self, uri: &str) -> Option<&FunctionBinding> {
        self.bindings.get(uri)
    }

    pub fn all_entities(&self) -> Vec<&SemioticEntity> {
        self.entities.values().collect()
    }

    /// Executes the function bound to a semiotic entity, returning an ExecutableMeme object.
    pub fn execute_binding(&self, uri: &str) -> Result<Box<dyn ExecutableMeme>, Box<dyn Error>> {
        if let Some(binding) = self.get_binding(uri) {
            let lib = unsafe { Library::new(&binding.library_path)? };
            let func: Symbol<unsafe extern "C" fn() -> Box<dyn ExecutableMeme>> = unsafe { lib.symbol(binding.function_symbol.as_bytes()) }?;
            
            println!("Executing function for entity: {:?}", binding.entity);
            let result = unsafe { func() };
            Ok(result)
        } else {
            Err(format!("No binding found for URI '{}'", uri).into())
        }
    }
}
