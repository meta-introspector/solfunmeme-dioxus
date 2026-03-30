use dioxus::prelude::*;

#[component]
pub fn SourceBrowserStyle() -> Element {
    let style = r#"
            .source-browser {
                height: 100vh;
                display: flex;
                flex-direction: column;
            }
            .browser-header {
                padding: 1rem;
                border-bottom: 1px solid #ccc;
            }
            .browser-content {
                display: flex;
                flex: 1;
                overflow: hidden;
            }
            .sidebar {
                width: 300px;
                border-right: 1px solid #ccc;
                padding: 1rem;
                overflow-y: auto;
            }
            .module-list, .file-list {
                list-style: none;
                padding: 0;
                margin: 0;
            }
            .module-list li, .file-list li {
                padding: 0.5rem;
                cursor: pointer;
                border-radius: 4px;
                margin-bottom: 2px;
            }
            .module-list li:hover, .file-list li:hover {
                background-color: #f0f0f0;
            }
            .module-list li.selected, .file-list li.selected {
                background-color: #007acc;
                color: white;
            }
            .content-area {
                flex: 1;
                padding: 1rem;
                overflow: auto;
            }
            .code-content {
                background-color: #f8f8f8;
                padding: 1rem;
                border-radius: 4px;
                overflow: auto;
                white-space: pre-wrap;
                font-family: 'Courier New', monospace;
                font-size: 14px;
                line-height: 1.4;
            }
            .placeholder {
                display: flex;
                align-items: center;
                justify-content: center;
                height: 100%;
                color: #666;
            }
        "#;
    rsx! {
        style { style }
    }
}
