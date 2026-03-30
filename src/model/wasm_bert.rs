use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

/// WASM-compatible embedding model using lightweight approaches
pub struct WasmBertEmbedder {
    // Simple hash-based embedding for demonstration
    // In production, you could use pre-computed embeddings or a lighter model
    embedding_cache: HashMap<String, Vec<f32>>,
    embedding_dim: usize,
}

impl WasmBertEmbedder {
    pub fn new(embedding_dim: usize) -> Self {
        let mut cache = HashMap::new();

        // Pre-populate with some common words for demonstration
        let common_words = vec![
            "hello",
            "world",
            "rust",
            "wasm",
            "bert",
            "embedding",
            "incredible",
            "awesome",
            "explorer",
            "journey",
            "inspiration",
            "love",
            "hate",
            "good",
            "bad",
            "happy",
            "sad",
            "fast",
            "slow",
        ];

        for (_i, word) in common_words.iter().enumerate() {
            let mut embedding = vec![0.0; embedding_dim];
            // Create a simple deterministic embedding based on word hash
            let hash = word
                .as_bytes()
                .iter()
                .fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
            for j in 0..embedding_dim {
                embedding[j] = (hash.wrapping_add(j as u32) as f32) / (u32::MAX as f32) * 2.0 - 1.0;
            }
            cache.insert(word.to_string(), embedding);
        }

        Self {
            embedding_cache: cache,
            embedding_dim,
        }
    }

    /// Generate embedding for text using lightweight approach
    pub fn embed_text(&mut self, text: &str) -> Vec<f32> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut text_embedding = vec![0.0; self.embedding_dim];

        for word in &words {
            let word_lower = word.to_lowercase();
            if let Some(word_embedding) = self.embedding_cache.get(&word_lower) {
                for (i, &val) in word_embedding.iter().enumerate() {
                    text_embedding[i] += val;
                }
            } else {
                // Generate embedding for unknown word
                let mut new_embedding = vec![0.0; self.embedding_dim];
                let hash = word_lower
                    .as_bytes()
                    .iter()
                    .fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
                for j in 0..self.embedding_dim {
                    new_embedding[j] =
                        (hash.wrapping_add(j as u32) as f32) / (u32::MAX as f32) * 2.0 - 1.0;
                }
                self.embedding_cache
                    .insert(word_lower.clone(), new_embedding.clone());

                for (i, &val) in new_embedding.iter().enumerate() {
                    text_embedding[i] += val;
                }
            }
        }

        // Normalize by number of words
        if !words.is_empty() {
            for val in text_embedding.iter_mut() {
                *val /= words.len() as f32;
            }
        }

        text_embedding
    }

    /// Calculate cosine similarity between two embeddings
    pub fn cosine_similarity(&self, embedding1: &[f32], embedding2: &[f32]) -> f32 {
        if embedding1.len() != embedding2.len() {
            return 0.0;
        }

        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for (a, b) in embedding1.iter().zip(embedding2.iter()) {
            dot_product += a * b;
            norm1 += a * a;
            norm2 += b * b;
        }

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        dot_product / (norm1.sqrt() * norm2.sqrt())
    }
}

/// WASM-compatible sentiment analysis
pub struct WasmSentimentAnalyzer {
    positive_words: std::collections::HashSet<String>,
    negative_words: std::collections::HashSet<String>,
}

