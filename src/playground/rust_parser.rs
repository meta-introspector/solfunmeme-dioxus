use dioxus::prelude::*;
use std::collections::HashMap;
use syn;
use syn_serde::json;

// --- Data Structures ---

#[derive(PartialEq, Clone, Debug)]
pub struct AstNode {
    pub type_name: String,
    pub key: String,
    pub value: String,
    pub children: Vec<AstNode>,
    pub is_expanded: bool,
    pub is_editing: bool,
    pub edited_value: String,
}

fn json_to_ast_nodes(json_val: serde_json::Value) -> Vec<AstNode> {
    fn value_to_string_short(v: &serde_json::Value) -> String {
        match v {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => "".to_string(),
        }
    }

    let mut nodes = Vec::new();
    if let serde_json::Value::Object(obj) = json_val {
        for (key, mut value) in obj {
            let type_name = value["type"].as_str().unwrap_or("Unknown").to_string();
            let mut children = vec![];

            if let Some(content) = value.get_mut("content") {
                if content.is_object() || content.is_array() {
                    children = json_to_ast_nodes(content.take());
                }
            }

            nodes.push(AstNode {
                type_name,
                key,
                value: value_to_string_short(&value["content"]),
                children,
                is_expanded: false,
                is_editing: false,
                edited_value: "".to_string(),
            });
        }
    } else if let serde_json::Value::Array(arr) = json_val {
        for (i, mut item) in arr.into_iter().enumerate() {
            nodes.push(AstNode {
                type_name: item["type"].as_str().unwrap_or("ArrayItem").to_string(),
                key: i.to_string(),
                value: value_to_string_short(&item["content"]),
                children: json_to_ast_nodes(item["content"].take()),
                is_expanded: false,
                is_editing: false,
                edited_value: "".to_string(),
            });
        }
    }
    nodes
}

#[derive(PartialEq, Clone, Copy, Default)]
pub enum ViewMode {
    #[default]
    Json,
    Interactive,
    Settings,
}

#[derive(PartialEq, Clone)]
pub struct RustParserState {
    pub input_code: String,
    pub parsed_ast: Option<String>,
    pub ast_nodes: Vec<AstNode>,
    pub error_message: Option<String>,
    pub is_pretty: bool,
    pub view_mode: ViewMode,
    pub emoji_map: HashMap<String, String>,
}

impl Default for RustParserState {
    fn default() -> Self {
        Self {
            input_code: "".to_string(),
            parsed_ast: None,
            ast_nodes: vec![],
            error_message: None,
            is_pretty: false,
            view_mode: ViewMode::default(),
            emoji_map: HashMap::from([
                ("Ident".to_string(), "🏷️".to_string()),
                ("Path".to_string(), "🛤️".to_string()),
                ("Lit".to_string(), "💎".to_string()),
                ("Expr".to_string(), "🧮".to_string()),
                ("Stmt".to_string(), "💬".to_string()),
                ("Item".to_string(), "📦".to_string()),
                ("Type".to_string(), "📜".to_string()),
                ("Pat".to_string(), "🎭".to_string()),
                ("Arm".to_string(), "💪".to_string()),
                ("Block".to_string(), "🧱".to_string()),
                ("Field".to_string(), "🏷️".to_string()),
                ("Fn".to_string(), "🔧".to_string()),
                ("Struct".to_string(), "🏗️".to_string()),
                ("Enum".to_string(), "🎲".to_string()),
                ("Trait".to_string(), "✨".to_string()),
                ("Impl".to_string(), "🔌".to_string()),
                ("Use".to_string(), "📤".to_string()),
                ("Macro".to_string(), "🔮".to_string()),
                ("Attribute".to_string(), "🔖".to_string()),
                ("File".to_string(), "📄".to_string()),
            ]),
        }
    }
}

// --- Helper Functions ---

