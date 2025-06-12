
// Cargo.toml
/*
[package]
name = "quine-meme-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = "0.4"
dioxus-web = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
nalgebra = "0.32"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
*/

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use std::fmt;
use uuid::Uuid;
use nalgebra::DVector;

#[derive(Clone, Copy, PartialEq)]
pub struct Expression {
    //expressions: Vec<Expression>,
    // lam 
    // bind
    // app
    astring: Signal<String>
}   


#[derive(Clone, Copy, PartialEq)]
pub struct ExpressionListObj {
    expressions: Signal<Vec<Expression>>,
}   

#[derive(Clone, PartialEq)]
pub struct ExpressionList {
    expressions: Vec<Expression>,
}   

#[component]
fn ExpressionList2() -> Element {
    let state = use_signal(AppState::default);

    let expression_ids = if state.read().search_query.is_empty() {
        state.read().expressions.keys().cloned().collect::<Vec<_>>()
    } else {
        state.read().filtered_expressions.clone()
    };

    rsx! {
        section {
            class: "expression-list",
            h2 { "Lifted Expressions" }
            div {
                class: "cards-container",
                for id in expression_ids {
                    if let Some(expr) = state.read().expressions.get(&id) {
                        ExpressionCard { expression: expr.clone() }
                    }
                }
            }
        }
    }
}

// Remove UseState2 and use Dioxus's built-in UseState<AppState> instead.

// ============================================================================
// MONADS - Functional Programming Core
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Maybe<T> {
    value: Option<T>,
}

impl<T> Maybe<T> {
    pub fn some(value: T) -> Self {
        Maybe { value: Some(value) }
    }
    
    pub fn none() -> Self {
        Maybe { value: None }
    }
    
    pub fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self.value {
            Some(v) => Maybe::some(f(v)),
            None => Maybe::none(),
        }
    }
    
    pub fn flat_map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self.value {
            Some(v) => f(v),
            None => Maybe::none(),
        }
    }
    
    pub fn unwrap_or(self, default: T) -> T {
        self.value.unwrap_or(default)
    }
    
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }
}

//#[derive(Debug)]
// pub struct IO<T> {
//     action: Box<dyn Fn() -> T + 'static>(),
// }

// impl<T> IO<T> {
//     pub fn pure(value: T) -> IO<T> 
//     where 
//         T: Clone + 'static,
//     {
//         IO {
//             action: Box::new(move || value.clone())
//         }
//     }
    
//     pub fn map<U, F>(self, f: F) -> IO<U>
//     where
//         F: Fn(T) -> U + 'static,
//         T: 'static,
//         U: 'static,
//     {
//         IO {
//             action: Box::new(move || f((self.action)())),
//         }
//     }
    
//     pub fn run(self) -> T {
//         (self.action)()
//     }
// }
// #[derive(Debug, Clone)]
// pub struct IO<T> {
//     action: fn() -> T,
// }

// impl<T> IO<T> {
//     pub fn pure(value: T) -> IO<T> 
//     where 
//         T: Clone + 'static,
//     {
//         IO {
//             action: move || value.clone(),
//         }
//     }
    
//     pub fn map<U, F>(self, f: F) -> IO<U>
//     where
//         F: Fn(T) -> U + 'static,
//         T: 'static,
//         U: 'static,
//     {
//         IO {
//             action: || f((self.action)()),
//         }
//     }
    
//     pub fn run(self) -> T {
//         (self.action)()
//     }
// }

// ============================================================================
// MODEL - Data Layer
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quine {
    pub id: String,
    pub expression: String,
    pub self_reference: String,
    pub complexity_score: f64,
}

impl Quine {
    pub fn new(expression: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let self_reference = format!("fn quine() -> String {{ \"{}\" }}", expression);
        let complexity_score = expression.len() as f64 * 0.1;
        
        Quine {
            id,
            expression,
            self_reference,
            complexity_score,
        }
    }
    
