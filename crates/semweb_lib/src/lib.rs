//! Semantic Web Library for SolFunMeme
//! 
//! This library provides semantic web functionality inspired by Tim Berners-Lee (timbl) 
//! and Dan Brickley (danbri) concepts, including CWM-like reasoning and RDF processing.
//! 
//! ## Core Features
//! - **Timbl Traits**: Semantic web reasoning and inference
//! - **Danbri Traits**: RDF processing and ontology management
//! - **CWM Integration**: Closed World Machine functionality
//! - **Turtle Generation**: RDF serialization with solfunmeme_rdf_utils
//! - **Ontology Integration**: Dynamic ontology loading and querying

pub mod timbl;
pub mod danbri;
pub mod cwm;
pub mod turtle;
pub mod ontology;
pub mod reasoning;
pub mod inference;
pub mod graph;
pub mod query;
pub mod serialization;
pub mod aspects;

pub use timbl::*;
pub use danbri::*;
pub use cwm::*;
pub use turtle::*;
pub use ontology::*;
pub use reasoning::*;
pub use inference::*;
pub use graph::*;
pub use query::*;
pub use use serialization::*;
pub use aspects::*;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

/// Error types for the semantic web library
#[derive(Debug, thiserror::Error)]
pub enum SemWebError {
    #[error("RDF parsing error: {0}")]
    RdfParse(String),
    
    #[error("Ontology error: {0}")]
    Ontology(String),
    
    #[error("Reasoning error: {0}")]
    Reasoning(String),
    
    #[error("Inference error: {0}")]
    Inference(String),
    
    #[error("Graph error: {0}")]
    Graph(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Sophia error: {0}")]
    Sophia(String),
}

/// Result type for semantic web operations
pub type SemWebResult<T> = Result<T, SemWebError>;

/// Core semantic web context that combines all functionality
pub struct SemWebContext<'a> {
    pub graph: RdfGraph<'a>,
    pub ontologies: Vec<Ontology>,
    pub reasoning_engine: ReasoningEngine,
    pub inference_engine: InferenceEngine,
}

impl<'a> SemWebContext<'a> {
    /// Create a new semantic web context
    pub fn new() -> Self {
        Self {
            graph: RdfGraph::new(),
            ontologies: Vec::new(),
            reasoning_engine: ReasoningEngine::new(),
            inference_engine: InferenceEngine::new(),
        }
    }
    
    /// Load an ontology from a Turtle file
    pub async fn load_ontology(&mut self, path: &str) -> SemWebResult<()> {
        let ontology = Ontology::from_turtle_file(path).await?;
        self.ontologies.push(ontology);
        Ok(())
    }
    
    /// Add triples from Turtle string
    pub fn add_turtle(&mut self, turtle_data: &str) -> SemWebResult<()> {
        self.graph.add_turtle_str(turtle_data)
            .map_err(|e| SemWebError::RdfParse(e.to_string()))
    }
    
    /// Query the graph using SPARQL-like syntax
    pub fn query(&self, query: &str) -> SemWebResult<Vec<(String, String, String)>> {
        // This is a placeholder, a real implementation would parse the query
        // and execute it against the graph.
        Ok(Vec::new())
    }
    
    /// Perform inference on the graph
    pub fn infer(&mut self) -> SemWebResult<()> {
        self.inference_engine.infer(&mut self.graph, &self.ontologies)
    }
    
    /// Export to Turtle format
    pub fn to_turtle(&self, path: &str) -> SemWebResult<()> {
        self.graph.serialize_to_turtle(std::path::Path::new(path)).map_err(|e| SemWebError::Serialization(e.to_string()))
    }
}

impl<'a> Default for SemWebContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semweb_context_basic_ops() -> SemWebResult<()> {
        let mut context = SemWebContext::new();

        // Add a triple
        context.graph.add_triple(
            "http://example.org/subject1",
            "http://example.org/predicate1",
            "http://example.org/object1",
        )?;

        // Serialize to string and check content
        let turtle_string = context.graph.serialize_to_turtle_string()?;
        assert!(turtle_string.contains("<http://example.org/subject1> <http://example.org/predicate1> <http://example.org/object1>"));

        // Add a literal triple
        context.graph.add_literal_triple(
            "http://example.org/subject2",
            "http://example.org/predicate2",
            "Hello, World!",
            "http://www.w3.org/2001/XMLSchema#string",
        )?;

        let turtle_string_with_literal = context.graph.serialize_to_turtle_string()?;
        assert!(turtle_string_with_literal.contains("<http://example.org/subject2> <http://example.org/predicate2> \"Hello, World!\"^^<http://www.w3.org/2001/XMLSchema#string>"));

        Ok(())
    }
}
