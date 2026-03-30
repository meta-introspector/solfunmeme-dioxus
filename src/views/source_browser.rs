//use crate::generated::*;
use crate::generated::src::*;
use crate::generated::src_bin::*;
use crate::generated::src_model::*;
use crate::generated::src_views::*;
use crate::generated::src_model_git::*;
use crate::generated::src_model_math::*;
use crate::generated::src_model_lean::*;
use crate::generated::src_model_lean_types::*;
use crate::generated::src_playground::*;
use crate::generated::src_state::*;
use crate::views::source_browser_style;
use dioxus::prelude::*;
use dioxus_logger::tracing::debug;

/*
The idea of this module is to embed enought of
the source code to read it to then to know what other source
code is there.
We ran into limits of compilation adding more sources in.
So we want to create zip files of these other parts to prepare for loading on demand,
these could be stored in different account on SOLFUNMEME.

we want next to take the source code, parse it to asks,
decorate it as emojis, as vectors and visualize it.

We can imagine ever enum giving a certain prime emoji and each enum value another.
 */
//use source_browser_style;

#[component]
pub fn SourceBrowser() -> Element {
//    crate::embedself::printall();
    let selected_module = use_signal(|| "src".to_string());
    let selected_file = use_signal(|| None::<String>);

    let modules = vec![
        ("src", "Main Source"),
        ("src/bin", "Binaries"),
        ("src/extractor", "Extractor"),
        ("src/extractor/components", "Extractor Components"),
        ("src/extractor/model", "Extractor Model"),
        ("src/extractor/system", "Extractor System"),
        ("src/model", "Model"),
        ("src/model/git", "Git Model"),
        ("src/model/lean", "Lean Model"),
        ("src/model/lean/types", "Lean Types"),
        ("src/model/math", "Math Model"),
        ("src/playground", "Playground"),
        ("src/state", "State"),
        ("src/views", "Views"),
        ("src/views/component_memes", "Component Memes"),
        ("src/views/crypto_frontend", "Crypto Frontend"),
        ("src/views/extras_views", "Extra Views"),
        ("src/views/wikidata_memes", "Wikidata Memes"),
        ("src/views/workflow_memes", "Workflow Memes"),
    ];

    let files = match selected_module().as_str() {
        "src" => OurSrcExtractor::iter().collect::<Vec<_>>(),
        "src/bin" => OurSrcBinExtractor::iter().collect::<Vec<_>>(),
        // "src/extractor" => OurSrcExtractorExtractor::iter().collect::<Vec<_>>(),
        // "src/extractor/components" => OurSrcExtractorComponentsExtractor::iter().collect::<Vec<_>>(),
        // "src/extractor/model" => OurSrcExtractorModelExtractor::iter().collect::<Vec<_>>(),
        // "src/extractor/system" => OurSrcExtractorSystemExtractor::iter().collect::<Vec<_>>(),
         "src/model" => OurSrcModelExtractor::iter().collect::<Vec<_>>(),
         "src/model/git" => OurSrcModelGitExtractor::iter().collect::<Vec<_>>(),
         "src/model/lean" => OurSrcModelLeanExtractor::iter().collect::<Vec<_>>(),
         "src/model/lean/types" => OurSrcModelLeanTypesExtractor::iter().collect::<Vec<_>>(),
         "src/model/math" => OurSrcModelMathExtractor::iter().collect::<Vec<_>>(),
        "src/playground" => OurSrcPlaygroundExtractor::iter().collect::<Vec<_>>(),
         "src/state" => OurSrcStateExtractor::iter().collect::<Vec<_>>(),
         "src/views" => OurSrcViewsExtractor::iter().collect::<Vec<_>>(),
        // "src/views/component_memes" => OurSrcViewComponentExtractor::iter().collect::<Vec<_>>(),
        // "src/views/crypto_frontend" => OurSrcViewCryptoExtractor::iter().collect::<Vec<_>>(),
        // "src/views/extras_views" => OurSrcViewextraExtractor::iter().collect::<Vec<_>>(),
        // "src/views/wikidata_memes" => OurSrcViewWikwidataExtractor::iter().collect::<Vec<_>>(),
        // "src/views/workflow_memes" => OurSrcViewWorkflowExtractor::iter().collect::<Vec<_>>(),
        _ => vec![],
    };
    debug!("SourceBrowser: files for module {}: {:?}", selected_module(), files);

    let file_content = selected_file().and_then(|filename| match selected_module().as_str() {
        "src" => OurSrcExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
         "src/bin" => OurSrcBinExtractor::get(&filename).map(|f| {
             std::str::from_utf8(&f.data)
                 .unwrap_or("Binary file")
                 .to_string()
         }),
        // "src/extractor" => OurSrcExtractor::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/extractor/components" => OurSrcExtractorComponents::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/extractor/model" => OurSrcExtractorModel::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/extractor/system" => OurSrcExtractorSystem::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        "src/model" => OurSrcModelExtractor::get(&filename).map(|f| {
             std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/git" => OurSrcModelGitExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/lean" => OurSrcModelLeanExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/lean/types" => OurSrcModelLeanTypesExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/model/math" => OurSrcModelMathExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/playground" => OurSrcPlaygroundExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
        "src/state" => OurSrcStateExtractor::get(&filename).map(|f| {
            std::str::from_utf8(&f.data)
                .unwrap_or("Binary file")
                .to_string()
        }),
         "src/views" => OurSrcViewsExtractor::get(&filename).map(|f| {
             std::str::from_utf8(&f.data)
                 .unwrap_or("Binary file")
                 .to_string()
         }),
        // "src/views/component_memes" => OurSrcViewComponent::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/views/crypto_frontend" => OurSrcViewCrypto::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/views/extras_views" => OurSrcViewextra::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/views/wikidata_memes" => OurSrcViewWikwidata::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        // "src/views/workflow_memes" => OurSrcViewWorkflow::get(&filename).map(|f| {
        //     std::str::from_utf8(&f.data)
        //         .unwrap_or("Binary file")
        //         .to_string()
        // }),
        _ => None,
    });

    rsx! {
        div { class: "source-browser",
            div { class: "browser-header",
                h2 { "Source Code Browser" }
            }
            div { class: "browser-content",
                ModuleSidebar { modules: modules.clone(), selected_module: selected_module.clone(), set_selected_module: selected_module.clone(), set_selected_file: selected_file.clone(), files: files.clone(), selected_file: selected_file.clone() }
                div { class: "content-area",
                    FileViewer { file_content: file_content.clone(), selected_file: selected_file.clone() }
                }
            }
        }
        source_browser_style::SourceBrowserStyle {}
    }
}