    pub fn vectorize(&self) -> DVector<f64> {
        let chars: Vec<f64> = self.expression
            .chars()
            .map(|c| c as u8 as f64)
            .collect();
        
        let mut vector = vec![0.0; 256]; // ASCII vector space
        for &char_val in &chars {
            if char_val < 256.0 {
                vector[char_val as usize] += 1.0;
            }
        }
        
        vector.push(self.complexity_score);
        vector.push(chars.len() as f64);
        
        DVector::from_vec(vector)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meme {
    pub id: String,
    pub content: String,
    pub virality_score: f64,
    pub semantic_tags: Vec<String>,
    pub propagation_count: u32,
}

impl Meme {
    pub fn new(content: String, semantic_tags: Vec<String>) -> Self {
        let id = Uuid::new_v4().to_string();
        let virality_score = content.len() as f64 * semantic_tags.len() as f64 * 0.01;
        
        Meme {
            id,
            content,
            virality_score,
            semantic_tags,
            propagation_count: 0,
        }
    }
    
    pub fn propagate(&mut self) {
        self.propagation_count += 1;
        self.virality_score *= 1.1;
    }
    
    pub fn vectorize(&self) -> DVector<f64> {
        let content_chars: Vec<f64> = self.content
            .chars()
            .map(|c| c as u8 as f64)
            .collect();
        
        let mut vector = vec![0.0; 256]; // ASCII vector space
        for &char_val in &content_chars {
            if char_val < 256.0 {
                vector[char_val as usize] += 1.0;
            }
        }
        
        vector.push(self.virality_score);
        vector.push(self.propagation_count as f64);
        vector.push(self.semantic_tags.len() as f64);
        
        DVector::from_vec(vector)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiftedExpression {
    pub id: String,
    pub quine: Option<Quine>,
    pub meme: Option<Meme>,
    pub lifted_at: String,
    pub vector_representation: Vec<f64>,
}

impl LiftedExpression {
    pub fn from_quine(quine: Quine) -> Self {
        let vector_representation = quine.vectorize().data.as_vec().clone();
        
        LiftedExpression {
            id: Uuid::new_v4().to_string(),
            quine: Some(quine),
            meme: None,
            lifted_at: chrono::Utc::now().to_rfc3339(),
            vector_representation,
        }
    }
    
    pub fn from_meme(meme: Meme) -> Self {
        let vector_representation = meme.vectorize().data.as_vec().clone();
        
        LiftedExpression {
            id: Uuid::new_v4().to_string(),
            quine: None,
            meme: Some(meme),
            lifted_at: chrono::Utc::now().to_rfc3339(),
            vector_representation,
    }
}
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub expressions: HashMap<String, LiftedExpression>,
    pub current_input: String,
    pub current_tags: String,
    pub expression_type: ExpressionType,
    pub search_query: String,
    pub filtered_expressions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Quine,
    Meme,
}

impl Default for ExpressionType {
    fn default() -> Self {
        ExpressionType::Meme
    }
}

// ============================================================================
// CONTROLLER - Business Logic Layer
// ============================================================================

pub struct Controller;

impl Controller {
    pub fn add_quine_expression(
        state: &mut AppState,
        expression: String,
    ) -> Maybe<LiftedExpression> {
        if expression.trim().is_empty() {
            return Maybe::none();
        }
        
        let quine = Quine::new(expression);
        let lifted = LiftedExpression::from_quine(quine);
        let id = lifted.id.clone();
        
        state.expressions.insert(id.clone(), lifted.clone());
        state.current_input.clear();
        
        Maybe::some(lifted)
    }
    
    pub fn add_meme_expression(
        state: &mut AppState,
        content: String,
        tags: Vec<String>,
    ) -> Maybe<LiftedExpression> {
        if content.trim().is_empty() {
            return Maybe::none();
        }
        
        let meme = Meme::new(content, tags);
        let lifted = LiftedExpression::from_meme(meme);
        let id = lifted.id.clone();
        
        state.expressions.insert(id.clone(), lifted.clone());
        state.current_input.clear();
        state.current_tags.clear();
        
        Maybe::some(lifted)
    }
    
    pub fn search_expressions(
        state: &mut AppState,
        query: String,
    ) -> Vec<String> {
        let query_lower = query.to_lowercase();
        
        let filtered: Vec<String> = state
            .expressions
            .iter()
            .filter(|(_, expr)| {
                if let Some(ref quine) = expr.quine {
                    quine.expression.to_lowercase().contains(&query_lower)
                } else if let Some(ref meme) = expr.meme {
                    meme.content.to_lowercase().contains(&query_lower) ||
                    meme.semantic_tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
                } else {
                    false
                }
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        state.filtered_expressions = filtered.clone();
        filtered
    }
    
    pub fn propagate_meme(state: &mut AppState, expression_id: String) -> Maybe<()> {
        if let Some(expr) = state.expressions.get_mut(&expression_id) {
            if let Some(ref mut meme) = expr.meme {
                meme.propagate();
                expr.vector_representation = meme.vectorize().data.as_vec().clone();
                return Maybe::some(());
            }
        }
        Maybe::none()
    }
    
    pub fn delete_expression(state: &mut AppState, expression_id: String) -> Maybe<()> {
        match state.expressions.remove(&expression_id) {
            Some(_) => Maybe::some(()),
            None => Maybe::none(),
        }
    }
    
    pub fn get_vector_similarity(expr1: &LiftedExpression, expr2: &LiftedExpression) -> f64 {
        let v1 = DVector::from_vec(expr1.vector_representation.clone());
        let v2 = DVector::from_vec(expr2.vector_representation.clone());
        
        let dot_product = v1.dot(&v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot_product / (norm1 * norm2)
        }
    }
}
            //ExpressionList()
// ============================================================================

// fn main() {
//     console_error_panic_hook::set_once();
//     dioxus_web::launch(App);
// }
// fn use_signal2<T: 'static + Default>() -> UseState<T> {
//         use_signal(T::default)
//     }    

#[component]
fn App() -> Element {
    //let state = use_signal(cx, AppState::default);
    //let state = use_signal2(AppState::default);
    // Custom use_signal2 hook for demonstration (returns the same as use_signal)
    
    rsx! {
        div {
            class: "app-container",
            Header {}
            InputSection {
               // state: state,
            }
            ExpressionList2 {
                //state: state,
            }
            VectorSpace {
                //state: state,
            }
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            class: "app-header",
            h1 { 
                class: "title",
                "🧠 Lifted Language Expression Manager" 
            }
            p { 
                class: "subtitle",
                "Manage Quines & Memes in Vector Space" 
            }
        }
    }
}

//#[derive(Props)]
//struct InputSectionProps {
    //state: UseState<AppState>,
//}
//cx: Scope<InputSectionProps>
#[component]
fn InputSection() -> Element {
    //let state = &cx.props.state;
    let mut state = use_signal(AppState::default);

    rsx! {
        section {
            class: "input-section",
            div {
                class: "type-selector",
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().expression_type == ExpressionType::Quine,
                        onchange: move |_| {
                            state.with_mut(|s| s.expression_type = ExpressionType::Quine);
                        },
                    }
                    "Quine"
                }
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().expression_type == ExpressionType::Meme,
                        onchange: move |_| {
                            state.with_mut(|s| s.expression_type = ExpressionType::Meme);
                        },
                    }
                    "Meme"
                }
            }
            
            div {
                class: "input-controls",
                textarea {
                    class: "expression-input",
                    placeholder: match state.read().expression_type {
                        ExpressionType::Quine => "Enter quine expression...",
                        ExpressionType::Meme => "Enter meme content...",
                    },
                    value: state.read().current_input.clone(),
                    oninput: move |evt| {
                        state.with_mut(|s| s.current_input = evt.value().clone());
                    },
                }
                
                { 
                    if state.read().expression_type == ExpressionType::Meme {
                        
                    }
                }
            }
        }
    }
}

//pub struct Scope {}

fn InputSection2() -> Element {
    let mut  state = use_signal(AppState::default);

    rsx! {
        section {
            class: "input-section",
            div {
                class: "type-selector",
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().expression_type == ExpressionType::Quine,
                        onchange: move |_| {
                            state.write().expression_type = ExpressionType::Quine;
                        },
                    }
                    "Quine"
                }
                label {
                    input {
                        r#type: "radio",
                        name: "expression_type",
                        checked: state.read().expression_type == ExpressionType::Meme,
                        onchange: move |_| {
                            state.write().expression_type = ExpressionType::Meme;
                        },
                    }
                    "Meme"
                }
            }

            div {
                class: "input-controls",
                textarea {
                    class: "expression-input",
                    placeholder: match state.read().expression_type {
                        ExpressionType::Quine => "Enter quine expression...",
                        ExpressionType::Meme => "Enter meme content...",
                    },
                    value: state.read().current_input.clone(),
                    oninput: move |evt| {
                        state.write().current_input = evt.value().to_string();
                    },
                }

                {
                    if state.read().expression_type == ExpressionType::Meme {
                        Some(rsx! {
                            input {
                                class: "tags-input",
                                placeholder: "Semantic tags (comma-separated)...",
                                value: state.read().current_tags.clone(),
                                oninput: move |evt| {
                                    state.write().current_tags = evt.value().to_string();
                                },
                            }
                        })
                    } else {
                        None
                    }
                }

                button {
                    class: "add-button",
                    onclick: move |_| {
                        let expression_type = state.read().expression_type.clone();
                        let current_input = state.read().current_input.clone();
                        let current_tags = state.read().current_tags.clone();

                        let result = match expression_type {
                            ExpressionType::Quine => {
                                Controller::add_quine_expression(
                                    &mut state.write(),
                                    current_input
                                )
                            },
                            ExpressionType::Meme => {
                                let tags: Vec<String> = current_tags
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.to_string())
                                    .collect();

                                Controller::add_meme_expression(
                                    &mut state.write(),
                                    current_input,
                                    tags
                                )
                            }
                        };

                        if result.is_some() {
                            // Expression added successfully
                        }
                    },
                    "Lift Expression"
                }
            }

            div {
                class: "search-section",
                input {
                    class: "search-input",
                    placeholder: "Search expressions...",
                    value: state.read().search_query.clone(),
                    oninput: move |evt| {
                        let query = evt.value().to_string();
                        state.write().search_query = query.clone();
                        Controller::search_expressions(&mut state.write(), query);
                    },
                }
            }
        }
    }
}

