use crate::model::lean::style::Styles;
use dioxus::prelude::*;

// #[component]
// pub fn MemeManagement() -> Element {
//     rsx! {
//         div {
//             class: "{Styles::section()}",
//             h2 { class: "{Styles::h2()}", "Meme Management" }
//             p { class: "{Styles::p()}", "This section demonstrates meme management functionality." }
//         }
//     }
// }

#[derive(Clone, PartialEq)]
pub enum MemeCategory {
    ComponentMemes,
    WorkflowMemes,
    WikidataMemes,
    CryptoMemes,
    LeanMemes,
    FunMemes,
}

#[derive(Clone, PartialEq)]
pub struct Meme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: MemeCategory,
    pub emoji: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[component]
pub fn MemeManagement() -> Element {
    let mut selected_category = use_signal(|| MemeCategory::ComponentMemes);
    let mut selected_meme = use_signal(|| None::<Meme>);
    let mut show_meme_details = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());

    let memes = get_memes();

    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::h2()}", "🎭 Meme Management Toolbox" }
            p { class: "{Styles::p()}", "Explore and manage different types of memes for your SolFunMeme application." }

            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6 mt-6",
                // Meme Categories Sidebar
                div { class: "lg:col-span-1",
                    div { class: "bg-white dark:bg-gray-800 shadow-lg rounded-lg p-4",
                        h3 { class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white", "Meme Categories" }

                        // Search Bar
                        div { class: "mb-4",
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white",
                                placeholder: "Search memes...",
                                value: "{search_query()}",
                                oninput: move |e| search_query.set(e.value()),
                            }
                        }

                        for category in [
                            MemeCategory::ComponentMemes,
                            MemeCategory::WorkflowMemes,
                            MemeCategory::WikidataMemes,
                            MemeCategory::CryptoMemes,
                            MemeCategory::LeanMemes,
                            MemeCategory::FunMemes,
                        ] {
                            button {
                                class: format!(
                                    "w-full text-left p-3 mb-2 rounded-lg transition-colors flex items-center gap-2 {}",
                                    if selected_category() == category {
                                        "bg-blue-500 text-white"
                                    } else {
                                        "bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-900 dark:text-white"
                                    }
                                ),
                                onclick: move |_| selected_category.set(category.clone()),
                                span { class: "text-lg", "{category_emoji(&category)}" }
                                span { "{category_name(&category)}" }
                            }
                        }
                    }
                }

                // Meme Toolbox Panel
                div { class: "lg:col-span-3",
                    div { class: "bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6",
                        div { class: "flex justify-between items-center mb-4",
                            h3 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                                "{category_emoji(&selected_category())} {category_name(&selected_category())}"
                            }
                            div { class: "flex gap-2",
                                button {
                                    class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600 transition-colors",
                                    onclick: move |_| {
                                        // Add new meme functionality
                                    },
                                    "➕ Add Meme"
                                }
                                button {
                                    class: "bg-purple-500 text-white px-4 py-2 rounded-lg hover:bg-purple-600 transition-colors",
                                    onclick: move |_| {
                                        // Import memes functionality
                                    },
                                    "📥 Import"
                                }
                            }
                        }

                        // Meme Grid
                        div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4",
                            for meme in filter_memes(&memes, &selected_category(), &search_query()) {
                                MemeCard {
                                    meme: meme.clone(),
                                    on_select: move |selected_meme_data| {
                                        selected_meme.set(Some(selected_meme_data));
                                        show_meme_details.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Meme Details Modal
            if show_meme_details() {
                if let Some(meme) = selected_meme() {
                    MemeDetailsModal {
                        meme: meme,
                        on_close: move |_| {
                            show_meme_details.set(false);
                            selected_meme.set(None);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MemeCard(meme: Meme, on_select: EventHandler<Meme>) -> Element {
    let meme1 = meme.clone();
    let meme2 = meme.clone();

    rsx! {
        div {
            class: "border border-gray-200 dark:border-gray-600 rounded-lg p-4 hover:shadow-md transition-shadow cursor-pointer bg-white dark:bg-gray-700",
            onclick: move |_| on_select.call(meme1.clone()),

            div { class: "flex items-center gap-3 mb-3",
                span { class: "text-2xl", "{meme.emoji}" }
                div {
                    h4 { class: "font-medium text-gray-900 dark:text-white", "{meme.name}" }
                    p { class: "text-sm text-gray-600 dark:text-gray-300", "{meme.description}" }
                }
            }

            div { class: "flex flex-wrap gap-1 mb-3",
                for tag in meme.tags.iter().take(3) {
                    span {
                        class: "px-2 py-1 bg-gray-100 dark:bg-gray-600 text-xs rounded-full text-gray-700 dark:text-gray-300",
                        "{tag}"
                    }
                }
            }

            div { class: "flex justify-between items-center",
                button {
                    class: "text-blue-500 hover:text-blue-700 text-sm font-medium",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_select.call(meme2.clone());
                    },
                    "View Details"
                }
                button {
                    class: "text-green-500 hover:text-green-700 text-sm font-medium",
                    onclick: move |e| {
                        e.stop_propagation();
                        // Use meme functionality
                    },
                    "Use Meme"
                }
            }
        }
    }
}

#[component]
fn MemeDetailsModal(meme: Meme, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex justify-between items-start mb-4",
                    div { class: "flex items-center gap-3",
                        span { class: "text-3xl", "{meme.emoji}" }
                        div {
                            h3 { class: "text-xl font-semibold text-gray-900 dark:text-white", "{meme.name}" }
                            p { class: "text-sm text-gray-600 dark:text-gray-300", "{category_name(&meme.category)}" }
                        }
                    }
                    button {
                        class: "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                div { class: "mb-4",
                    h4 { class: "font-medium text-gray-900 dark:text-white mb-2", "Description" }
                    p { class: "text-gray-700 dark:text-gray-300", "{meme.description}" }
                }

                div { class: "mb-4",
                    h4 { class: "font-medium text-gray-900 dark:text-white mb-2", "Content" }
                    div { class: "bg-gray-100 dark:bg-gray-700 p-3 rounded-lg",
                        pre { class: "text-sm text-gray-800 dark:text-gray-200 whitespace-pre-wrap", "{meme.content}" }
                    }
                }

                div { class: "mb-6",
                    h4 { class: "font-medium text-gray-900 dark:text-white mb-2", "Tags" }
                    div { class: "flex flex-wrap gap-2",
                        for tag in meme.tags.iter() {
                            span {
                                class: "px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-sm rounded-full",
                                "{tag}"
                            }
                        }
                    }
                }

                div { class: "flex gap-3",
                    button {
                        class: "bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 transition-colors",
                        onclick: move |_| {
                            // Copy to clipboard
                        },
                        "📋 Copy"
                    }
                    button {
                        class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600 transition-colors",
                        onclick: move |_| {
                            // Use meme
                        },
                        "✨ Use Meme"
                    }
                    button {
                        class: "bg-purple-500 text-white px-4 py-2 rounded-lg hover:bg-purple-600 transition-colors",
                        onclick: move |_| {
                            // Edit meme
                        },
                        "✏️ Edit"
                    }
                }
            }
        }
    }
}

fn category_name(category: &MemeCategory) -> &'static str {
    match category {
        MemeCategory::ComponentMemes => "Component Memes",
        MemeCategory::WorkflowMemes => "Workflow Memes",
        MemeCategory::WikidataMemes => "Wikidata Memes",
        MemeCategory::CryptoMemes => "Crypto Memes",
        MemeCategory::LeanMemes => "Lean Memes",
        MemeCategory::FunMemes => "Fun Memes",
    }
}

fn category_emoji(category: &MemeCategory) -> &'static str {
    match category {
        MemeCategory::ComponentMemes => "🧩",
        MemeCategory::WorkflowMemes => "⚡",
        MemeCategory::WikidataMemes => "📚",
        MemeCategory::CryptoMemes => "🚀",
        MemeCategory::LeanMemes => "🎯",
        MemeCategory::FunMemes => "🎉",
    }
}

fn filter_memes(memes: &[Meme], category: &MemeCategory, search_query: &str) -> Vec<Meme> {
    memes
        .iter()
        .filter(|meme| meme.category == *category)
        .filter(|meme| {
            if search_query.is_empty() {
                true
            } else {
                let query = search_query.to_lowercase();
                meme.name.to_lowercase().contains(&query)
                    || meme.description.to_lowercase().contains(&query)
                    || meme
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query))
            }
        })
        .cloned()
        .collect()
}

