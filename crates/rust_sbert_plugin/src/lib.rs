use anyhow::Result;
use rust_sbert::SentenceEmbedder;

pub struct RustSbertPlugin {
    embedder: SentenceEmbedder,
}

impl RustSbertPlugin {
    pub fn new(model_path: &str) -> Result<Self> {
        let embedder = SentenceEmbedder::new(model_path)?;
        Ok(RustSbertPlugin { embedder })
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let embedding = self.embedder.embed(text)?;
        Ok(embedding)
    }
}
