use bootstrap_stage0::describable::Describable;
use crate::gemini::GeminiState;

pub struct GeminiStateArtifact {
    pub content: Vec<u8>,
}

impl Describable for GeminiStateArtifact {
    fn describe(&self) -> &[u8] {
        &self.content
    }
}

pub fn create_gemini_state_artifact(state: &GeminiState) -> GeminiStateArtifact {
    let content = bincode::serialize(state).unwrap();
    GeminiStateArtifact { content }
}
