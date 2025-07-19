// use anyhow::Result;
// use candle_core::{Device, Tensor};
// use candle_nn::VarBuilder;
// use candle_transformers::models::bert::{BertModel, Config};
// use hf_hub::{api::sync::Api, Repo, RepoType};
// use tokenizers::Tokenizer;

// const MODEL_ID: &str = "sentence-transformers/all-MiniLM-L12-v2";
// const REVISION: &str = "main";

// pub fn embed_text(text: &str) -> Result<Vec<f32>> {
//     let api = Api::new()?;
//     let repo = Repo::with_revision(MODEL_ID.to_string(), RepoType::Model, REVISION.to_string());
//     let tokenizer_path = api.get(&repo, "tokenizer.json")?;
//     let model_path = api.get(&repo, "model.safetensors")?;

//     let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::from)?;

//     let device = Device::Cpu;
//     let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], false, device)? };
//     let model = BertModel::load(vb, &Config::for_sentence_embedding())?;

//     let tokens = tokenizer
//         .encode(text, true)
//         .map_err(anyhow::Error::from)?
//         .get_ids()
//         .to_vec();
//     let token_ids = Tensor::new(&tokens, &device)?;
//     let token_type_ids = token_ids.zeros_like()?;
//     let embeddings = model.forward(&token_ids, &token_type_ids)?;

//     let (
//         n_sentences,
//         _n_tokens,
//         hidden_size,
//     ) = embeddings.dims3()?;
//     let embeddings = (embeddings.sum(1)? / (n_sentences as f32))?;
//     let embeddings = embeddings.flatten_all()?;

//     Ok(embeddings.to_vec1()?)
// }