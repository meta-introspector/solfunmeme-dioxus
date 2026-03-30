use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use memes;

// Define meme categories
#[derive(Debug)]
enum MemeCategory {
    ImageBased,
    VideoBased,
    TextPhrase,
    ChallengeTrend,
    CharacterFictional,
    CulturalRegional,
    ParodySatirical,
    Unknown,
}

impl MemeCategory {
    fn from_description(description: &str, label: &str) -> Self {
        let desc = description.to_lowercase();
        let label = label.to_lowercase();

        // Heuristic-based classification
        if desc.contains("image") || desc.contains("photo") || desc.contains("cartoon") || label.contains("cat") {
            MemeCategory::ImageBased
        } else if desc.contains("video") || desc.contains("viral video") || desc.contains("youtube") {
            MemeCategory::VideoBased
        } else if desc.contains("phrase") || desc.contains("slang") || desc.contains("quote") || desc.contains("typo") {
            MemeCategory::TextPhrase
        } else if desc.contains("challenge") || desc.contains("trend") || desc.contains("hashtag") {
            MemeCategory::ChallengeTrend
        } else if desc.contains("character") || desc.contains("fictional") || desc.contains("mascot") {
            MemeCategory::CharacterFictional
        } else if desc.contains("chinese") || desc.contains("russian") || desc.contains("japanese") || desc.contains("regional") {
            MemeCategory::CulturalRegional
        } else if desc.contains("parody") || desc.contains("satire") || desc.contains("conspiracy") || desc.contains("mock") {
            MemeCategory::ParodySatirical
        } else {
            MemeCategory::Unknown
        }
    }

    fn to_string(&self) -> &str {
        match self {
            MemeCategory::ImageBased => "Image-Based",
            MemeCategory::VideoBased => "Video-Based",
            MemeCategory::TextPhrase => "Text/Phrase-Based",
            MemeCategory::ChallengeTrend => "Challenge/Trend-Based",
            MemeCategory::CharacterFictional => "Character/Fictional",
            MemeCategory::CulturalRegional => "Cultural/Regional",
            MemeCategory::ParodySatirical => "Parody/Satirical",
            MemeCategory::Unknown => "Unknown",
        }
    }
}

// Structs for SPARQL response (same as before)
#[derive(Debug, Serialize, Deserialize)]
struct SparqlResponse {
    head: Head,
    results: Results,
}

#[derive(Debug, Serialize, Deserialize)]
struct Head {
    vars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Results {
    bindings: Vec<Binding>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Binding {
    item: Value,
    itemLabel: Value,
    #[serde(default)]
    description: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    #[serde(rename = "type")]
    value_type: String,
    value: String,
    #[serde(rename = "xml:lang", default)]
    lang: Option<String>,
}

fn meme_main() -> Result<(), Box<dyn Error>> {
    // SPARQL query for internet memes
    let query = r#"
    SELECT DISTINCT ?item ?itemLabel ?description
    WHERE {
      {
        ?item wdt:P31 wd:Q2927074 . # instance of internet meme
      } UNION {
        ?item wdt:P31 ?type . 
        ?type wdt:P279* wd:Q2927074 . # subclasses of internet meme
      } UNION {
        ?item wdt:P31 ?type .
        FILTER (?type IN (wd:Q104699263, wd:Q2587068, wd:Q1063745)) # viral phenomenon, internet slang, internet culture
      }
      OPTIONAL { ?item schema:description ?description . FILTER (lang(?description) = "en") }
      SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en" . }
    }
    LIMIT 1000
    "#;

    // Create HTTP client
    let client = Client::new();
    let endpoint = "https://query.wikidata.org/sparql";

    // Encode query parameters
    let params = [("query", query), ("format", "json")];
    let response = client
        .get(endpoint)
        .query(&params)
        .send()?
        .json::<SparqlResponse>()?;

    // Process and categorize results
    for binding in response.results.bindings {
        let item_uri = &binding.item.value;
        let label = &binding.itemLabel.value;
        let description = &binding.description.value;
        let category = MemeCategory::from_description(description, label);

        println!("Meme: {}", label);
        println!("URI: {}", item_uri);
        println!("Category: {}", category.to_string());
        if !description.is_empty() {
            println!("Description: {}", description);
        }
        println!("---");
    }

    Ok(())
}