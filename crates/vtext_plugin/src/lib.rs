use anyhow::Result;
use vtext::tokenize::VtextTokenizerParams;
use vtext::vectorize::TfidfVectorizer;

pub struct VtextPlugin {
    tokenizer_params: VtextTokenizerParams,
    vectorizer: TfidfVectorizer,
}

impl VtextPlugin {
    pub fn new() -> Result<Self> {
        let tokenizer_params = VtextTokenizerParams::default();
        let vectorizer = TfidfVectorizer::default();
        Ok(VtextPlugin { tokenizer_params, vectorizer })
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        self.tokenizer_params.tokenizer().tokenize(text)
    }

    pub fn vectorize(&mut self, texts: &[&str]) -> Result<Vec<Vec<f64>>> {
        let X = self.vectorizer.fit_transform(texts);
        Ok(X.outer_iter().map(|row| row.iter().cloned().collect()).collect())
    }
}