impl WasmSentimentAnalyzer {
    pub fn new() -> Self {
        let positive_words: std::collections::HashSet<String> = [
            "good",
            "great",
            "excellent",
            "amazing",
            "wonderful",
            "fantastic",
            "awesome",
            "incredible",
            "love",
            "like",
            "happy",
            "joy",
            "pleasure",
            "beautiful",
            "perfect",
            "best",
            "outstanding",
            "superb",
            "brilliant",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let negative_words: std::collections::HashSet<String> = [
            "bad",
            "terrible",
            "awful",
            "horrible",
            "disgusting",
            "hate",
            "dislike",
            "sad",
            "angry",
            "furious",
            "disappointed",
            "upset",
            "worst",
            "ugly",
            "broken",
            "failed",
            "wrong",
            "evil",
            "cruel",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            positive_words,
            negative_words,
        }
    }

    pub fn analyze_sentiment(&self, text: &str) -> f32 {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut score = 0.0;

        for word in &words {
            let word_lower = word.to_lowercase();
            if self.positive_words.contains(&word_lower) {
                score += 1.0;
            } else if self.negative_words.contains(&word_lower) {
                score -= 1.0;
            }
        }

        // Normalize score to [-1, 1] range
        if !words.is_empty() {
            score / words.len() as f32
        } else {
            0.0
        }
    }
}

/// WASM-compatible text classification
pub struct WasmTextClassifier {
    categories: Vec<String>,
    category_keywords: HashMap<String, std::collections::HashSet<String>>,
}

impl WasmTextClassifier {
    pub fn new() -> Self {
        let mut category_keywords = HashMap::new();

        // Technology category
        category_keywords.insert(
            "technology".to_string(),
            [
                "computer",
                "software",
                "hardware",
                "programming",
                "code",
                "algorithm",
                "data",
                "network",
                "internet",
                "digital",
                "electronic",
                "system",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        // Science category
        category_keywords.insert(
            "science".to_string(),
            [
                "research",
                "experiment",
                "theory",
                "hypothesis",
                "discovery",
                "analysis",
                "study",
                "scientific",
                "laboratory",
                "chemistry",
                "physics",
                "biology",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        // Business category
        category_keywords.insert(
            "business".to_string(),
            [
                "company",
                "market",
                "profit",
                "investment",
                "strategy",
                "management",
                "finance",
                "economy",
                "trade",
                "commerce",
                "enterprise",
                "corporate",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        let categories = vec![
            "technology".to_string(),
            "science".to_string(),
            "business".to_string(),
        ];

        Self {
            categories,
            category_keywords,
        }
    }

    pub fn classify_text(&self, text: &str) -> Vec<(String, f32)> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut scores = Vec::new();

        for category in &self.categories {
            let mut score = 0.0;
            if let Some(keywords) = self.category_keywords.get(category) {
                for word in &words {
                    let word_lower = word.to_lowercase();
                    if keywords.contains(&word_lower) {
                        score += 1.0;
                    }
                }
            }

            // Normalize score
            if !words.is_empty() {
                score /= words.len() as f32;
            }

            scores.push((category.clone(), score));
        }

        // Sort by score (descending)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }
}

/// WASM-compatible text summarization (extractive)
pub struct WasmTextSummarizer {
    embedder: WasmBertEmbedder,
}

impl WasmTextSummarizer {
    pub fn new() -> Self {
        Self {
            embedder: WasmBertEmbedder::new(384),
        }
    }

    pub fn summarize(&mut self, text: &str, max_sentences: usize) -> String {
        let sentences: Vec<&str> = text
            .split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .collect();

        if sentences.len() <= max_sentences {
            return text.to_string();
        }

        // Calculate sentence embeddings
        let mut sentence_embeddings = Vec::new();
        for sentence in &sentences {
            let embedding = self.embedder.embed_text(sentence);
            sentence_embeddings.push(embedding);
        }

        // Calculate sentence importance based on similarity to other sentences
        let mut sentence_scores = Vec::new();
        for (i, embedding1) in sentence_embeddings.iter().enumerate() {
            let mut total_similarity = 0.0;
            for (j, embedding2) in sentence_embeddings.iter().enumerate() {
                if i != j {
                    total_similarity += self.embedder.cosine_similarity(embedding1, embedding2);
                }
            }
            sentence_scores.push((i, total_similarity));
        }

        // Sort by score and take top sentences
        sentence_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut selected_indices: Vec<usize> = sentence_scores
            .iter()
            .take(max_sentences)
            .map(|(i, _)| *i)
            .collect();
        selected_indices.sort();

        // Reconstruct summary
        selected_indices
            .iter()
            .map(|&i| sentences[i])
            .collect::<Vec<&str>>()
            .join(". ")
            + "."
    }
}

// WASM bindings
#[wasm_bindgen]
pub struct WasmBert {
    embedder: WasmBertEmbedder,
    sentiment_analyzer: WasmSentimentAnalyzer,
    text_classifier: WasmTextClassifier,
    summarizer: WasmTextSummarizer,
}

#[wasm_bindgen]
impl WasmBert {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&"Initializing WASM-compatible BERT-like functionality".into());

        Self {
            embedder: WasmBertEmbedder::new(384),
            sentiment_analyzer: WasmSentimentAnalyzer::new(),
            text_classifier: WasmTextClassifier::new(),
            summarizer: WasmTextSummarizer::new(),
        }
    }

    /// Generate embedding for text
    pub fn embed_text(&mut self, text: &str) -> js_sys::Float32Array {
        let embedding = self.embedder.embed_text(text);
        unsafe { js_sys::Float32Array::view(&embedding) }
    }

    /// Analyze sentiment of text
    pub fn analyze_sentiment(&self, text: &str) -> f32 {
        self.sentiment_analyzer.analyze_sentiment(text)
    }

    /// Classify text into categories
    pub fn classify_text(&self, text: &str) -> js_sys::Object {
        let classifications = self.text_classifier.classify_text(text);
        let result = js_sys::Object::new();

        for (category, score) in classifications {
            js_sys::Reflect::set(&result, &category.into(), &score.into()).unwrap();
        }

        result
    }

    /// Summarize text
    pub fn summarize(&mut self, text: &str, max_sentences: usize) -> String {
        self.summarizer.summarize(text, max_sentences)
    }

    /// Calculate similarity between two texts
    pub fn calculate_similarity(&mut self, text1: &str, text2: &str) -> f32 {
        let embedding1 = self.embedder.embed_text(text1);
        let embedding2 = self.embedder.embed_text(text2);
        self.embedder.cosine_similarity(&embedding1, &embedding2)
    }
}

// Export functions for direct use
#[wasm_bindgen]
pub fn create_embedding(text: &str) -> js_sys::Float32Array {
    let mut embedder = WasmBertEmbedder::new(384);
    let embedding = embedder.embed_text(text);
    unsafe { js_sys::Float32Array::view(&embedding) }
}

#[wasm_bindgen]
pub fn analyze_sentiment_wasm(text: &str) -> f32 {
    let analyzer = WasmSentimentAnalyzer::new();
    analyzer.analyze_sentiment(text)
}

//#[cfg(test)]
mod tests {

    #[test]
    fn test_embedding() {
        let mut embedder = WasmBertEmbedder::new(384);
        let embedding = embedder.embed_text("hello world");
        assert_eq!(embedding.len(), 384);
        assert!(embedding.iter().all(|&x| x >= -1.0 && x <= 1.0));
    }

    #[test]
    fn test_sentiment() {
        let analyzer = WasmSentimentAnalyzer::new();
        let positive_score = analyzer.analyze_sentiment("I love this amazing product");
        let negative_score = analyzer.analyze_sentiment("I hate this terrible product");
        assert!(positive_score > negative_score);
    }

    #[test]
    fn test_classification() {
        let classifier = WasmTextClassifier::new();
        let classifications = classifier.classify_text("computer programming software");
        assert!(!classifications.is_empty());
    }
}
