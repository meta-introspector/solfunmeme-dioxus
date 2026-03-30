use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PluginCall {
    pub plugin: String,
    pub method: String,
    pub args: String,
}

#[derive(Serialize, Deserialize)]
pub struct PluginResponse {
    pub success: bool,
    pub data: String,
}

pub struct PluginManager {
    plugins: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }
    
    pub async fn load_plugin(&mut self, url: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            log::info!("Loading plugin: {}", url);
            self.plugins.push(url.to_string());
        }
    }
    
    pub fn call(&self, plugin: &str, method: &str, args: &str) -> String {
        format!("{{\"success\":true,\"data\":\"called {}.{}({})\"}}", plugin, method, args)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
