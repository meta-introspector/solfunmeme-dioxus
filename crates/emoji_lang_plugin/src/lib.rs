use anyhow::{Result, anyhow};
use urlencoding::{encode, decode};
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use workflow_manager::{Workflow, WorkflowManager};

lazy_static::lazy_static! {
    pub static ref GLOBAL_WORKFLOW_MANAGER: std::sync::Mutex<WorkflowManager> = {
        std::sync::Mutex::new(WorkflowManager::new())
    };

    static ref EMOJI_MAP: HashMap<String, Meme> = {
        let mut map = HashMap::new();
        let content = fs::read_to_string("ontologies/solfunmem.jsonld")
            .expect("Failed to read ontologies/solfunmem.jsonld");
        let ontology_graph: OntologyGraph = serde_json::from_str(&content)
            .expect("Failed to parse ontologies/solfunmem.jsonld");

        for item in ontology_graph.graph {
            if let Ok(meme) = serde_json::from_value::<Meme>(item) {
                map.insert(meme.emoji_rep.clone(), meme);
            }
        }
        map
    };
}

#[derive(Debug, Deserialize)]
struct Meme {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    _type: String,
    name: String,
    #[serde(rename = "emojiRep")]
    emoji_rep: String,
}

#[derive(Debug, Deserialize)]
struct OntologyGraph {
    #[serde(rename = "@graph")]
    graph: Vec<Value>,
}

pub struct EmojiWorkflow {
    name: String,
    emoji_string: String,
}

impl EmojiWorkflow {
    pub fn new(name: String, emoji_string: String) -> Self {
        EmojiWorkflow { name, emoji_string }
    }

    pub fn from_url(url: &str) -> Result<Self> {
        let parts: Vec<&str> = url.split("workflow=").collect();
        if parts.len() < 2 {
            return Err(anyhow!("Invalid workflow URL"));
        }
        let encoded_emoji_string = parts[1];
        let emoji_string = decode(encoded_emoji_string)?.into_owned();
        Ok(EmojiWorkflow::new("url_workflow".to_string(), emoji_string))
    }

    pub fn to_url(&self) -> String {
        format!("http://example.com/workflow?workflow={}", encode(&self.emoji_string))
    }

    pub fn parse_emoji_string(&self) -> Vec<String> {
        let mut parsed_emojis = Vec::new();
        let mut current_string = self.emoji_string.as_str();

        while !current_string.is_empty() {
            let mut found_match = false;
            for (emoji_rep, meme) in EMOJI_MAP.iter() {
                if current_string.starts_with(emoji_rep) {
                    parsed_emojis.push(format!("{} ({})", meme.name, emoji_rep));
                    current_string = &current_string[emoji_rep.len()..];
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                // If no emoji match, take the first character and move on
                let char_len = current_string.chars().next().map_or(0, |c| c.len_utf8());
                if char_len > 0 {
                    parsed_emojis.push(current_string[..char_len].to_string());
                    current_string = &current_string[char_len..];
                } else {
                    break; // Should not happen if current_string is not empty
                }
            }
        }
        parsed_emojis
    }
}

impl Workflow for EmojiWorkflow {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> Result<()> {
        println!("Executing Emoji Workflow: {}", self.name);
        println!("Emoji String: {}", self.emoji_string);
        let parsed_emojis = self.parse_emoji_string();
        println!("Parsed Emojis: {:?}", parsed_emojis);
        // Here, you would map parsed_emojis to actual workflow steps
        Ok(())
    }

    fn get_process_configs(&self) -> Vec<workflow_manager::ProcessConfig> {
        // This workflow does not manage external processes directly
        Vec::new()
    }
}

pub fn register_emoji_workflow(name: String, emoji_string: String) {
    let workflow = EmojiWorkflow::new(name, emoji_string);
    GLOBAL_WORKFLOW_MANAGER.lock().unwrap().register_workflow(Box::new(workflow));
}
