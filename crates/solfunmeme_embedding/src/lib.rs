use anyhow::Result;
use std::collections::HashMap;
use solfunmeme_clifford::{SolMultivector};

mod ontology_processing;

pub fn load_emoji_multivectors(ontology_path: &str) -> Result<HashMap<String, (SolMultivector, String)>> {
    let mut graph = ontology_processing::read_and_parse_triples::read_and_parse_triples(ontology_path)?;
    let (emoji_data, concept_descriptions) = ontology_processing::extract_emoji_data::extract_emoji_data(&mut graph);
    let emoji_multivectors = ontology_processing::process_emoji_multivectors::process_emoji_multivectors(emoji_data, concept_descriptions)?;

    Ok(emoji_multivectors)
}