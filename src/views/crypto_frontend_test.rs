#[cfg(test)]
mod tests {
    use crate::views::crypto_frontend::{
        validate_decryption_inputs, ButtonType, CardHeader, CardHeaderProps, ErrorMessage,
        ErrorMessageProps, SuccessMessage, SuccessMessageProps,
    };
    //validate_encryption_inputs,

    #[test]
    fn button_type_enum_variants() {
        // Ensure all variants exist and are comparable
        let primary = ButtonType::Primary;
        let success = ButtonType::Success;
        let danger = ButtonType::Danger;

        assert_ne!(primary, success);
        assert_ne!(primary, danger);
        assert_ne!(success, danger);
    }

    // FIXME:
    // Must be called from inside a Dioxus runtime.

    // Help: Some APIs in dioxus require a global runtime to be present.
    // If you are calling one of these APIs from outside of a dioxus runtime
    // (typically in a web-sys closure or dynamic library), you will need to
    // grab the runtime from a scope that has it and then move it into your
    // new scope with a runtime guard.

    // For example, if you are trying to use dioxus apis from a web-sys
    // closure, you can grab the runtime from the scope it is created in:

    // ```rust
    // use dioxus::prelude::*;
    // static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

    // #[component]
    // fn MyComponent() -> Element {
    //     use_effect(|| {
    //         // Grab the runtime from the MyComponent scope
    //         let runtime = Runtime::current().expect("Components run in the Dioxus runtime");
    //         // Move the runtime into the web-sys closure scope
    //         let web_sys_closure = Closure::new(|| {
    //             // Then create a guard to provide the runtime to the closure
    //             let _guard = RuntimeGuard::new(runtime);
    //             // and run whatever code needs the runtime
    //             tracing::info!("The count is: {COUNT}");
    //         });
    //     })
    // }

    // #[test]
    // fn action_button_class_assignment() {
    //     // Test that the correct class is assigned for each button type
    //     let props_primary = ActionButtonProps {
    //         text: "Test".to_string(),
    //         button_type: ButtonType::Primary,
    //         disabled: false,
    //         on_click: EventHandler::new(|_| {}),
    //     };
    //     let props_success = ActionButtonProps {
    //         text: "Test".to_string(),
    //         button_type: ButtonType::Success,
    //         disabled: false,
    //         on_click: EventHandler::new(|_| {}),
    //     };
    //     let props_danger = ActionButtonProps {
    //         text: "Test".to_string(),
    //         button_type: ButtonType::Danger,
    //         disabled: false,
    //         on_click: EventHandler::new(|_| {}),
    //     };

    //     // Render and check class assignment (pseudo, as Dioxus doesn't support direct DOM testing in unit tests)
    //     let _ = ActionButton(props_primary);
    //     let _ = ActionButton(props_success);
    //     let _ = ActionButton(props_danger);
    // }

    // #[test]
    // fn validate_encryption_inputs_empty_fields() {
    //     let err = validate_encryption_inputs("", "msg", "rec", "priv", "pub").unwrap_err();
    //     assert_eq!(err, "Title is required");

    //     let err = validate_encryption_inputs("title", "", "rec", "priv", "pub").unwrap_err();
    //     assert_eq!(err, "Message is required");

    //     let err = validate_encryption_inputs("title", "msg", "", "priv", "pub").unwrap_err();
    //     assert_eq!(err, "Recipient public key is required");

    //     let err = validate_encryption_inputs("title", "msg", "rec", "", "pub").unwrap_err();
    //     assert_eq!(err, "Private key is required");

    //     let err = validate_encryption_inputs("title", "msg", "rec", "priv", "").unwrap_err();
    //     assert_eq!(err, "Public key is required");
    // }

    #[test]
    fn validate_decryption_inputs_empty_fields() {
        let err = validate_decryption_inputs("", "priv").unwrap_err();
        assert_eq!(err, "Encrypted payload is required");

        let err = validate_decryption_inputs("payload", "").unwrap_err();
        assert_eq!(err, "Private key is required");
    }

    #[test]
    fn card_header_renders_title() {
        let props = CardHeaderProps {
            title: "Test Title".to_string(),
        };
        let _ = CardHeader(props);
    }

    #[test]
    fn error_message_renders() {
        let props = ErrorMessageProps {
            message: "An error occurred".to_string(),
        };
        let _ = ErrorMessage(props);
    }

    #[test]
    fn success_message_renders_code_and_text() {
        let props_code = SuccessMessageProps {
            title: "Success".to_string(),
            content: "let x = 1;".to_string(),
            is_code: true,
        };
        let props_text = SuccessMessageProps {
            title: "Success".to_string(),
            content: "Operation completed".to_string(),
            is_code: false,
        };
        let _ = SuccessMessage(props_code);
        let _ = SuccessMessage(props_text);
    }
}