#[component]
fn ModuleSidebar(
    modules: Vec<(&'static str, &'static str)>,
    selected_module: Signal<String>,
    set_selected_module: Signal<String>,
    set_selected_file: Signal<Option<String>>,
    files: Vec<std::borrow::Cow<'static, str>>,
    selected_file: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "sidebar",
            h3 { "Modules" }
            ul { class: "module-list",
                for (path, name) in modules {
                    li {
                        class: if selected_module() == path { "selected" } else { "" },
                        onclick: move |_| {
                            set_selected_module.set(path.to_string());
                            set_selected_file.set(None);
                        },
                        "{name}"
                    }
                }
            }
            if !files.is_empty() {
                FileList { files: files.clone(), selected_file: selected_file.clone(), set_selected_file: set_selected_file.clone() }
            }
        }
    }
}

#[component]
fn FileList(
    files: Vec<std::borrow::Cow<'static, str>>,
    selected_file: Signal<Option<String>>,
    set_selected_file: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            h3 { "Files" }
            ul { class: "file-list",
                for file in files {
                    li {
                        class: if selected_file().as_ref() == Some(&file.to_string()) { "selected" } else { "" },
                        onclick: move |_| set_selected_file.set(Some(file.to_string())),
                        "{file}"
                    }
                }
            }
        }
    }
}

#[component]
fn FileViewer(file_content: Option<String>, selected_file: Signal<Option<String>>) -> Element {
    rsx! {
        if let Some(content) = file_content {
            div { class: "file-viewer",
                h3 { "File: {selected_file().unwrap_or_default()}" }
                pre { class: "code-content",
                    code { "{content}" }
                }
            }
        } else {
            div { class: "placeholder",
                p { "Select a file to view its content" }
            }
        }
    }
}
