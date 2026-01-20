use dioxus::prelude::*;

mod plugin_loader;
use plugin_loader::PluginManager;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut plugin_manager = use_signal(|| PluginManager::new());
    
    rsx! {
        div { class: "app",
            h1 { "SOLFUNMEME v2" }
            p { "Minimal plugin-based architecture" }
            
            button {
                onclick: move |_| {
                    let result = plugin_manager.read().call("wallet", "connect", "");
                    log::info!("Plugin result: {}", result);
                },
                "Test Plugin Call"
            }
        }
    }
}
