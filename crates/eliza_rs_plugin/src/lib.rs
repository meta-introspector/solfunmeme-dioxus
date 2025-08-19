use anyhow::Result;
use eliza_rs::Eliza;

pub struct ElizaRsPlugin {
    eliza: Eliza,
}

impl ElizaRsPlugin {
    pub fn new() -> Result<Self> {
        let eliza = Eliza::new();
        Ok(ElizaRsPlugin { eliza })
    }

    pub fn respond(&self, input: &str) -> String {
        self.eliza.respond(input)
    }
}
