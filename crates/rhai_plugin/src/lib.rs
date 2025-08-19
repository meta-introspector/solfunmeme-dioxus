use anyhow::Result;
use rhai::{Engine, Scope};

pub struct RhaiPlugin {
    engine: Engine,
}

impl RhaiPlugin {
    pub fn new() -> Self {
        let engine = Engine::new();
        RhaiPlugin { engine }
    }

    pub fn run_script(&self, script: &str) -> Result<String> {
        let mut scope = Scope::new();
        let result = self.engine.eval_with_scope::<String>(&mut scope, script)?;
        Ok(result)
    }
}
