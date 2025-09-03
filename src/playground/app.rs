use crate::model::lean::style::{Styles, THEME};
use dioxus::prelude::*;
//use crate::app::{FAVICON, TAILWIND_CSS};
use crate::{model::NotificationInfo, playground::solfunnice::SolFunNiceApp};
//use crate::extractor::components::app::EmbeddingApp;
//use crate::extractor::system::clipboard::copy_all_snippets_combined;
//ouse crate::extractor::error;
//use crate::password_manager::DecryptedEntry;
// use crate::playground::MenuOption::Airdrop;
// todo rename : airdrop::Airdrop,
use crate::views::{
    accounts::Accounts, clusters::Clusters,
    component_memes::ComponentMemeExplorer, connection_buttons::ConnectionButtons,
    connections::Connections, core_buttons::CoreButtons, crypto_buttons::CryptoButtons,
    dashboard::Dashboard, encryption::Encryption, expression_parsing::ExpressionParsing,
    management_buttons::ManagementButtons, meme_management::MemeManagement,
    meta_meme_operations::MetaMemeOperations,

    // views
    receive_sol::ReceiveSolComponent, send_sol::SendSolComponent,
    styling_and_emojis::StylingAndEmojis, transaction_buttons::TransactionButtons,
    airdrop::AirdropComponent,
    source_browser::SourceBrowser,
    emoji_matrix_view::EmojiMatrixView,
};
//pub mod embedding;
//use crate::playground::embedding::EmbeddingApp;
use crate::playground::{
    bert_test::BertTestApp, performance_charts::PerformanceCharts, rust_parser::RustParserApp,
};

use crate::{
    extractor::components::extractor::MarkdownCodeExtractor,
    playground::{mcp::MCPPlaygroundApp, monster_meta_meme::MonsterMetaMemeApp},
};

#[derive(PartialEq, Clone)]
pub enum MenuOption {
    Embedding,
    PerformanceCharts,
    BertTest,
    RustParser,
    #[allow(dead_code)]
    MemeManagement,
    #[allow(dead_code)]
    ExpressionParsing,
    #[allow(dead_code)]
    Encryption,
    #[allow(dead_code)]
    MetaMemeOperations,
    #[allow(dead_code)]
    StylingAndEmojis,
    #[allow(dead_code)]
    CryptoFrontend,
    #[allow(dead_code)]
    Memes,
    #[allow(dead_code)]
    Lean,
    #[allow(dead_code)]
    ConnectionManagement,
    #[allow(dead_code)]
    ConnectionList,
    #[allow(dead_code)]
    SendSol,
    #[allow(dead_code)]
    ReceiveSol,
    #[allow(dead_code)]
    QueryAccounts,
    #[allow(dead_code)]
    Dashboard,
    #[allow(dead_code)]
    Connections,
    #[allow(dead_code)]
    ClustersManagement,
    #[allow(dead_code)]
    Clusters,
    #[allow(dead_code)]
    Airdrop,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    ComponentMemes,
    #[allow(dead_code)]
    MonsterMetaMeme,
    #[allow(dead_code)]
    SolFunMeme,
    #[allow(dead_code)]
    Extractor,
    SourceBrowser,
    EmojiMatrix,
}

