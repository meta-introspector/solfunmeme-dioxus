use anyhow::Result;
use gline_rs::Gline;

pub struct GlineRsPlugin {
    gline: Gline,
}

impl GlineRsPlugin {
    pub fn new() -> Result<Self> {
        let gline = Gline::new();
        Ok(GlineRsPlugin { gline })
    }

    pub fn process_line(&self, line: &str) -> String {
        self.gline.process_line(line)
    }
}