// Add this component for rendering an expression card
#[derive(Props, PartialEq, Clone)]
struct ExpressionCardProps {
    expression: LiftedExpression,
}

#[component]
fn ExpressionCard(props: ExpressionCardProps) -> Element {
    let expr = props.expression.clone();
    let mut state = use_signal(AppState::default);
    let expr_id = expr.id.clone();

    rsx! {
        div {
            class: "expression-card",

            div {
                class: "card-header",
                span {
                    class: "expression-type",
                    if expr.quine.is_some() { "🔄 QUINE" } else { "🎭 MEME" }
                }
                button {
                    class: "delete-button",
                    onclick: move |_| {
                        Controller::delete_expression(&mut state.write(), expr_id.clone());
                    },
                    "×"
                }
            }

            div {
                class: "card-content",
                {
                    if let Some(ref quine) = expr.quine {
                        Some(rsx! {
                            div {
                                p {
                                    class: "expression-text",
                                    strong { "Expression: " }
                                    "{quine.expression}"
                                }
                                p {
                                    class: "self-reference",
                                    strong { "Self-Reference: " }
                                    code { "{quine.self_reference}" }
                                }
                                p {
                                    class: "complexity",
                                    strong { "Complexity: " }
                                    "{quine.complexity_score:.2}"
                                }
                            }
                        })
                    } else if let Some(ref meme) = expr.meme {
                        Some(rsx! {
                            div {
                                p {
                                    class: "meme-content",
                                    strong { "Content: " }
                                    "{meme.content}"
                                }
                                p {
                                    class: "semantic-tags",
                                    strong { "Tags: " }
                                    "{meme.semantic_tags.join(\", \")}"
                                }
                                p {
                                    class: "virality",
                                    strong { "Virality: " }
                                    "{meme.virality_score:.2}"
                                }
                                p {
                                    class: "propagation",
                                    strong { "Propagations: " }
                                    // Move expr_id binding outside rsx! and use it here
                                    button {
                                        class: "propagate-button",
                                        onclick: {
                                            let expr_id = expr.id.clone();
                                            move |_| {
                                                Controller::propagate_meme(&mut state.write(), expr_id.clone());
                                            }
                                        },
                                        "🚀 Propagate"
                                    }
                                }
                            }
                        })
                    } else {
                        None
                    }
                }
            }

            div {
                class: "vector-info",
                p {
                    strong { "Vector Dimensions: " }
                    "{expr.vector_representation.len()}"
                }
                p {
                    strong { "Lifted: " }
                    "{expr.lifted_at}"
                }
            }
        }
    }
}

