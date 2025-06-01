use dioxus::prelude::*;

use crate::WalletSvg;

#[component]
pub fn ConnectWalletFirst() -> Element {
    rsx! {
        div {class:"flex w-full text-2xl justify-center items-center",
            span { class:"flex w-[30px]", {WalletSvg()}}
            "Connect a Wallet first!"
        }
    }
}
