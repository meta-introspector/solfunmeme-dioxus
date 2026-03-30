#[component]
fn GitParser2() -> Element {
    let repo_path = use_signal(|| String::new());
    let file_path = use_signal(|| String::new());
    let commit_id = use_signal(|| String::new());
    let result = use_signal(|| String::new());
    let error = use_signal(|| String::new());

    let on_parse = move |_| {
        spawn(async move {
            match GitParser::new(&repo_path()) {
                Ok(parser) => {
                    match parser.get_file_at_commit(&commit_id(), &file_path()) {
                        Ok(content) => {
                            match parser.parse_rust_file(&content) {
                                Ok(json) => {
                                    result.set(format!("Parsed JSON:\n{}", serde_json::to_string_pretty(&json).unwrap()));
                                    error.set(String::new());
                                },
                                Err(e) => error.set(format!("Parse error: {}", e)),
                            }
                        },
                        Err(e) => error.set(format!("Git error: {}", e)),
                    }
                },
                Err(e) => error.set(format!("Repository error: {}", e)),
            }
        });
    };

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Git Parser" }
            
            div { class: "mb-4",
                label { class: "block mb-2", "Repository Path:" }
                input {
                    class: "w-full p-2 border rounded",
                    value: "{repo_path}",
                    oninput: move |e| repo_path.set(e.value().to_string())
                }
            }
            
            div { class: "mb-4",
                label { class: "block mb-2", "File Path:" }
                input {
                    class: "w-full p-2 border rounded",
                    value: "{file_path}",
                    oninput: move |e| file_path.set(e.value().to_string())
                }
            }
            
            div { class: "mb-4",
                label { class: "block mb-2", "Commit ID:" }
                input {
                    class: "w-full p-2 border rounded",
                    value: "{commit_id}",
                    oninput: move |e| commit_id.set(e.value().to_string())
                }
            }
            
            button {
                class: "bg-blue-500 text-white px-4 py-2 rounded",
                onclick: on_parse,
                "Parse File"
            }
            
            if !error().is_empty() {
                div { class: "mt-4 p-4 bg-red-100 text-red-700 rounded",
                    "{error}"
                }
            }
            
            if !result().is_empty() {
                div { class: "mt-4 p-4 bg-gray-100 rounded",
                    pre { class: "whitespace-pre-wrap",
                        "{result}"
                    }
                }
            }
        }
    }
}
