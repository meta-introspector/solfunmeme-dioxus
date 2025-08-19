use anyhow::Result;
use std::collections::HashMap;
use std::fs;

use sophia_turtle::parser::turtle::parse_str;
use sophia_api::term::SimpleTerm;
use sophia_api::triple::Triple;
use sophia_api::source::TripleSource;

#[cfg(feature = "gpu_backend")]
use crate::clifford::{BertCliffordEncoder, BertConfig, SolMultivector};
use crate::embedding;

// Helper function to convert a SimpleTerm to a String
fn term_to_string(term: &SimpleTerm) -> String {
    match term {
        SimpleTerm::Iri(iri) => iri.to_string(),
        SimpleTerm::LiteralDatatype(value, _) => value.to_string(),
        SimpleTerm::LiteralLanguage(value, _) => value.to_string(),
        _ => "".to_string(), // Handle other term types as needed
    }
}

#[cfg(feature = "gpu_backend")]
pub fn load_emoji_multivectors(ontology_path: &str, device: &Device) -> Result<HashMap<String, (SolMultivector, String)>> {
    let turtle_data = fs::read_to_string(ontology_path)?;
    let triples: Vec<[SimpleTerm; 3]> = parse_str(&turtle_data).collect_triples().map_err(|e| anyhow::anyhow!("Failed to parse Turtle: {:?}", e))?;

    let mut emoji_data: HashMap<String, (String, String)> = HashMap::new(); // emoji_char -> (concept_id, category)
    let mut concept_descriptions: HashMap<String, String> = HashMap::new(); // concept_id -> description

    let em_ns = "http://example.org/emoji#";

    for t in triples.iter() {
        let subject = term_to_string(t.s());
        let predicate = term_to_string(t.p());
        let object = term_to_string(t.o());

        if subject.starts_with(em_ns) {
            let concept_id = subject.replace(em_ns, "");
            if predicate == format!("{}emoji", em_ns) {
                let emoji_char = object.trim_matches('"').to_string();
                let category = triples.iter()
                    .find(|t2| term_to_string(t2.s()) == subject && term_to_string(t2.p()) == format!("{}category", em_ns))
                    .map(|t2| term_to_string(t2.o()).trim_matches('"').to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                emoji_data.insert(emoji_char, (concept_id.clone(), category));
            } else if predicate == format!("{}description", em_ns) {
                concept_descriptions.insert(concept_id, object.trim_matches('"').to_string());
            }
        }
    }

    let mut emoji_multivectors: HashMap<String, (SolMultivector, String)> = HashMap::new();
    let config = BertConfig::default();
    let encoder = BertCliffordEncoder::new(config);

    for (emoji_char, (concept_id, category)) in emoji_data {
        let description = concept_descriptions.get(&concept_id).cloned().unwrap_or(concept_id.clone());
        let bert_embedding = embedding::embed_text(&description, device)?;
        let multivector = encoder.encode_embedding(&bert_embedding)?;
        emoji_multivectors.insert(emoji_char, (multivector, category));
    }

    Ok(emoji_multivectors)
}

#[cfg(not(feature = "gpu_backend"))]
pub fn load_emoji_multivectors(_ontology_path: &str) -> Result<HashMap<String, (Vec<f32>, String)>> {
    // Dummy implementation for when GPU backend is not enabled
    Ok(HashMap::new())
}
