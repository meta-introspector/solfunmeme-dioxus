
#[cfg(feature = "with-embedding")]
use anyhow::Result;
#[cfg(feature = "with-embedding")]
use solfunmeme_embedding::{embed_text, load_emoji_multivectors, find_closest_emojis};
#[cfg(feature = "with-embedding")]
use solfunmeme_clifford::{BertCliffordEncoder, BertConfig, get_sieve_address};
#[cfg(feature = "with-embedding")]
use candle_core::Device;
#[cfg(feature = "with-embedding")]
use tokenizers::Tokenizer;
#[cfg(feature = "with-embedding")]
use solfunmeme_function_analysis::{AnalyzedFunction, ClosestEmojiInfo};
#[cfg(feature = "with-embedding")]
use std::collections::HashMap;

#[cfg(feature = "with-embedding")]
pub struct EmbeddingContext {
    pub bert_encoder: BertCliffordEncoder,
    pub emoji_multivectors: HashMap<String, (Vec<f32>, solfunmeme_clifford::SieveAddress, String)>,
    pub device: Device,
}

#[cfg(feature = "with-embedding")]
impl EmbeddingContext {
    pub fn new(ontology_path: &str, use_gpu: bool) -> Result<Self> {
        let config = BertConfig::default();
        let encoder = BertCliffordEncoder::new(config);
        let emoji_mvs = load_emoji_multivectors(ontology_path)?;
        let device = if use_gpu { Device::cuda_if_available(0)? } else { Device::Cpu };
        Ok(Self {
            bert_encoder: encoder,
            emoji_multivectors: emoji_mvs,
            device,
        })
    }

    pub fn process_function_embedding(&self, func_info: &mut AnalyzedFunction) -> Result<()> {
        let embedding = embed_text(&func_info.semantic_summary, &self.device)?;
        let multivector = self.bert_encoder.encode_embedding(&embedding)?;
        let sieve_address = get_sieve_address(&multivector);
        let closest_emojis = find_closest_emojis(&multivector, &self.emoji_multivectors);

        func_info.multivector_str = format!("{:?}", multivector);
        func_info.sieve_address = sieve_address;
        func_info.closest_emojis = closest_emojis;
        Ok(())
    }

    pub fn process_snippet_embedding(&self, snippet_content: &str) -> Result<(String, String, Vec<ClosestEmojiInfo>)> {
        let embedding = embed_text(snippet_content, &self.device).ok().unwrap_or_default();
        let multivector = self.bert_encoder.encode_embedding(&embedding)?;
        let sieve_address = get_sieve_address(&multivector);
        let closest_emojis = find_closest_emojis(&multivector, &self.emoji_multivectors);
        Ok((format!("{:?}", multivector), sieve_address, closest_emojis))
    }

    pub fn process_token_embedding(&self, word: &str) -> Result<String> {
        let embedding = embed_text(word, &self.device)?;
        let multivector = self.bert_encoder.encode_embedding(&embedding)?;
        Ok(format!("{:?}", multivector))
    }
}
