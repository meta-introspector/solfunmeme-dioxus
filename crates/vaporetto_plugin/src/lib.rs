use anyhow::Result;
use vaporetto::Sentence;
use vaporetto::Vaporetto;

pub struct VaporettoPlugin {
    vaporetto: Vaporetto,
}

impl VaporettoPlugin {
    pub fn new(model_data: &[u8]) -> Result<Self> {
        let vaporetto = Vaporetto::new(model_data)?;
        Ok(VaporettoPlugin { vaporetto })
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let sentence = Sentence::from_raw(text);
        self.vaporetto.tokenize(sentence).into_token_strings()
    }
}
