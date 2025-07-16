use anyhow::Result;
use json_ld::{Compact, Expand, Flatten, Frame, JsonLdProcessor, Options};
use serde_json::Value;

pub struct JsonLdPlugin;

impl JsonLdPlugin {
    pub async fn compact(value: Value, context: Value, options: Options) -> Result<Value> {
        let compacted = JsonLdProcessor::compact(value, context, options).await?;
        Ok(compacted)
    }

    pub async fn expand(value: Value, options: Options) -> Result<Value> {
        let expanded = JsonLdProcessor::expand(value, options).await?;
        Ok(expanded)
    }

    pub async fn flatten(value: Value, context: Option<Value>, options: Options) -> Result<Value> {
        let flattened = JsonLdProcessor::flatten(value, context, options).await?;
        Ok(flattened)
    }

    pub async fn frame(value: Value, frame: Value, options: Options) -> Result<Value> {
        let framed = JsonLdProcessor::frame(value, frame, options).await?;
        Ok(framed)
    }
}
