use dioxus::prelude::*;

mod plugin_loader;
use plugin_loader::PluginManager;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut plugin_manager = use_signal(|| PluginManager::new());
    let mut block_data = use_signal(|| String::from("No data"));
    
    // Load plugin on mount
    use_effect(move || {
        spawn(async move {
            plugin_manager.write().load_plugin("/plugins/solana_p2p.wasm").await;
        });
    });
    
    rsx! {
        div { class: "app",
            h1 { "SOLFUNMEME v2" }
            p { "P2P Solana Block Fetcher" }
            
            button {
                onclick: move |_| {
                    spawn(async move {
                        let result = plugin_manager.read().call("solana-p2p", "get_block", "12345");
                        block_data.set(result);
                    });
                },
                "Fetch Block"
            }
            
            div { class: "result",
                pre { "{block_data}" }
            }
        }
    }
}