fn get_memes() -> Vec<Meme> {
    vec![
        // Component Memes
        Meme {
            id: "comp_001".to_string(),
            name: "Button Bonanza".to_string(),
            description: "A collection of animated button components".to_string(),
            category: MemeCategory::ComponentMemes,
            emoji: "🎭".to_string(),
            content: "rsx! { button { class: \"animate-bounce\", \"Click me!\" } }".to_string(),
            tags: vec![
                "button".to_string(),
                "animation".to_string(),
                "interactive".to_string(),
            ],
        },
        Meme {
            id: "comp_002".to_string(),
            name: "Card Carousel".to_string(),
            description: "Rotating card components with smooth transitions".to_string(),
            category: MemeCategory::ComponentMemes,
            emoji: "🎠".to_string(),
            content: "rsx! { div { class: \"transform rotate-3d\", \"Card content\" } }"
                .to_string(),
            tags: vec![
                "card".to_string(),
                "carousel".to_string(),
                "rotation".to_string(),
            ],
        },
        // Workflow Memes
        Meme {
            id: "work_001".to_string(),
            name: "State Machine Meme".to_string(),
            description: "Visual representation of state transitions".to_string(),
            category: MemeCategory::WorkflowMemes,
            emoji: "⚡".to_string(),
            content: "State: Loading -> Success -> Error -> Retry".to_string(),
            tags: vec![
                "state".to_string(),
                "workflow".to_string(),
                "transitions".to_string(),
            ],
        },
        Meme {
            id: "work_002".to_string(),
            name: "Pipeline Flow".to_string(),
            description: "Data processing pipeline visualization".to_string(),
            category: MemeCategory::WorkflowMemes,
            emoji: "🔄".to_string(),
            content: "Input -> Process -> Transform -> Output".to_string(),
            tags: vec![
                "pipeline".to_string(),
                "data".to_string(),
                "processing".to_string(),
            ],
        },
        // Wikidata Memes
        Meme {
            id: "wiki_001".to_string(),
            name: "Knowledge Graph".to_string(),
            description: "Connected knowledge representation".to_string(),
            category: MemeCategory::WikidataMemes,
            emoji: "🕸️".to_string(),
            content: "Entity -> Property -> Value -> Reference".to_string(),
            tags: vec![
                "knowledge".to_string(),
                "graph".to_string(),
                "entities".to_string(),
            ],
        },
        Meme {
            id: "wiki_002".to_string(),
            name: "Semantic Web".to_string(),
            description: "Linked data relationships".to_string(),
            category: MemeCategory::WikidataMemes,
            emoji: "🌐".to_string(),
            content: "Subject -> Predicate -> Object".to_string(),
            tags: vec![
                "semantic".to_string(),
                "linked-data".to_string(),
                "rdf".to_string(),
            ],
        },
        // Crypto Memes
        Meme {
            id: "crypto_001".to_string(),
            name: "To The Moon".to_string(),
            description: "Classic crypto enthusiasm meme".to_string(),
            category: MemeCategory::CryptoMemes,
            emoji: "🚀".to_string(),
            content: "SOL 🚀🌙 HODL 💎🙌".to_string(),
            tags: vec!["moon".to_string(), "hodl".to_string(), "solana".to_string()],
        },
        Meme {
            id: "crypto_002".to_string(),
            name: "Diamond Hands".to_string(),
            description: "Never selling, always holding".to_string(),
            category: MemeCategory::CryptoMemes,
            emoji: "💎".to_string(),
            content: "💎🙌 NEVER SELLING 💎🙌".to_string(),
            tags: vec![
                "diamond".to_string(),
                "hands".to_string(),
                "holding".to_string(),
            ],
        },
        // Lean Memes
        Meme {
            id: "lean_001".to_string(),
            name: "Proof by Contradiction".to_string(),
            description: "When the proof doesn't work out".to_string(),
            category: MemeCategory::LeanMemes,
            emoji: "🤔".to_string(),
            content: "assume ¬P → ⊥ → P (but at what cost?)".to_string(),
            tags: vec![
                "proof".to_string(),
                "contradiction".to_string(),
                "logic".to_string(),
            ],
        },
        Meme {
            id: "lean_002".to_string(),
            name: "Tactic Soup".to_string(),
            description: "When you throw every tactic at the goal".to_string(),
            category: MemeCategory::LeanMemes,
            emoji: "🍲".to_string(),
            content: "simp; ring; omega; tauto; sorry".to_string(),
            tags: vec![
                "tactics".to_string(),
                "automation".to_string(),
                "sorry".to_string(),
            ],
        },
        // Fun Memes
        Meme {
            id: "fun_001".to_string(),
            name: "This is Fine".to_string(),
            description: "Everything is totally under control".to_string(),
            category: MemeCategory::FunMemes,
            emoji: "🔥".to_string(),
            content: "🐕☕ \"This is fine\" 🔥🔥🔥".to_string(),
            tags: vec![
                "fine".to_string(),
                "chaos".to_string(),
                "coffee".to_string(),
            ],
        },
        Meme {
            id: "fun_002".to_string(),
            name: "Distracted Boyfriend".to_string(),
            description: "When new tech catches your eye".to_string(),
            category: MemeCategory::FunMemes,
            emoji: "👀".to_string(),
            content: "Old Framework 😠 Me 👨 New Shiny Framework 😍".to_string(),
            tags: vec![
                "distracted".to_string(),
                "technology".to_string(),
                "frameworks".to_string(),
            ],
        },
    ]
}
