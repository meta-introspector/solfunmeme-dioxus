use solfunmeme_function_analysis::CodeChunk as CodeSnippet;
//use tclifford::Multivector

pub fn annotate_code_snippet(_snippet: &CodeSnippet) {}
// pub fn annotate_code_snippet(snippet: &CodeSnippet) -> AnnotatedWord {
//         // let embedding = rust_bert_embed_with_context(&snippet.content, None);
//         // let reduced = pca_reduce(&embedding, 3);
//         // let sentiment = if *enable_sentiment.read() {
//         //     WasmSentimentAnalyzer::new().analyze(&snippet.content)
//         // } else {
//         //     0.0
//         // };
//         // let multivector = Multivector::from_vector(reduced.try_into().unwrap_or([0.0; 3]));

//         // if *enable_wikidata.read() {
//         //     if let Some(data) = wikidata_data.read().as_ref() {
//         //         return annotator.annotate_word(&snippet.content, data);
//         //     }
//         // }

//      AnnotatedWord {
//         //     word: snippet.content.clone(),
//         //     primary_emoji: "💻".to_string(),
//         //     secondary_emoji: "🔍".to_string(),
//         //     wikidata: None,
//         //     embedding,
//         //     multivector,
//         //     sentiment,
//      }
// }
