use anyhow::Result;
use extractous::Extractor;

pub struct ExtractousPlugin {
    extractor: Extractor,
}

impl ExtractousPlugin {
    pub fn new() -> Result<Self> {
        let extractor = Extractor::new();
        Ok(ExtractousPlugin { extractor })
    }

    pub fn extract_text(&self, html: &str) -> Result<String> {
        let text = self.extractor.extract_text(html)?;
        Ok(text)
    }
}