fn get_node_mut<'a>(nodes: &'a mut Vec<AstNode>, path: &[usize]) -> Option<&'a mut AstNode> {
    if path.is_empty() {
        return None;
    }
    let mut current_node_list = nodes;
    for &index in &path[..path.len() - 1] {
        if let Some(node) = current_node_list.get_mut(index) {
            current_node_list = &mut node.children;
        } else {
            return None;
        }
    }
    current_node_list.get_mut(*path.last().unwrap())
}

fn get_emoji_for_type<'a>(type_name: &str, emoji_map: &'a HashMap<String, String>) -> &'a str {
    emoji_map
        .iter()
        .find(|(key, _)| type_name.contains(*key))
        .map(|(_, emoji)| emoji.as_str())
        .unwrap_or("❓")
}

// --- Helper Functions for RustParserApp ---

fn parse_code(state: &mut RustParserState) {
    let code = state.input_code.clone();
    match syn::parse_file(&code) {
        Ok(file) => {
            let json_str_result = if state.is_pretty {
                json::to_string_pretty(&file)
            } else {
                json::to_string(&file)
            };
            let json_val: serde_json::Value =
                serde_json::from_str(&json_str_result).unwrap_or_default();
            state.parsed_ast = Some(json_val.to_string());
            state.ast_nodes = json_to_ast_nodes(json_val);
            state.error_message = None;
        }
        Err(e) => {
            state.parsed_ast = None;
            state.ast_nodes = vec![];
            state.error_message = Some(e.to_string());
        }
    }
}

fn clear_state(state: &mut RustParserState) {
    state.input_code.clear();
    state.parsed_ast = None;
    state.ast_nodes = vec![];
    state.error_message = None;
}

// --- Main Application Component ---

pub fn RustParserApp() -> Element {
    let mut state = use_signal(RustParserState::default);

    let handle_parse = {
        let mut state = state.clone();
        move || {
            let mut state_w = state.write();
            parse_code(&mut state_w);
        }
    };

    let handle_clear = move || {
        let mut state_w = state.write();
        clear_state(&mut state_w);
    };

    let handle_load_example = {
        let mut state = use_signal(RustParserState::default);
        let mut handle_parse = {
            let mut state = state.clone();
            move || {
                let mut state_w = state.write();
                let code = state_w.input_code.clone();
                match syn::parse_file(&code) {
                    Ok(file) => {
                        let json_str_result = if state_w.is_pretty {
                            json::to_string_pretty(&file)
                        } else {
                            json::to_string(&file)
                        };

                        let json_val: serde_json::Value =
                            serde_json::from_str(&json_str_result).unwrap_or_default();

                        state_w.parsed_ast = Some(json_val.to_string());
                        state_w.ast_nodes = json_to_ast_nodes(json_val);
                        state_w.error_message = None;
                    }
                    Err(e) => {
                        state_w.parsed_ast = None;
                        state_w.ast_nodes = vec![];
                        state_w.error_message = Some(e.to_string());
                    }
                }
            }
        };
        move |code: &'static str| {
            state.write().input_code = code.to_string();
            handle_parse();
        }
    };

    let on_input_change = {
        let mut state = use_signal(RustParserState::default);
        move |evt: Event<FormData>| {
            state.write().input_code = evt.value();
        }
    };

    let on_toggle_pretty = {
        let mut state = state.clone();
        let mut handle_parse = handle_parse.clone();
        move |_| {
            {
                let mut state_w = state.write();
                state_w.is_pretty = !state_w.is_pretty;
            }
            handle_parse();
        }
    };

    let mut on_toggle_view_mode = {
        let mut state = state.clone();
        move |new_mode: Option<ViewMode>| {
            let current_mode = state.read().view_mode;
            let next_mode = new_mode.unwrap_or(match current_mode {
                ViewMode::Json => ViewMode::Interactive,
                ViewMode::Interactive => ViewMode::Json,
                ViewMode::Settings => ViewMode::Json,
            });
            state.write().view_mode = next_mode;
        }
    };

    let on_toggle_expand = move |path: Vec<usize>| {
        if let Some(node) = get_node_mut(&mut state.write().ast_nodes, &path) {
            node.is_expanded = !node.is_expanded;
        }
    };

    let on_edit = move |(path, new_value): (Vec<usize>, String)| {
        if let Some(node) = get_node_mut(&mut state.write().ast_nodes, &path) {
            node.is_editing = !node.is_editing;
            node.edited_value = new_value;
        }
    };

    let on_save_edit = move |(path, value): (Vec<usize>, String)| {
        if let Some(node) = get_node_mut(&mut state.write().ast_nodes, &path) {
            node.is_editing = false;
            node.value = value;
        }
    };

    let on_emoji_map_change = move |(key, value): (String, String)| {
        state.write().emoji_map.insert(key, value);
    };

    rsx! {
        div { class: "p-4 bg-gray-900 text-white min-h-screen font-sans",
            RustParserHeader {}
            RustParserControls {
                on_parse: handle_parse,
                on_clear: handle_clear,
                on_toggle_pretty: on_toggle_pretty,
                on_toggle_view_mode: move |_| on_toggle_view_mode(None),
                is_pretty: state.read().is_pretty,
            }
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4",
                RustParserMainArea {
                    state: state.read().clone(),
                    on_input_change: on_input_change,
                    on_toggle_view_mode: move |_| on_toggle_view_mode(None),
                    on_toggle_expand: on_toggle_expand,
                    on_edit: on_edit,
                    on_save_edit: on_save_edit,
                    on_emoji_map_change: on_emoji_map_change,
                }
                RustParserExamples { on_load_example: handle_load_example }
            }
        }
    }
}

