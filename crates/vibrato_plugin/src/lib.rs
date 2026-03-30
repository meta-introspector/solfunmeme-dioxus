use anyhow::Result;
use vibrato::tokenizer::Tokenizer;

pub struct VibratoPlugin {
    tokenizer: Tokenizer,
}

impl VibratoPlugin {
    pub fn new() -> Result<Self> {
        // This is a placeholder. Actual initialization would involve loading a model.
        let tokenizer = Tokenizer::new(vibrato::tokenizer::worker::WorkerBuilder::default());
        Ok(VibratoPlugin { tokenizer })
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        self.tokenizer.tokenize(text).iter().map(|x| x.surface.to_string()).collect()
    }
}