// #[derive(Props)]
// struct VectorSpaceProps<'a> {
//     state: &'a UseState<AppState>,
// }

#[component]
fn VectorSpace() -> Element {
    let state = use_signal(AppState::default);
    
    let avg_vector_dim = if state.read().expressions.is_empty() {
        "0".to_string()
    } else {
        let avg = state.read().expressions.values()
            .map(|e| e.vector_representation.len())
            .sum::<usize>() as f64 / state.read().expressions.len() as f64;
        format!("{:.0}", avg)
    };

    rsx! {
        section {
            class: "vector-space",
            h2 { "Vector Space Analysis" }
            
            div {
                class: "stats-grid",
                div {
                    class: "stat-card",
                    h3 { "Total Expressions" }
                    p { class: "stat-value", "{state.read().expressions.len()}" }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Quines" }
                    p { 
                        class: "stat-value",
                        "{state.read().expressions.values().filter(|e| e.quine.is_some()).count()}"
                    }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Memes" }
                    p { 
                        class: "stat-value",
                        "{state.read().expressions.values().filter(|e| e.meme.is_some()).count()}"
                    }
                }
                
                div {
                    class: "stat-card",
                    h3 { "Avg Vector Dim" }
                    p { 
                        class: "stat-value",
                        "{avg_vector_dim}"
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            class: "app-footer",
            p { "Functional Reactive Architecture • Rust + Dioxus • MVC Pattern" }
        }
    }
}
