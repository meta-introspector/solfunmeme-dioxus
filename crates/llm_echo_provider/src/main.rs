use anyhow::Result;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use solfunmeme_models::LlmTaskPayload;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let task_payload: LlmTaskPayload = serde_json::from_str(&buffer)?;

    // Echo the received task payload back as JSON
    let response_json = serde_json::to_string_pretty(&task_payload)?;
    io::stdout().write_all(response_json.as_bytes())?;

    Ok(())
}