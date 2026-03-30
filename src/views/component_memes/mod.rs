use dioxus::prelude::*;
//use std::collections::HashMap;
mod style;
use serde::{Deserialize, Serialize};
use style::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentMeme {
    pub name: String,
    pub emoji: String,
    pub lean_expression: String,
    pub description: String,
    pub matrix_representation: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemeCategory {
    pub name: String,
    pub emoji: String,
    pub components: Vec<ComponentMeme>,
}
#[allow(dead_code)]
pub fn get_crypto_component_memes() -> Vec<ComponentMeme> {
    vec![
        ComponentMeme {
            name: "CryptoFrontendApp".to_string(),
            emoji: "🔐".to_string(),
            lean_expression:
                "structure CryptoApp := (encryption: EncryptionForm) (decryption: DecryptionForm)"
                    .to_string(),
            description: "The main encryption application component".to_string(),
            matrix_representation: vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ],
        },
        ComponentMeme {
            name: "EncryptionForm".to_string(),
            emoji: "📝".to_string(),
            lean_expression:
                "structure EncryptionForm := (title: String) (message: String) (keys: KeyPair)"
                    .to_string(),
            description: "Form for encrypting messages".to_string(),
            matrix_representation: vec![
                vec![1.0, 1.0, 0.0],
                vec![1.0, 0.0, 1.0],
                vec![0.0, 1.0, 1.0],
            ],
        },
        ComponentMeme {
            name: "DecryptionForm".to_string(),
            emoji: "🔓".to_string(),
            lean_expression:
                "structure DecryptionForm := (payload: EncryptedPayload) (key: PrivateKey)"
                    .to_string(),
            description: "Form for decrypting messages".to_string(),
            matrix_representation: vec![
                vec![0.0, 1.0, 1.0],
                vec![1.0, 0.0, 1.0],
                vec![1.0, 1.0, 0.0],
            ],
        },
    ]
}
#[allow(dead_code)]
pub fn get_ui_component_memes() -> Vec<ComponentMeme> {
    vec![
        ComponentMeme {
            name: "CardHeader".to_string(),
            emoji: "📋".to_string(),
            lean_expression: "structure CardHeader := (title: String)".to_string(),
            description: "Header component for cards".to_string(),
            matrix_representation: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        },
        ComponentMeme {
            name: "InputField".to_string(),
            emoji: "✏️".to_string(),
            lean_expression:
                "structure InputField := (label: String) (value: String) (onInput: String → Unit)"
                    .to_string(),
            description: "Reusable input field component".to_string(),
            matrix_representation: vec![vec![1.0, 1.0], vec![1.0, 0.0]],
        },
        ComponentMeme {
            name: "ActionButton".to_string(),
            emoji: "🔘".to_string(),
            lean_expression: "structure ActionButton := (text: String) (onClick: Unit → Unit)"
                .to_string(),
            description: "Reusable button component".to_string(),
            matrix_representation: vec![vec![0.0, 1.0], vec![1.0, 0.0]],
        },
    ]
}

#[component]
pub fn ComponentMemeView(component_meme: ComponentMeme) -> Element {
    rsx! {
        div { class: COMPONENT_MEME,
            div { class: MEME_HEADER,
                span { class: MEME_EMOJI, "{component_meme.emoji}" }
                h3 { class: MEME_TITLE, "{component_meme.name}" }
            }
            p { class: MEME_DESCRIPTION, "{component_meme.description}" }
            pre { class: LEAN_CODE,
                "{component_meme.lean_expression}"
            }
            div { class: MATRIX,
                table { class: MATRIX_TABLE,
                    // component_meme.matrix_representation.iter().map(|row| rsx! {
                    //     tr {
                    //         row.iter().map(|&val| rsx! {
                    //             td { class: MATRIX_CELL, "{val}" }
                    //         })
                    //     }
                    // })
                }
            }

        }
    }
}

#[component]
pub fn MemeCategoryView(category: MemeCategory) -> Element {
    rsx! {
        div { class: MEME_CATEGORY,
            div { class: CATEGORY_HEADER,
                span { class: CATEGORY_EMOJI, "{category.emoji}" }
                h2 { class: CATEGORY_TITLE, "{category.name}" }
            }
            div { class: COMPONENTS_GRID,
                // category.components.iter().map(|component| rsx! {
                //     ComponentMemeView { component_meme: component.clone() }
                // })
            }
        }
    }
}

#[component]
pub fn ComponentMemeExplorer() -> Element {
    //let crypto_memes = get_crypto_component_memes();
    //let ui_memes = get_ui_component_memes();

    // let categories = vec![
    //     MemeCategory {
    //         name: "Crypto Components".to_string(),
    //         emoji: "🔐".to_string(),
    //         components: crypto_memes,
    //     },
    //     MemeCategory {
    //         name: "UI Components".to_string(),
    //         emoji: "🎨".to_string(),
    //         components: ui_memes,
    //     },
    // ];

    rsx! {
        div { class: MEME_EXPLORER,
            h1 { class: "text-3xl font-bold text-gray-900 mb-8", "Component Meme Explorer" }
            div { class: CATEGORIES,
                // categories.iter().map(|category| rsx! {
                //     MemeCategoryView { category: category.clone() }
                // })
            }
        }
    }
}