use crate::extractor::components::example::register_all_components;
#[component]
pub fn PlaygroundApp() -> Element {
//    crate::embedself::printall;
    register_all_components();

    //    let mut menu_option = use_signal(|| MenuOption::MemeManagement);
    let mut menu_option = use_signal(|| MenuOption::Embedding);
    let notifications = use_signal(|| {
        vec![NotificationInfo {
            key: 1,
            secs: 5,
            message: "Welcome to SOLFUNMEME App!".to_string(),
        }]
    });
    let show_send_modal = use_signal(|| false);
    let show_receive_modal = use_signal(|| false);
    let show_airdrop_modal = use_signal(|| false);

    rsx! {

//            link { rel: "stylesheet", href: TAILWIND_CSS }
//            link { rel: "icon", href: FAVICON }
            div {
                style: format!(
                    "background: {}; padding: {}; font-family: {}",
                    THEME.background_color, THEME.spacing_md, THEME.font_family_sans
                ),
                nav {
                    // The change here is to fix the code block so that the `div` element is properly closed.
                    // Previously, the `div` was not closed before the next sibling element, which would cause a syntax or rendering error.
                    // Now, the `div` wraps only the intended button components and is closed before the next elements in the parent `nav`.
                    class: "{Styles::header()}",
                    div {
                        class: "{Styles::flex_between()}",
                        style: "flex-wrap: wrap; gap: 0.5rem;",
                        CoreButtons { on_menu_change: move |opt| menu_option.set(opt) }
                        CryptoButtons { on_menu_change: move |opt| menu_option.set(opt) }
                        ConnectionButtons { on_menu_change: move |opt| menu_option.set(opt) }
                    }
                    div {
                        TransactionButtons { on_menu_change: move |opt| menu_option.set(opt) }
                        ManagementButtons { on_menu_change: move |opt| menu_option.set(opt) }
                    }
                }
                div {
                    class: "{Styles::section()}",

                    {
                        notifications.read().iter().map(|notif| rsx! {
                            div {
                                key: "{notif.key}",
                                style: "color: {THEME.text_primary}; margin-bottom: {THEME.spacing_sm}",
                                "{notif.message} (Visible for {notif.secs} seconds)"
                            }
                        })
                    }
                }
                div {
                    class: "{Styles::app_container()}",
                    match *menu_option.read() {
                        MenuOption::MemeManagement => rsx!(MemeManagement {}),
                        MenuOption::Memes => rsx!(MemeManagement {}),
                        MenuOption::ExpressionParsing => rsx!(ExpressionParsing {}),
                        MenuOption::Encryption => rsx!(Encryption {}),
                        MenuOption::MetaMemeOperations => rsx!(MetaMemeOperations {}),
                        MenuOption::StylingAndEmojis => rsx!(StylingAndEmojis {}),
                        //MenuOption::CryptoFrontend => rsx!(CryptoFrontendApp {}),
                        //MenuOption::Lean => rsx!(LeanEditor {}),
                        //MenuOption::ConnectionManagement => rsx!(ConnectionManagement {}),
                        //MenuOption::ConnectionList => rsx!(ConnectionList {}),
                        MenuOption::SendSol => rsx!(SendSolComponent { show_send_modal: show_send_modal }),
                        MenuOption::ReceiveSol => rsx!(ReceiveSolComponent { show_receive_modal: show_receive_modal }),
                        //MenuOption::QueryAccounts => rsx!(QueryAccounts {}),
                        MenuOption::Dashboard => rsx!(Dashboard {}),
                        MenuOption::Connections => rsx!(Connections {}),
                        //MenuOption::ClustersManagement => rsx!(ClustersManagement {}),
                        MenuOption::Clusters => rsx!(Clusters {}),
                        MenuOption::Airdrop => rsx!(AirdropComponent { show_airdrop_modal: show_airdrop_modal }),
                        MenuOption::Accounts => rsx!(Accounts {}),
                        MenuOption::ComponentMemes => rsx!(ComponentMemeExplorer {}),
    //                    MenuOption::Embedding => rsx!(EmbeddingApp {}),
                        MenuOption::PerformanceCharts => rsx!(PerformanceCharts {}),
                        MenuOption::BertTest => rsx!(BertTestApp {}),
                        MenuOption::RustParser => rsx!(RustParserApp {}),
                        MenuOption::MonsterMetaMeme => rsx!(MonsterMetaMemeApp {}),
                //                    MenuOption::SolFunMeme => rsx!(SolFunMemeApp {}),
                MenuOption::SolFunMeme => rsx!(SolFunNiceApp {}),
                MenuOption::Extractor => rsx!(MarkdownCodeExtractor {}),
                        MenuOption::SourceBrowser => rsx!(SourceBrowser {}),
                        MenuOption::EmojiMatrix => rsx!(EmojiMatrixView {}),
                        _ => rsx!(div { "TODO"})
                    }
                }
            }

        MCPPlaygroundApp {}
        } // rsx
}