// --- UI Components ---

#[component]
fn RustParserHeader() -> Element {
    rsx! {
        div { class: "text-center mb-4",
            h1 { class: "text-4xl font-bold text-cyan-400", "Rust Code AST Explorer" }
            p { class: "text-lg text-gray-400",
                "Parse Rust code into an Abstract Syntax Tree using "
                a {
                    class: "text-cyan-400 hover:underline",
                    href: "https://github.com/dtolnay/syn",
                    target: "_blank",
                    "syn"
                }
                " and view it as JSON or an interactive tree."
            }
        }
    }
}

#[component]
fn RustParserControls(
    on_parse: EventHandler<()>,
    on_clear: EventHandler<()>,
    on_toggle_pretty: EventHandler<()>,
    on_toggle_view_mode: EventHandler<()>,
    is_pretty: bool,
) -> Element {
    rsx! {
        div { class: "flex justify-center items-center gap-4 p-2 bg-gray-800 rounded-lg",
            button {
                class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold",
                onclick: move |_| on_parse.call(()),
                "Parse"
            }
            button {
                class: "px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg font-semibold",
                onclick: move |_| on_clear.call(()),
                "Clear"
            }
            button {
                class: "px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg font-semibold",
                onclick: move |_| on_toggle_view_mode.call(()),
                "Settings"
            }
            label { class: "flex items-center gap-2 cursor-pointer",
                input {
                    r#type: "checkbox",
                    class: "form-checkbox h-5 w-5 text-cyan-400 bg-gray-700 border-gray-600 rounded focus:ring-cyan-500",
                    checked: is_pretty,
                    oninput: move |_| on_toggle_pretty.call(()),
                }
                "Pretty Print JSON"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct RustParserMainAreaProps {
    state: RustParserState,
    on_input_change: EventHandler<Event<FormData>>,
    on_toggle_view_mode: EventHandler<()>,
    on_toggle_expand: EventHandler<Vec<usize>>,
    on_edit: EventHandler<(Vec<usize>, String)>,
    on_save_edit: EventHandler<(Vec<usize>, String)>,
    on_emoji_map_change: EventHandler<(String, String)>,
}

#[component]
fn ToggleViewButton(view_mode: ViewMode, on_toggle: EventHandler<()>) -> Element {
    let label = match view_mode {
        ViewMode::Json => "Interactive",
        ViewMode::Interactive => "JSON",
        ViewMode::Settings => "Back",
    };
    rsx! {
        button {
            class: "px-3 py-1 bg-purple-600 hover:bg-purple-700 rounded-lg text-sm",
            onclick: move |_| on_toggle.call(()),
            "Toggle View ( { label} )"
        }
    }
}

#[component]
fn RustAstSwitcher(
    view_mode: ViewMode,
    parsed_ast: Option<String>,
    error_message: Option<String>,
    ast_nodes: Vec<AstNode>,
    emoji_map: HashMap<String, String>,
    on_toggle_expand: EventHandler<Vec<usize>>,
    on_edit: EventHandler<(Vec<usize>, String)>,
    on_save_edit: EventHandler<(Vec<usize>, String)>,
    on_emoji_map_change: EventHandler<(String, String)>,
) -> Element {
    match view_mode {
        ViewMode::Json => rsx! {
            RustAstOutput {
                parsed_ast: parsed_ast,
                error_message: error_message,
            }
        },
        ViewMode::Interactive => rsx! {
            // InteractiveAstViewer {
            //     ast_nodes: ast_nodes,
            //     emoji_map: emoji_map,
            //     on_toggle_expand: on_toggle_expand,
            //     on_edit: on_edit,
            //     on_save_edit: on_save_edit,
            // }
        },
        ViewMode::Settings => rsx! {
            EmojiSettings {
                emoji_map: emoji_map,
                on_change: on_emoji_map_change,
            }
        },
    }
}

#[component]
fn RustParserMainArea(props: RustParserMainAreaProps) -> Element {
    let state = props.state.clone();
    rsx! {
        div { class: "flex flex-col gap-4",
            RustCodeInput {
                input_code: state.input_code.clone(),
                on_input_change: props.on_input_change.clone(),
            }
            div { class: "p-4 bg-gray-800 rounded-lg",
                div { class: "flex justify-end mb-2",
                    ToggleViewButton {
                        view_mode: state.view_mode,
                        on_toggle: props.on_toggle_view_mode.clone(),
                    }
                }
                RustAstSwitcher {
                    view_mode: state.view_mode,
                    parsed_ast: state.parsed_ast.clone(),
                    error_message: state.error_message.clone(),
                    ast_nodes: state.ast_nodes.clone(),
                    emoji_map: state.emoji_map.clone(),
                    on_toggle_expand: props.on_toggle_expand.clone(),
                    on_edit: props.on_edit.clone(),
                    on_save_edit: props.on_save_edit.clone(),
                    on_emoji_map_change: props.on_emoji_map_change.clone(),
                }
            }
        }
    }
}

#[component]
fn RustCodeInput(input_code: String, on_input_change: EventHandler<Event<FormData>>) -> Element {
    rsx! {
        div { class: "bg-gray-800 p-4 rounded-lg",
            label { r#for: "code-input", class: "block text-lg font-semibold mb-2", "Rust Code Input" }
            textarea {
                id: "code-input",
                class: "w-full h-80 p-2 bg-gray-900 border border-gray-700 rounded-lg font-mono text-sm focus:ring-cyan-500 focus:border-cyan-500",
                value: "{input_code}",
                oninput: on_input_change,
                placeholder: "Enter your Rust code here...",
            }
        }
    }
}

#[component]
fn RustAstOutput(parsed_ast: Option<String>, error_message: Option<String>) -> Element {
    rsx! {
        div {
            if let Some(error) = error_message {
                div {
                    h3 { class: "text-lg font-semibold text-red-500 mb-2", "Parsing Error" }
                    pre { class: "p-2 bg-red-900 bg-opacity-50 text-red-300 border border-red-500 rounded-lg overflow-auto",
                        "{error}"
                    }
                }
            } else if let Some(ast) = parsed_ast {
                 div {
                    h3 { class: "text-lg font-semibold text-green-400 mb-2", "JSON AST Output" }
                    pre { class: "p-2 bg-green-900 bg-opacity-20 text-green-300 border border-green-500 rounded-lg overflow-auto",
                        "{ast}"
                    }
                }
            } else {
                 p { class: "text-gray-500", "Parse some code to see the AST." }
            }
        }
    }
}

// --- Example Snippets ---

const HELLO_WORLD_CODE: &str = r#"
fn main() {
    println!("Hello, world!");
}
"#;

const STRUCT_DEF_CODE: &str = r#"
struct Point {
    x: f64,
    y: f64,
}
"#;

const FIBONACCI_CODE: &str = r#"
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
"#;

const SELF_CODE: &str = include_str!("./rust_parser.rs");

#[component]
fn RustParserExamples(on_load_example: EventHandler<&'static str>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { class: "text-2xl font-semibold text-center text-cyan-400", "Or Try an Example" }
            ExampleCard { title: "Hello, World!", code: HELLO_WORLD_CODE, button_class: "bg-green-600 hover:bg-green-700", on_load: move |_| on_load_example.call(HELLO_WORLD_CODE) }
            ExampleCard { title: "Struct Definition", code: STRUCT_DEF_CODE, button_class: "bg-yellow-600 hover:bg-yellow-700", on_load: move |_| on_load_example.call(STRUCT_DEF_CODE) }
            ExampleCard { title: "Fibonacci Function", code: FIBONACCI_CODE, button_class: "bg-indigo-600 hover:bg-indigo-700", on_load: move |_| on_load_example.call(FIBONACCI_CODE) }
            ExampleCard { title: "Parse This App's Source!", code: "Click to load...", button_class: "bg-pink-600 hover:bg-pink-700", on_load: move |_| on_load_example.call(SELF_CODE) }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ExampleCardProps {
    title: &'static str,
    code: &'static str,
    button_class: &'static str,
    on_load: EventHandler<()>,
}

#[component]
fn ExampleCard(props: ExampleCardProps) -> Element {
    rsx! {
        div { class: "bg-gray-800 p-4 rounded-lg",
            h3 { class: "text-lg font-semibold text-gray-200", "{props.title}" }
            pre { class: "mt-2 p-2 bg-gray-900 text-sm text-gray-300 rounded-md overflow-auto max-h-40 font-mono",
                code { "{props.code}" }
            }
            button {
                class: "mt-2 px-4 py-2 w-full text-white font-semibold rounded-lg {props.button_class}",
                onclick: move |_| props.on_load.call(()),
                "Load Example"
            }
        }
    }
}

// --- Settings ---

#[derive(Props, PartialEq, Clone)]
struct EmojiSettingsProps {
    emoji_map: HashMap<String, String>,
    on_change: EventHandler<(String, String)>,
}

fn handle_emoji_input(
    key: String,
    on_change: EventHandler<(String, String)>,
) -> impl FnMut(Event<FormData>) {
    move |evt: Event<FormData>| on_change.call((key.clone(), evt.value()))
}

#[component]
fn EmojiSettings(props: EmojiSettingsProps) -> Element {
    rsx! {
        div { class: "p-4 bg-gray-800 text-white rounded-lg",
            h3 { class: "text-lg font-semibold text-cyan-400 mb-4", "Emoji Settings" }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                for (key, value) in props.emoji_map.iter() {
                    div { class: "flex items-center gap-2 p-2 bg-gray-700 rounded",
                        span { class: "font-semibold text-purple-300 w-24", "{key}" }
                        input {
                            class: "w-16 bg-gray-900 text-center rounded",
                            value: "{value}",
                            oninput: handle_emoji_input(key.clone(), props.on_change.clone()),
                        }
                    }
                }
            }
        }
    }
}
