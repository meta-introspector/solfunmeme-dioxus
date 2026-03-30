use dioxus::prelude::*;

use crate::{
    model::storage::ACTIVE_CONNECTION,
    views::{
        connect_first::ConnectWalletFirst,
        extras_views::{SignInWithSolana, SignMessage, SignTx},
    },
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
