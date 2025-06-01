use dioxus::prelude::*;

use crate::{
    views::{SignMessage, SignTx},
    ConnectWalletFirst, SignInWithSolana, ACTIVE_CONNECTION,
};

#[component]
pub fn Extras() -> Element {
    if ACTIVE_CONNECTION.read().connected_account().is_ok() {
        rsx! {
            div { class:"flex justify-center mt-10 mb-5 gap-8 w-full flex-wrap items-stretch",
                SignInWithSolana{}
                SignMessage{}
                SignTx{}
            }
        }
    } else {
        rsx! {ConnectWalletFirst {}}
    }
}
