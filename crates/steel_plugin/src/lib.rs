use anyhow::Result;
use steel::Steel;

pub struct SteelPlugin {
    steel: Steel,
}

impl SteelPlugin {
    pub fn new() -> Result<Self> {
        let steel = Steel::new();
        Ok(SteelPlugin { steel })
    }

    pub fn run_script(&self, script: &str) -> Result<String> {
        let result = self.steel.eval_to_string(script)?;
        Ok(result)
    }
}
