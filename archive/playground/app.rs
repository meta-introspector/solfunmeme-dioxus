use dioxus::prelude::*;
//use crate::app::{FAVICON, TAILWIND_CSS};
//use crate::extractor::components::app::EmbeddingApp;
//use crate::extractor::system::clipboard::copy_all_snippets_combined;
//ouse crate::extractor::error;
//use crate::password_manager::DecryptedEntry;
// use crate::playground::MenuOption::Airdrop;
// todo rename : airdrop::Airdrop,
//pub mod embedding;
//use crate::playground::embedding::EmbeddingApp;


use signals_lib::rdf_signal::{new_rdf_signal_from_turtle, query_triples};
use std::fs;

#[derive(PartialEq, Clone)]
pub enum MenuOption {
    Embedding,
    PerformanceCharts,
    BertTest,
    RustParser,
    #[allow(dead_code)]
    MemeManagement,
    #[allow(dead_code)]
    ExpressionParsing,
    #[allow(dead_code)]
    Encryption,
    #[allow(dead_code)]
    MetaMemeOperations,
    #[allow(dead_code)]
    StylingAndEmojis,
    #[allow(dead_code)]
    CryptoFrontend,
    #[allow(dead_code)]
    Memes,
    #[allow(dead_code)]
    Lean,
    #[allow(dead_code)]
    ConnectionManagement,
    #[allow(dead_code)]
    ConnectionList,
    #[allow(dead_code)]
    SendSol,
    #[allow(dead_code)]
    ReceiveSol,
    #[allow(dead_code)]
    QueryAccounts,
    #[allow(dead_code)]
    Dashboard,
    #[allow(dead_code)]
    Connections,
    #[allow(dead_code)]
    ClustersManagement,
    #[allow(dead_code)]
    Clusters,
    #[allow(dead_code)]
    Airdrop,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    ComponentMemes,
    #[allow(dead_code)]
    MonsterMetaMeme,
    #[allow(dead_code)]
    SolFunMeme,
    #[allow(dead_code)]
    Extractor,
    SourceBrowser,
    EmojiMatrix,
    Mcp
}

#[component]
pub fn PlaygroundApp() -> Element {
    // Load Turtle files for menu and components
    let menu_turtle = fs::read_to_string("../ontologies/zos/components.ttl").expect("Failed to read menu Turtle");
    let menu_signal = new_rdf_signal_from_turtle(&menu_turtle);
    // For demonstration, use the same file for both menu and components; split as needed
    let component_signal = new_rdf_signal_from_turtle(&menu_turtle);

    // Query menu items and components from the RDF signals
    let menu_items = query_triples(&menu_signal, None, Some("http://example.org/emoji#label"), None);
    let components = query_triples(&component_signal, None, Some("http://example.org/emoji#label"), None);

    rsx! {
        div {
            h2 { "Playground Menu (from Turtle)" }
            ul {
                for (s, _, label) in menu_items.iter() {
                    li { "Menu: {label}" }
                }
            }
            h2 { "Component List (from Turtle)" }
            ul {
                for (s, _, label) in components.iter() {
                    li { "Component: {label}" }
                }
            }
            // ... rest of the app ...
        }
    }
}
