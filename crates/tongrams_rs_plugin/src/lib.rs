use anyhow::Result;
use tongrams_rs::Tongrams;

pub struct TongramsRsPlugin {
    tongrams: Tongrams,
}

impl TongramsRsPlugin {
    pub fn new(model_path: &str) -> Result<Self> {
        let tongrams = Tongrams::new(model_path)?; // Placeholder for actual model loading
        Ok(TongramsRsPlugin { tongrams })
    }

    pub fn query_ngrams(&self, text: &str) -> Vec<String> {
        self.tongrams.query(text)
    }
}
