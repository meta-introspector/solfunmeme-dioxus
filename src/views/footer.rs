use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "flex p-3 items-center justify-center w-full ",
            div{p { class: "flex flex-wrap md:flex-row lg:flex-row items-center justify-center",
                "Generated Using "
                a { href: "https://crates.io/crates/cargo-generate", class: "ml-2 mr-2 underline", " cargo-generate" }
                " and "
                a { href: "https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/templates", class: "ml-2 underline", "Rust Wallet Adapter Dioxus Template" }
            }}
        }
    }
}
