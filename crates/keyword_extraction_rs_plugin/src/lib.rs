use anyhow::Result;
use keyword_extraction_rs::KeywordExtractor;

pub struct KeywordExtractionRsPlugin {
    extractor: KeywordExtractor,
}

impl KeywordExtractionRsPlugin {
    pub fn new() -> Result<Self> {
        let extractor = KeywordExtractor::new(); // Placeholder for actual initialization
        Ok(KeywordExtractionRsPlugin { extractor })
    }

    pub fn extract_keywords(&self, text: &str) -> Result<Vec<String>> {
        let keywords = self.extractor.extract_keywords(text);
        Ok(keywords)
    }
}
