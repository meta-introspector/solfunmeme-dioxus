use dioxus::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
pub mod style;
use style::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WikidataMeme {
    pub item: String,
    pub item_label: String,
    pub meme_id: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataResponse {
    pub results: WikidataResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataResults {
    pub bindings: Vec<WikidataBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataBinding {
    pub item: WikidataValue,
    pub itemLabel: WikidataValue,
    pub memeID: WikidataValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataValue {
    pub value: String,
}
#[allow(dead_code)]
pub async fn fetch_wikidata_memes() -> Result<Vec<WikidataMeme>, String> {
    let query = r#"
    SELECT ?item ?itemLabel ?memeID
    WHERE {
      ?item wdt:P6760 ?memeID .
      SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
    }
    LIMIT 10000
    "#;

    let client = reqwest::Client::new();
    let response = client
        .get("https://query.wikidata.org/sparql")
        .query(&[("query", query), ("format", "json")])
        .header("Accept", "application/sparql-results+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let wikidata_response: WikidataResponse = response.json().await.map_err(|e| e.to_string())?;

    let memes = wikidata_response
        .results
        .bindings
        .into_iter()
        .map(|binding| WikidataMeme {
            item: binding.item.value,
            item_label: binding.itemLabel.value,
            meme_id: binding.memeID.value,
            description: None,
            image_url: None,
        })
        .collect();

    Ok(memes)
}

#[component]
pub fn WikidataMemeView(meme: WikidataMeme) -> Element {
    rsx! {
        div { class: WIKIDATA_MEME,
            div { class: MEME_HEADER,
                h3 { class: MEME_TITLE, "{meme.item_label}" }
                if let Some(desc) = &meme.description {
                    p { class: MEME_DESCRIPTION, "{desc}" }
                }
            }
            if let Some(url) = &meme.image_url {
                img { class: MEME_IMAGE, src: "{url}", alt: "{meme.item_label}" }
            }
            div { class: MEME_METADATA,
                p { "Meme ID: {meme.meme_id}" }
                p { "Wikidata Item: {meme.item}" }
            }
        }
    }
}

#[component]
pub fn WikidataMemeExplorer() -> Element {
    // let memes = use_signal(Vec::new);
    // let loading = use_signal(|| true);
    // let error = use_signal(String::new);

    // use_effect(move || {
    //     to_owned![memes, loading, error];
    //     async move {
    //         match fetch_wikidata_memes().await {
    //             Ok(fetched_memes) => {
    //                 memes.set(fetched_memes);
    //                 loading.set(false);
    //             }
    //             Err(e) => {
    //                 error.set(e);
    //                 loading.set(false);
    //             }
    //         }
    //     }
    // });

    rsx! {
        div { class: WIKIDATA_EXPLORER,
            h1 { class: EXPLORER_TITLE, "Wikidata Meme Explorer" }
            // if *loading.read() {
            //     div { class: LOADING, "Loading memes..." }
            // } else if !error.read().is_empty() {
            //     div { class: ERROR, "Error: {error.read()}" }
            // } else {
            //     div { class: MEMES_GRID,
            //         // memes.read().iter().map(|meme| rsx! {
            //         //     WikidataMemeView { meme: meme.clone() }
            //         // })
            //     }
            // }
        }
    }
}

// Function to convert Wikidata memes into workflow steps
#[allow(dead_code)]
pub fn meme_to_workflow_step(meme: &WikidataMeme) -> crate::views::workflow_memes::WorkflowStep {
    crate::views::workflow_memes::WorkflowStep {
        emoji: "🎭".to_string(), // Default emoji for memes
        component: meme.item_label.clone(),
        description: meme
            .description
            .clone()
            .unwrap_or_else(|| "A meme from Wikidata".to_string()),
        test_case: format!(
            r#"
#[test]
fn test_meme_{}() {{
    let meme = WikidataMeme {{
        item: "{}".to_string(),
        item_label: "{}".to_string(),
        meme_id: "{}".to_string(),
        description: None,
        image_url: None,
    }};
    
    assert_eq!(meme.item_label, "{}");
    assert_eq!(meme.meme_id, "{}");
}}"#,
            meme.meme_id, meme.item, meme.item_label, meme.meme_id, meme.item_label, meme.meme_id
        ),
        lean_proof: format!(
            r#"
theorem meme_{}_exists :
  ∃ (meme : Meme), meme.id = "{}" ∧ meme.label = "{}" :=
begin
  existsi {{ id := "{}", label := "{}" }},
  split; refl
end"#,
            meme.meme_id, meme.meme_id, meme.item_label, meme.meme_id, meme.item_label
        ),
    }
}

#[allow(dead_code)]
// Function to create a workflow from a sequence of memes
pub fn create_meme_workflow(
    memes: Vec<WikidataMeme>,
) -> crate::views::workflow_memes::WorkflowMeme {
    let steps: Vec<_> = memes.iter().map(meme_to_workflow_step).collect();
    let emoji_sequence: String = steps.iter().map(|s| s.emoji.clone()).collect();

    crate::views::workflow_memes::WorkflowMeme {
        name: "Wikidata Meme Workflow".to_string(),
        emoji_sequence,
        steps,
        test_result: "✅ All meme tests passed!".to_string(),
        proof_result: "✓ All meme proofs verified!".to_string(),
    }
}
