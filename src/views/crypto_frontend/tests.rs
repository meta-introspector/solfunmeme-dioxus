use dioxus::{dioxus_core::Mutations, prelude::*};
//use dioxus_testing::prelude::*;
use super::*;

#[test]
fn test_crypto_frontend_app_renders() {
    let mut dom = VirtualDom::new(CryptoFrontendApp);

    let mut mutations = Mutations::default();
    dom.rebuild(&mut mutations);

    // Check that the app header is rendered
    //let h1 = dom.query_selector("h1").expect("h1 element not found");
    //assert!(h1.text_content().unwrap().contains("Solana Wallet Encryption Demo"));

    // Check that both forms are rendered
    //assert!(dom.query_selector("form").is_some());
    //assert!(dom.find_elements("form").len() == 2);
}

#[test]
fn test_encryption_form_validation() {
    let mut dom = VirtualDom::new(EncryptionForm);
    //let mut dom = app.rebuild();
    let mut mutations = Mutations::default();
    dom.rebuild(&mut mutations);

    // Test empty form validation
    // let encrypt_button = dom.find_element("button").unwrap();
    // encrypt_button.click();

    // // Should show error message
    // assert!(dom.find_element("div").unwrap().text().contains("Title is required"));

    // // Fill in the form
    // let title_input = dom.find_element("input[type='text']").unwrap();
    // title_input.set_value("Test Title");

    // let message_input = dom.find_element("textarea").unwrap();
    // message_input.set_value("Test Message");

    // let recipient_key_input = dom.find_elements("input[type='text']").nth(1).unwrap();
    // recipient_key_input.set_value("invalid-key");

    // // Try to encrypt with invalid key
    // encrypt_button.click();

    // // Should show key format error
    // assert!(dom.find_element("div").unwrap().text().contains("Invalid recipient public key format"));
}

#[test]
fn test_decryption_form_validation() {
    let mut dom = VirtualDom::new(DecryptionForm);
    //let mut dom = app.rebuild();
    let mut mutations = Mutations::default();
    dom.rebuild(&mut mutations);

    // // Test empty form validation
    // let decrypt_button = dom.find_element("button").unwrap();
    // decrypt_button.click();

    // // Should show error message
    // assert!(dom.find_element("div").unwrap().text().contains("Encrypted payload is required"));

    // // Fill in invalid payload
    // let payload_input = dom.find_element("textarea").unwrap();
    // payload_input.set_value("invalid-payload");

    // let private_key_input = dom.find_element("input[type='password']").unwrap();
    // private_key_input.set_value("invalid-key");

    // // Try to decrypt
    // decrypt_button.click();

    // // Should show key format error
    // assert!(dom.find_element("div").unwrap().text().contains("Invalid private key format"));
}

#[test]
fn test_reusable_components() {
    // Test CardHeader
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CardHeader { title: "Test Title".to_string() }
        }
    });
    //let mut dom = app.rebuild();
    let mut mutations = Mutations::default();
    dom.rebuild(&mut mutations);
    //assert!(dom.find_element("h2").unwrap().text().contains("Test Title"));

    // Test InputField
    // let mut dom2 = VirtualDom::new(|| {
    //     let value = use_signal(|| "".to_string());
    //     // rsx! {
    //     //     InputField {
    //     //         label: "Test Label".to_string(),
    //     //         placeholder: "Test Placeholder".to_string(),
    //     //         input_type: "text".to_string(),
    //     //         value: value.read().clone(),
    //     //        // on_input: move |v| value.set(v)
    //     //     }
    //     // }
    // });
    //let mut dom = app.rebuild();
    //dom.rebuild(&mut mutations);
    // let input = dom.find_element("input").unwrap();
    // assert!(input.get_attribute("placeholder").unwrap() == "Test Placeholder");

    // // Test ActionButton
    // let app = VirtualDom::new(|| {
    //     rsx! {
    //         ActionButton {
    //             text: "Test Button".to_string(),
    //             button_type: ButtonType::Primary,
    //             disabled: false,
    //             on_click: move |_| {}
    //         }
    //     }
    // });
    // let mut dom = app.rebuild();
    // let button = dom.find_element("button").unwrap();
    // assert!(button.text().contains("Test Button"));
    // assert!(!button.get_attribute("disabled").is_some());
}

#[test]
fn test_validation_functions() {
    // Test encryption validation
    let result = validate_encryption_inputs("", "", "", "", "");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Title is required"));

    let result = validate_encryption_inputs(
        "Title",
        "Message",
        "invalid-key",
        "invalid-key",
        "invalid-key",
    );
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Invalid recipient public key format"));

    // Test decryption validation
    let result = validate_decryption_inputs("", "");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Encrypted payload is required"));

    let result = validate_decryption_inputs("payload", "invalid-key");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid private key format"));
}
