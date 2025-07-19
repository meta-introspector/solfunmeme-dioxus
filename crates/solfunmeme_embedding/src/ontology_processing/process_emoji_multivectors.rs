use anyhow::Result;
use std::collections::HashMap;
use solfunmeme_clifford::{SolMultivector, generate_multivector_from_string};

#[cfg(feature = "with-candle")]
use candle_core::Device;

#[cfg(feature = "with-candle")]
pub fn process_emoji_multivectors(emoji_data: HashMap<String, (String, String)>, concept_descriptions: HashMap<String, String>) -> Result<HashMap<String, (SolMultivector, String)>> {
    let mut emoji_multivectors: HashMap<String, (SolMultivector, String)> = HashMap::new();

    for (emoji_char, (concept_id, category)) in emoji_data {
        let description = concept_descriptions.get(&concept_id).cloned().unwrap_or(concept_id.clone());
        let multivector = generate_multivector_from_string(&description);
        emoji_multivectors.insert(emoji_char, (multivector, category));
    }

    Ok(emoji_multivectors)
}

#[cfg(not(feature = "with-candle"))]
pub fn process_emoji_multivectors(emoji_data: HashMap<String, (String, String)>, concept_descriptions: HashMap<String, String>) -> Result<HashMap<String, (SolMultivector, String)>> {
    let mut emoji_multivectors: HashMap<String, (SolMultivector, String)> = HashMap::new();
    for (emoji_char, (_, category)) in emoji_data {
        let multivector = generate_multivector_from_string(&emoji_char);
        emoji_multivectors.insert(emoji_char, (multivector, category));
    }
    Ok(emoji_multivectors)
}
