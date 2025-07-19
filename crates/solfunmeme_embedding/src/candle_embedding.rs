use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use serde_json;

#[cfg(feature = "with-candle")]
use candle_core::{Device, Tensor, DType};
#[cfg(feature = "with-candle")]
use candle_nn::VarBuilder;
#[cfg(feature = "with-candle")]
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
#[cfg(feature = "with-candle")]
use hf_hub::{api::sync::Api, Repo, RepoType};
#[cfg(feature = "with-candle")]
use tokenizers::Tokenizer;

use solfunmeme_clifford::{SolMultivector};

#[cfg(feature = "with-candle")]
pub fn embed_text(text: &str, device: &Device) -> anyhow::Result<Vec<f32>> {
    let api = hf_hub::api::sync::Api::new()?;
    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let revision = "main".to_string();

    let repo = api.repo(hf_hub.Repo.with_revision(model_id, hf_hub.RepoType.Model, revision));
    let config_filename = repo.get("config.json")?;
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let weights_filename = repo.get("model.safetensors")?;

    let config = std::fs::read_to_string(config_filename)?;
    let config: candle_transformers::models::bert::Config = serde_json::from_str(&config)?;
    let vb = unsafe { candle_core::VarBuilder::from_mmaped_safetensors(&[weights_filename], candle_core::DType::F32, device)? };
    let model = candle_transformers::models::bert::BertModel::load(vb, &config)?;

    let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_filename).map_err(|e| anyhow::anyhow!(e.to_string()))?;

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

    let input_ids = candle_core::Tensor::new(token_ids.as_slice(), device)?.unsqueeze(0)?;
    let token_type_ids = input_ids.zeros_like()?; // Assuming no token type ids for now
    let attention_mask = candle_core::Tensor::new(attention_mask.as_slice(), device)?.unsqueeze(0)?;

    let embeddings = model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?; // Pass attention_mask

    // Pool the embeddings (e.g., mean pooling)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
    let embeddings = (embeddings.sum_keepdim(1)? / (n_tokens as f64))?;
    let embeddings = embeddings.squeeze(0)?.flatten_all()?;

    Ok(embeddings.to_vec1()?)
}
