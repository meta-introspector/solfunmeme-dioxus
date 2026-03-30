use dioxus::prelude::*;
mod style;
use serde::{Deserialize, Serialize};
use style::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub emoji: String,
    pub component: String,
    pub description: String,
    pub test_case: String,
    pub lean_proof: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMeme {
    pub name: String,
    pub emoji_sequence: String,
    pub steps: Vec<WorkflowStep>,
    pub test_result: String,
    pub proof_result: String,
}
#[allow(dead_code)]
pub fn get_encryption_workflow() -> WorkflowMeme {
    WorkflowMeme {
        name: "Message Encryption Flow".to_string(),
        emoji_sequence: "📝🔑🔐📨🔓".to_string(),
        steps: vec![
            WorkflowStep {
                emoji: "📝".to_string(),
                component: "EncryptionForm".to_string(),
                description: "Write message and enter keys".to_string(),
                test_case: r#"
#[test]
fn test_encryption_form_input() {
    let app = VirtualDom::new(EncryptionForm);
    let mut dom = app.rebuild();
    
    // Fill in the form
    let title_input = dom.find_element("input[type='text']").unwrap();
    title_input.set_value("Test Title");
    
    let message_input = dom.find_element("textarea").unwrap();
    message_input.set_value("Test Message");
    
    // Verify inputs
    assert_eq!(title_input.value(), "Test Title");
    assert_eq!(message_input.value(), "Test Message");
}"#
                .to_string(),
                lean_proof: r#"
theorem encryption_form_valid : 
  ∀ (title message : String) (keys : KeyPair),
  isValidInput title message keys → 
  ∃ (form : EncryptionForm), form.title = title ∧ form.message = message :=
begin
  intros title message keys h,
  existsi { title := title, message := message, keys := keys },
  split; refl
end"#
                    .to_string(),
            },
            WorkflowStep {
                emoji: "🔑".to_string(),
                component: "KeyValidation".to_string(),
                description: "Validate encryption keys".to_string(),
                test_case: r#"
#[test]
fn test_key_validation() {
    let valid_key = "valid-solana-key";
    let invalid_key = "invalid-key";
    
    assert!(SolanaEncryption::is_valid_public_key(valid_key));
    assert!(!SolanaEncryption::is_valid_public_key(invalid_key));
}"#
                .to_string(),
                lean_proof: r#"
theorem key_validation_sound :
  ∀ (key : String),
  isValidKey key → 
  ∃ (pubKey : PublicKey), pubKey.value = key :=
begin
  intros key h,
  existsi { value := key },
  refl
end"#
                    .to_string(),
            },
            WorkflowStep {
                emoji: "🔐".to_string(),
                component: "Encryption".to_string(),
                description: "Encrypt the message".to_string(),
                test_case: r#"
#[test]
fn test_message_encryption() {
    let message = "Test Message";
    let key = "valid-solana-key";
    
    let encrypted = SolanaEncryption::encrypt(message, key);
    assert!(encrypted.is_ok());
    assert!(encrypted.unwrap().len() > 0);
}"#
                .to_string(),
                lean_proof: r#"
theorem encryption_preserves_message :
  ∀ (msg : String) (key : PublicKey),
  ∃ (encrypted : EncryptedPayload),
  decrypt encrypted key = msg :=
begin
  intros msg key,
  existsi (encrypt msg key),
  apply encryption_correct
end"#
                    .to_string(),
            },
            WorkflowStep {
                emoji: "📨".to_string(),
                component: "MessageTransmission".to_string(),
                description: "Send encrypted message".to_string(),
                test_case: r#"
#[test]
fn test_message_transmission() {
    let payload = "encrypted-payload";
    let result = transmit_message(payload);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, "sent");
}"#
                .to_string(),
                lean_proof: r#"
theorem transmission_preserves_payload :
  ∀ (payload : EncryptedPayload),
  transmit payload = some (transmitted payload) :=
begin
  intros payload,
  existsi (transmit payload),
  apply transmission_correct
end"#
                    .to_string(),
            },
            WorkflowStep {
                emoji: "🔓".to_string(),
                component: "DecryptionForm".to_string(),
                description: "Decrypt received message".to_string(),
                test_case: r#"
#[test]
fn test_message_decryption() {
    let payload = "encrypted-payload";
    let key = "valid-solana-key";
    
    let decrypted = SolanaEncryption::decrypt(payload, key);
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), "Test Message");
}"#
                .to_string(),
                lean_proof: r#"
theorem decryption_recovers_message :
  ∀ (payload : EncryptedPayload) (key : PrivateKey),
  decrypt (encrypt (decrypt payload key) key) key = decrypt payload key :=
begin
  intros payload key,
  apply encryption_decryption_inverse
end"#
                    .to_string(),
            },
        ],
        test_result: "✅ All tests passed!".to_string(),
        proof_result: "✓ All proofs verified!".to_string(),
    }
}

#[component]
pub fn WorkflowStepView(step: WorkflowStep) -> Element {
    rsx! {
        div { class: WORKFLOW_STEP,
            div { class: STEP_HEADER,
                span { class: STEP_EMOJI, "{step.emoji}" }
                h3 { class: STEP_TITLE, "{step.component}" }
            }
            p { class: STEP_DESCRIPTION, "{step.description}" }
            div { class: STEP_DETAILS,
                div { class: TEST_CASE,
                    h4 { "Test Case" }
                    pre { class: CODE_BLOCK,
                        "{step.test_case}"
                    }
                }
                div { class: LEAN_PROOF,
                    h4 { "Lean Proof" }
                    pre { class: CODE_BLOCK,
                        "{step.lean_proof}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn WorkflowMemeView(workflow: WorkflowMeme) -> Element {
    rsx! {
        div { class: WORKFLOW_MEME,
            div { class: WORKFLOW_HEADER,
                h2 { class: WORKFLOW_TITLE, "{workflow.name}" }
                div { class: EMOJI_SEQUENCE,
                    // workflow.emoji_sequence.chars().map(|emoji| rsx! {
                    //     span { class: SEQUENCE_EMOJI, "{emoji}" }
                    // })
                }
            }
            div { class: WORKFLOW_STEPS,
                // workflow.steps.iter().map(|step| rsx! {
                //     WorkflowStepView { step: step.clone() }
                // })
            }
            div { class: WORKFLOW_RESULTS,
                div { class: TEST_RESULT, "{workflow.test_result}" }
                div { class: PROOF_RESULT, "{workflow.proof_result}" }
            }
        }
    }
}

#[component]
pub fn WorkflowMemeExplorer() -> Element {
    let encryption_workflow = get_encryption_workflow();

    rsx! {
        div { class: WORKFLOW_EXPLORER,
            h1 { class: EXPLORER_TITLE, "Workflow Meme Explorer" }
            WorkflowMemeView { workflow: encryption_workflow }
        }
    }
}
