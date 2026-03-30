use crate::model::wasm_bert::{
    WasmBertEmbedder, WasmSentimentAnalyzer, WasmTextClassifier, WasmTextSummarizer,
};
use dioxus::prelude::*;

pub fn BertTestApp() -> Element {
    let mut input_text = use_signal(|| {
        String::from(
            "Hello world! This is an amazing test of the WASM-compatible BERT functionality.",
        )
    });
    let input_text2 = input_text.clone();
    let mut results = use_signal(|| String::new());
    let mut is_processing = use_signal(|| false);

    let mut process_text = move |text: &str| {
        is_processing.set(true);

        // Create instances of our WASM-compatible BERT components
        let mut embedder = WasmBertEmbedder::new(384);
        let sentiment_analyzer = WasmSentimentAnalyzer::new();
        let text_classifier = WasmTextClassifier::new();
        let mut summarizer = WasmTextSummarizer::new();

        // Process the text
        let embedding = embedder.embed_text(text);
        let sentiment = sentiment_analyzer.analyze_sentiment(text);
        let classifications = text_classifier.classify_text(text);
        let summary = summarizer.summarize(text, 2);

        // Format results
        let mut result_text = String::new();
        result_text.push_str("=== WASM BERT Analysis Results ===\n\n");

        result_text.push_str(&format!("Input Text: {}\n\n", text));

        result_text.push_str("📊 Embedding (first 10 dimensions):\n");
        for (i, &val) in embedding.iter().take(10).enumerate() {
            result_text.push_str(&format!("  [{}]: {:.4}", i, val));
            if i < 9 {
                result_text.push_str(", ");
            }
        }
        result_text.push_str("\n\n");

        result_text.push_str(&format!(
            "😊 Sentiment Score: {:.4} ({})\n\n",
            sentiment,
            if sentiment > 0.1 {
                "Positive"
            } else if sentiment < -0.1 {
                "Negative"
            } else {
                "Neutral"
            }
        ));

        result_text.push_str("🏷️ Text Classification:\n");
        for (category, score) in classifications.iter().take(3) {
            result_text.push_str(&format!("  {}: {:.4}\n", category, score));
        }
        result_text.push_str("\n");

        result_text.push_str(&format!("📝 Summary:\n{}\n", summary));

        results.set(result_text);
        is_processing.set(false);
    };

    rsx! {
        div {
            class: "bert-test-container",
            style: "max-width: 800px; margin: 0 auto; padding: 20px; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;",

            h1 {
                style: "color: #2c3e50; text-align: center; margin-bottom: 30px;",
                "🧠 WASM-Compatible BERT Test"
            }

            div {
                style: "background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 20px;",

                h3 { style: "color: #495057; margin-bottom: 15px;", "Input Text" }

                textarea {
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value().clone()),
                    style: "width: 100%; height: 100px; padding: 10px; border: 1px solid #ced4da; border-radius: 4px; font-size: 14px; resize: vertical;",
                    placeholder: "Enter text to analyze..."
                }

                button {
                    onclick: move |_| process_text(&input_text()),
                    disabled: is_processing(),
                    style: "margin-top: 10px; padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    if is_processing() { "Processing..." } else { "Analyze Text" }
                }
            }

            if !results().is_empty() {
                div {
                    style: "background: white; padding: 20px; border: 1px solid #dee2e6; border-radius: 8px;",

                    h3 { style: "color: #495057; margin-bottom: 15px;", "Analysis Results" }

                    pre {
                        style: "background: #f8f9fa; padding: 15px; border-radius: 4px; white-space: pre-wrap; font-family: 'Courier New', monospace; font-size: 13px; line-height: 1.4; overflow-x: auto;",
                        "{results}"
                    }
                }
            }

            div {
                style: "margin-top: 30px; padding: 20px; background: #e3f2fd; border-radius: 8px; border-left: 4px solid #2196f3;",

                h4 { style: "color: #1976d2; margin-bottom: 10px;", "ℹ️ About This Implementation" }

                p { style: "color: #424242; line-height: 1.6; margin-bottom: 10px;",
                    "This demonstrates a WASM-compatible alternative to rust-bert that works entirely in the browser.
                    While not as sophisticated as the full BERT model, it provides:"
                }

                ul { style: "color: #424242; line-height: 1.6;",
                    li { "🔤 Text embeddings using hash-based word vectors" }
                    li { "😊 Sentiment analysis using keyword matching" }
                    li { "🏷️ Text classification with predefined categories" }
                    li { "📝 Extractive text summarization" }
                    li { "🔍 Cosine similarity calculations" }
                }

                p { style: "color: #424242; line-height: 1.6; margin-top: 15px;",
                    "This approach avoids the heavy computational requirements and C++ dependencies of the full rust-bert library,
                    making it suitable for browser-based applications."
                }
            }
        }
    }
}
