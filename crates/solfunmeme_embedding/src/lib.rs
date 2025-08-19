use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use serde_json;

use candle_core::{Device, Tensor, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

use sophia_turtle::parser::turtle::parse_str;
use sophia_api::term::SimpleTerm;
use sophia_api::triple::Triple;
use sophia_api::source::TripleSource;

use solfunmeme_clifford::{BertCliffordEncoder, SolMultivector, BertConfig as CliffordBertConfig}; // Adjusted import

pub fn embed_text(text: &str, device: &Device) -> Result<Vec<f32>> {
    let api = Api::new()?;
    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let revision = "main".to_string();

    let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));
    let config_filename = repo.get("config.json")?;
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let weights_filename = repo.get("model.safetensors")?;

    let config = std::fs::read_to_string(config_filename)?;
    let config: BertConfig = serde_json::from_str(&config)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DType::F32, device)? };
    let model = BertModel::load(vb, &config)?;

    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let encoding = tokenizer.encode(text, true).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let token_ids = encoding.get_ids().to_vec();
    let attention_mask = encoding.get_attention_mask().to_vec();

    // Truncate if necessary to max_position_embeddings
    let max_len = config.max_position_embeddings;
    let token_ids = if token_ids.len() > max_len {
        token_ids[..max_len].to_vec()
    } else {
        token_ids
    };
    let attention_mask = if attention_mask.len() > max_len {
        attention_mask[..max_len].to_vec()
    } else {
        attention_mask
    };

    let input_ids = Tensor::new(token_ids.as_slice(), device)?.unsqueeze(0)?;
    let token_type_ids = input_ids.zeros_like()?; // Assuming no token type ids for now
    let attention_mask = Tensor::new(attention_mask.as_slice(), device)?.unsqueeze(0)?;

    let embeddings = model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?; // Pass attention_mask

    // Pool the embeddings (e.g., mean pooling)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
    let embeddings = (embeddings.sum_keepdim(1)? / (n_tokens as f64))?;
    let embeddings = embeddings.squeeze(0)?.flatten_all()?;

    Ok(embeddings.to_vec1()?)
}

pub fn load_emoji_multivectors(ontology_path: &str, device: &Device) -> Result<HashMap<String, (SolMultivector, String)>> {
    let turtle_data = fs::read_to_string(ontology_path)?;
    let triples: Vec<[SimpleTerm; 3]> = parse_str(&turtle_data).collect_triples().map_err(|e| anyhow::anyhow!("Failed to parse Turtle: {:?}" , e))?;

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
    let config = CliffordBertConfig::default(); // Use CliffordBertConfig
    let encoder = BertCliffordEncoder::new(config);

    for (emoji_char, (concept_id, category)) in emoji_data {
        let description = concept_descriptions.get(&concept_id).cloned().unwrap_or(concept_id.clone());
        let bert_embedding = embed_text(&description, device)?; // Pass device
        let multivector = encoder.encode_embedding(&bert_embedding)?; // Pass device
        emoji_multivectors.insert(emoji_char, (multivector, category));
    }

    Ok(emoji_multivectors)
}

// Helper function to convert a SimpleTerm to a String
fn term_to_string(term: &SimpleTerm) -> String {
    match term {
        SimpleTerm::Iri(iri) => iri.to_string(),
        SimpleTerm::LiteralDatatype(value, _) => value.to_string(),
        SimpleTerm::LiteralLanguage(value, _) => value.to_string(),
        _ => "".to_string(), // Handle other term types as needed
    }
}
