use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "flex flex-col justify-around items-center w-full m-h-[100%] h-full p-5",
            h1 {class:"text-black dark:text-gray-300 text-6xl", "gm"}

            h2 {class:"dark:text-gray-300 text-2xl" , "Say hi to your new Solana dApp."}

            div {class:"text-center flex flex-col justify-between items-center",
                h3 {class:"text-black dark:text-gray-300 text-md", "Here are some helpful links to get you started."}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://crates.io/crates/wallet-adapter",  "Rust Wallet Adapter (crates.io)" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://github.com/JamiiDao/SolanaWalletAdapter", "Rust Wallet Adapter (Github)" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/examples", "Rust Wallet Adapter Examples" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/templates", "Rust Wallet Adapter Templates" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://docs.solana.com/", "Solana Docs" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://faucet.solana.com/", "Solana Faucet" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://solanacookbook.com/", "Solana Cookbook" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://solana.stackexchange.com/", "Solana Stack Overflow" }}
                div { a { class:"underline text-black dark:text-gray-300 text-md", href:"https://github.com/solana-developers/", "Solana Developers GitHub" }}
            }
        }
    }
}
