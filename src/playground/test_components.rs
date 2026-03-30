pub use dioxus::prelude::*;

use crate::password_manager::PasswordAppState;
pub use crate::playground::{
    test_components::ComponentName::{
        ActionButton, AddClusterModal, CardHeader, ClusterInfo, CodeDisplay, ConnectWalletFirst,
        ConnectionButtons, CoreButtons, CreateButton, CryptoAppHeader, CryptoButtons,
        CryptoErrorMessage, CryptoFrontendApp, DecryptionForm, EncryptionForm, ExpressionCard,
        ExpressionInputs, ExpressionList, ExpressionMetadata, ExpressionTypeSelector, Footer,
        InputField, InputSection, ManagementButtons, MemeCardHeader, MemesFooter, MetadataInputs,
        Notification, Notification2, PageNotFound,
	QueryAccountDialogName, // a name
	QueryCoinDialog,
        SearchInput, SignInWithSolana, SignTx, SimilaritySection, SuccessMessage,
        TextAreaField, TransactionButtons, VectorSpace, WikidataMemeExplorer, WikidataMemeView,
        WorkflowMemeExplorer, WorkflowMemeView, WorkflowStepView,
    },
    MenuOption::{
        Airdrop, MemeManagement, Memes, MetaMemeOperations, ReceiveSol, SendSol, StylingAndEmojis,
    },
};
//pub use crate::Route::Clusters;
//pub use crate::Route::TestMenuApp;
//crate::playground::test_app::TestMenuApp_completions::Component::TestMenuApp
//pub use crate::Route::Dashboard;
//pub use crate::Route::Extras;
//     airdrop::Airdrop,
//     clusters::{AddClusterModal, ClusterInfo, Clusters},
//     coins::QueryCoinDialog,
// //    component_meme::{ComponentMemeExplorer, ComponentMemeView, MemeCategoryView},
//     connect_first::ConnectWalletFirst,
//     connection_buttons::ConnectionButtons,
//     core_buttons::CoreButtons,
//     crypto_buttons::CryptoButtons,
//     crypto_frontend::{
//         app::{AppHeader as CryptoAppHeader, CryptoFrontendApp},
//         components::{ActionButton, CardHeader, ErrorMessage as CryptoErrorMessage, InputField, SuccessMessage, TextAreaField},
//         forms::{DecryptionForm, EncryptionForm},
//     },
//     dashboard::Dashboard,
//     //encryption::Encryption,
//     //expression_parsing::ExpressionParsing,
//     extras::Extras,
//     extras_views::{sign_message::SignMessage, sign_tx::SignTx, siws::SignInWithSolana},
//     footer::Footer,
//     //git::GitParser2,
//     //header::{ActiveAccountDropDown, ConnectWalletModalModal, Header, NavWalletItem, PingCluster},
//     management_buttons::ManagementButtons,
//     meme_management::MemeManagement,
//     memes::{
//         CardHeader as MemeCardHeader, CodeDisplay, CreateButton, ExpressionCard, ExpressionInputs, ExpressionList,
//         ExpressionMetadata, ExpressionTypeSelector, InputSection, Memes, MemesFooter, MetadataInputs, SearchInput,
//         SimilaritySection, VectorSpace,
//     },
//     meta_meme_operations::MetaMemeOperations,
//     notification::{Notification, Notification2},
//     page_not_found::PageNotFound,
//     // password_manager::{
//     //     AddPasswordForm, AppHeader as PasswordAppHeader, ErrorMessage as PasswordErrorMessage, LoginScreen, MainApp as PasswordMainApp,
//     //     PasswordApp, PasswordDetail, PasswordList, WelcomeScreen,
//     // },
//     query_accounts::QueryAccountDialog,
//     receive_sol::ReceiveSol,
//     send_sol::SendSol,
//     styling_and_emojis::StylingAndEmojis,
//     transaction_buttons::TransactionButtons,
//     wikidata_memes::{WikidataMemeExplorer, WikidataMemeView},
//     workflow_memes::{WorkflowMemeExplorer, WorkflowMemeView, WorkflowStepView},
// }};
// //pub use crate::{MenuOption, pub useConnections};

// Component metadata

//pub use dioxus_router::prelude::*;
pub use crate::{
    playground::{
        test_components::ComponentName::{
            AddPasswordForm, ComponentMemeExplorer, ComponentMemeView, GitParser2, LoginScreen,
            MemeCategoryView, PasswordAppHeader, PasswordDetail, PasswordErrorMessage,
            PasswordList, PasswordMainApp, WelcomeScreen,
        },
        MenuOption,
    },
    views::{
        component_memes::{ComponentMeme, MemeCategory},
        wikidata_memes::WikidataMeme,
        workflow_memes::WorkflowMeme,
    },
};
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
//pub use crate::password_manager::PasswordApp;
pub use crate::{
//    header::{ActiveAccountDropDown, ConnectWalletModalModal, Header},
    playground::test_components::ComponentName::{NavWalletItem, PingCluster},
};
//pub use crate::model::useConnections;
pub use crate::{
//    password_manager::PasswordAppState,
    views::{
        accounts::{Accounts, ClusterSuccess, TokenAccountCard, TxCard},
        workflow_memes::WorkflowStep,
    },
};
//let food =1;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentName {
    Header,
    ConnectWalletModalModal,
    NavWalletItem,
    ActiveAccountDropDown,
    PingCluster,
    PasswordApp,
    PasswordAppHeader,
    PasswordErrorMessage,
    LoginScreen,
    PasswordMainApp,
    PasswordList,
    AddPasswordForm,
    PasswordDetail,
    WelcomeScreen,
    Accounts,
    ClusterSuccess,
    TokenAccountCard,
    TxCard,
    AirdropName,
    Clusters,
    ClusterInfo,
    AddClusterModal,
    QueryCoinDialog,
    ComponentMemeView,
    MemeCategoryView,
    ComponentMemeExplorer,
    ConnectionButtons,
    ConnectWalletFirst,
    CoreButtons,
    CryptoButtons,
    CryptoFrontendApp,
    CryptoAppHeader,
    CardHeader,
    InputField,
    TextAreaField,
    ActionButton,
    CryptoErrorMessage,
    SuccessMessage,
    EncryptionForm,
    DecryptionForm,
    Dashboard,
    Extras,
    SignMessage,
    SignTx,
    SignInWithSolana,
    Footer,
    GitParser2,
    ManagementButtons,
    MemeManagement,
    Memes,
    MemeCardHeader,
    InputSection,
    ExpressionTypeSelector,
    ExpressionInputs,
    MetadataInputs,
    CreateButton,
    SearchInput,
    ExpressionList,
    ExpressionCard,
    CodeDisplay,
    ExpressionMetadata,
    SimilaritySection,
    VectorSpace,
    MemesFooter,
    MetaMemeOperations,
    Notification,
    Notification2,
    PageNotFound,
    QueryAccountDialogName,
    ReceiveSol,
    SendSol,
    StylingAndEmojis,
    TransactionButtons,
    WikidataMemeView,
    WikidataMemeExplorer,
    WorkflowStepView,
    WorkflowMemeView,
    WorkflowMemeExplorer,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    pub name: ComponentName,
    pub props: HashMap<String, PropValue>,
    pub id: u32,
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    Bool(bool),
    String(String),
    //SignalBool(Signal<bool>),
    //SignalString(Signal<String>),
    //SignalPasswordAppState(Signal<PasswordAppState>),
    ComponentMeme(ComponentMeme),
    MemeCategory(MemeCategory),
    WikidataMeme(WikidataMeme),
    WorkflowMeme(WorkflowMeme),
    WorkflowStep(WorkflowStep),
    //    pub useConnections(pub useConnections),
    //    MenuOptionHandler(EventHandler<MenuOption>),
    StringVec(Vec<String>),
}

#[component]
pub fn ComponentBuilderApp() -> Element {
    let selected_component = use_signal(|| None::<ComponentName>);
    let components = use_signal(|| vec![] as Vec<ComponentInstance>);
    let next_id = use_signal(|| 0u32);
    let mut props_config = use_signal(|| HashMap::<String, PropValue>::new());

    // Handle prop updates
    let update_prop = move |key: String, value: PropValue| {
        props_config.write().insert(key, value);
    };

    // Add component to composition
    let add_component = move || {
        // if let Some(name) = selected_component.read() {
        //     let instance = ComponentInstance {
        //         name: name.clone(),
        //         props: props_config.read().clone(),
        //         id: *next_id.read(),
        //     };
        //     components.write().push(instance);
        //     next_id += 1;
        //     props_config.set(HashMap::new()); // Reset props
        //     selected_component.set(None);
        // }
    };
    rsx! {
    div { "FIXME" }}

    //     rsx! {
    //         div { class: "container mx-auto p-4",
    //             h1 { class: "text-3xl font-bold mb-6 text-center",
    //                 "Component Builder"
    //             }
    //             div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
    //                 // Component Selection Sidebar
    //                 div { class: "lg:col-span-1",
    //                     div { class: "bg-white shadow-lg rounded-lg p-4",
    //                           h2 { class: "text-xl font-semibold mb-4", "Select Component" }

    //                         // for category in get_component_categories() {
    //                         //     div { class: "mb-4",
    //                         //         h3 { class: "text-lg font-medium mb-2", "{category.0}" }
    //                         //         for component in category.1 {
    //                         //             button {
    //                         //                 class: format!(
    //                         //                     "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
    //                         //                     if selected_component() == Some(component.clone()) {
    //                         //                         "bg-blue-500 text-white"
    //                         //                     } else {
    //                         //                         "bg-gray-100 hover:bg-gray-200"
    //                         //                     }
    //                         //                 ),
    //                         //                 onclick: move |_| selected_component.set(Some(component.clone())),
    //                         //                 "{component_name(&component)}"
    //                         //             }
    //                         //         }
    //                         //     }
    //                         // }
    //                     }
    //                 }
    // 	    }
    // 	}

    //         // Configuration and Preview Panel
    //         div { class: "lg:col-span-3",
    //               div { class: "bg-white shadow-lg rounded-lg p-6",
    //                     div { class: "flex justify-between items-center mb-4",
    //                           h2 { class: "text-2xl font-semibold",
    //                                "{selected_component().map(|c| component_name(&c)).unwrap_or_default()}"
    //                           }
    //                           button {
    //                               class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
    //                               onclick: move |_| add_component(),
    //                               "Add Component"
    //                             }
    //                         }
    //                         if let Some(component) = selected_component() {
    //                             ComponentConfigPanel {
    //                                 component,
    // //                                on_update: update_prop
    //                             }
    //                         }
    //                         div { class: "mt-6",
    //                             h3 { class: "text-lg font-semibold mb-2", "Composed Components" }
    //                             div { class: "grid gap-4",
    //                                 for instance in components.read().iter() {
    //                                     RenderComponent { instance: instance.clone() }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }

    //             // Navigation Link
    //             div { class: "mt-4",
    // //                Link {
    // //                    to: Route::TestMenuApp {},
    // //                    class: "bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600",
    // //                    "Back to Test Menu"
    // //                }
    //             }
    //         }
}

#[component]
fn ComponentConfigPanel(
    component: ComponentName,
    on_update: EventHandler<(String, PropValue)>,
) -> Element {
    let props = get_component_props(&component);
    rsx! {
            div { class: "border border-gray-200 rounded-lg p-4",
                h3 { class: "font-medium text-lg mb-2", "Configure Properties" }
                for prop in props {
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700", "{prop.0}" }
                        match prop.1 {
                            "bool" => {
                                let value = use_signal(|| false);
                                rsx! {
                                    input {
                                        r#type: "checkbox",
                                        checked: "{value}",
                                        onchange: move |evt| {
    //                                        value.set(evt.checked());
    //                                        on_update.call((prop.0.to_string(), PropValue::SignalBool(value)));
                                        }
                                    }
                                }
                            }
                            "string" => {
                                let value = use_signal(|| String::new());
                                rsx! {
                                    input {
                                        class: "w-full p-2 border border-gray-300 rounded-lg",
                                        value: "{value}",
                                        oninput: move |evt| {
    //                                        value.set(evt.value().clone());
    //                                        on_update.call((prop.0.to_string(), PropValue::SignalString(value)));
                                        }
                                    }
                                }
                            }
                            "password_app_state" => {
                                let value = use_signal(|| PasswordAppState::default());
                                rsx! {
                                    button {
                                        class: "bg-blue-500 text-white px-3 py-1 rounded",
                                        onclick: move |_| {
    //                                        value.set(PasswordAppState::default());
    //                                        on_update.call((prop.0.to_string(), PropValue::SignalPasswordAppState(value)));
                                        },
                                        "Reset State"
                                    }
                                }
                            }
                            _ => rsx! { span { class: "text-sm text-gray-500", "Unsupported prop type" } },
                        }
                    }
                }
            }
        }
}

#[component]
fn RenderComponent(instance: ComponentInstance) -> Element {
    match instance.name {
        //         ComponentName::Header => rsx! { Header {} },
        //         ComponentName::ConnectWalletModalModal => {
        //             let show_modal = instance.props.get("show_modal").and_then(|v| match v {
        //                 //PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let show_connecting = instance.props.get("show_connecting").and_then(|v| match v {
        // //                PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ConnectWalletModalModal { show_modal, show_connecting } }
        //         }
        //         ComponentName::NavWalletItem => {
        // //             let show_modal = instance.props.get("show_modal").and_then(|v| match v {
        // // //                PropValue::SignalBool(s) => Some(*s),
        // //                 _ => None,
        // //             }).unwrap_or_default();
        // //             let show_connecting = instance.props.get("show_connecting").and_then(|v| match v {
        // // //                PropValue::SignalBool(s) => Some(*s),
        // //                 _ => None,
        // //             }).unwrap_or_default();
        // 	    // //            rsx! { NavWalletItem { show_modal, show_connecting } }
        // 	    //rsx! div {"fixme"}
        //         }
        //         ComponentName::ActiveAccountDropDown => {

        //             let show_modal = instance.props.get("show_modal").and_then(|v| match v {
        // //                PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let shortened_address = instance.props.get("shortened_address").and_then(|v| match v {
        // //                PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             //rsx! { ActiveAccountDropDown { show_modal, shortened_address } }
        //         }
        // //        ComponentName::PingCluster => rsx! { PingCluster {} },
        //         ComponentName::PasswordApp => rsx! { PasswordApp {} },
        //         ComponentName::PasswordAppHeader => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PasswordAppHeader { app_state } }
        //         }
        //         ComponentName::PasswordErrorMessage => {
        //             let message = instance.props.get("message").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PasswordErrorMessage { message } }
        //         }
        //         ComponentName::LoginScreen => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { LoginScreen { app_state } }
        //         }
        //         ComponentName::PasswordMainApp => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PasswordMainApp { app_state } }
        //         }
        //         ComponentName::PasswordList => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PasswordList { app_state } }
        //         }
        //         ComponentName::AddPasswordForm => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { AddPasswordForm { app_state } }
        //         }
        //         ComponentName::PasswordDetail => {
        //             let app_state = instance.props.get("app_state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PasswordDetail { app_state } }
        //         }
        //         ComponentName::WelcomeScreen => rsx! { WelcomeScreen {} },
        //         ComponentName::Accounts => rsx! { Accounts {} },
        //         ComponentName::ClusterSuccess => {
        //             let address = instance.props.get("address").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let shortened_address = instance.props.get("shortened_address").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ClusterSuccess { address, shortened_address } }
        //         }
        //         ComponentName::TokenAccountCard => {
        //             let mint = instance.props.get("mint").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let ata_address = instance.props.get("ata_address").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { TokenAccountCard { mint, ata_address } }
        //         }
        //         ComponentName::TxCard => {
        //             let tx = instance.props.get("tx").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let timestamp = instance.props.get("timestamp").and_then(|v| match v {
        //                 PropValue::String(s) => s.parse::<i64>().ok(),
        //                 _ => None,
        //             });
        //             rsx! { TxCard { tx, timestamp } }
        //         }
        //         ComponentName::Airdrop => {
        //             let show_airdrop_modal = instance.props.get("show_airdrop_modal").and_then(|v| match v {
        //                 PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { Airdrop { show_airdrop_modal } }
        //         }
        //         ComponentName::Clusters => rsx! { Clusters {} },
        //         ComponentName::ClusterInfo => {
        //             let connections = instance.props.get("connections").and_then(|v| match v {
        //                 PropValue::pub useConnections(c) => Some(c.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ClusterInfo { connections } }
        //         }
        //         ComponentName::AddClusterModal => {
        //             let show_add_entry_modal = instance.props.get("show_add_entry_modal").and_then(|v| match v {
        //                 PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let connections = instance.props.get("connections").and_then(|v| match v {
        //                 PropValue::pub useConnections(c) => Some(c.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { AddClusterModal { show_add_entry_modal, connections } }
        //         }
        //         ComponentName::QueryCoinDialog => rsx! { QueryCoinDialog {} },
        //         ComponentName::ComponentMemeView => {
        //             let component_meme = instance.props.get("component_meme").and_then(|v| match v {
        //                 PropValue::ComponentMeme(c) => Some(c.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ComponentMemeView { component_meme } }
        //         }
        //         ComponentName::MemeCategoryView => {
        //             let category = instance.props.get("category").and_then(|v| match v {
        //                 PropValue::MemeCategory(c) => Some(c.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { MemeCategoryView { category } }
        //         }
        //         ComponentName::ComponentMemeExplorer => rsx! { ComponentMemeExplorer {} },
        //         ComponentName::ConnectionButtons => {
        //             let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //                 PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ConnectionButtons { on_menu_change } }
        //         }
        //         ComponentName::ConnectWalletFirst => rsx! { ConnectWalletFirst {} },
        //         ComponentName::CoreButtons => {
        //             let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //                 PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CoreButtons { on_menu_change } }
        //         }
        //         ComponentName::CryptoButtons => {
        //             let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //                 PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CryptoButtons { on_menu_change } }
        //         }
        //         ComponentName::CryptoFrontendApp => rsx! { CryptoFrontendApp {} },
        //         ComponentName::CryptoAppHeader => rsx! { CryptoAppHeader {} },
        //         ComponentName::CardHeader => {
        //             let title = instance.props.get("title").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CardHeader { title } }
        //         }
        //         ComponentName::InputField => {
        //             let label = instance.props.get("label").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { InputField { label } }
        //         }
        //         ComponentName::TextAreaField => {
        //             let label = instance.props.get("label").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { TextAreaField { label } }
        //         }
        //         ComponentName::ActionButton => {
        //             let label = instance.props.get("label").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ActionButton { label } }
        //         }
        //         ComponentName::CryptoErrorMessage => {
        //             let message = instance.props.get("message").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CryptoErrorMessage { message } }
        //         }
        //         ComponentName::SuccessMessage => {
        //             let message = instance.props.get("message").and_then(|v| match v {
        //                 PropValue::SignalString(s) => Some(s.read().clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { SuccessMessage { message } }
        //         }
        //         ComponentName::EncryptionForm => rsx! { EncryptionForm {} },
        //         ComponentName::DecryptionForm => rsx! { DecryptionForm {} },
        //         ComponentName::Dashboard => rsx! { Dashboard {} },
        //         ComponentName::Extras => rsx! { Extras {} },
        //         ComponentName::SignMessage => rsx! { SignMessage {} },
        //         ComponentName::SignTx => rsx! { SignTx {} },
        //         ComponentName::SignInWithSolana => rsx! { SignInWithSolana {} },
        //         ComponentName::Footer => rsx! { Footer {} },
        //         ComponentName::GitParser2 => rsx! { GitParser2 {} },
        //         ComponentName::ManagementButtons => {
        //             let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //                 PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ManagementButtons { on_menu_change } }
        //         }
        //         ComponentName::MemeManagement => rsx! { MemeManagement {} },
        //         ComponentName::Memes => rsx! { Memes {} },
        //         ComponentName::MemeCardHeader => {
        //             let expr = instance.props.get("expression").and_then(|v| match v {
        //                 PropValue::String(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { MemeCardHeader { expression: expr, state } }
        //         }
        //         ComponentName::InputSection => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { InputSection { state } }
        //         }
        //         ComponentName::ExpressionTypeSelector => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ExpressionTypeSelector { state } }
        //         }
        //         ComponentName::ExpressionInputs => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ExpressionInputs { state } }
        //         }
        //         ComponentName::MetadataInputs => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { MetadataInputs { state } }
        //         }
        //         ComponentName::CreateButton => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CreateButton { state } }
        //         }
        //         ComponentName::SearchInput => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { SearchInput { state } }
        //         }
        //         ComponentName::ExpressionList => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ExpressionList { state } }
        //         }
        //         ComponentName::ExpressionCard => {
        //             let expr = instance.props.get("expression").and_then(|v| match v {
        //                 PropValue::String(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ExpressionCard { expression: expr, state } }
        //         }
        //         ComponentName::CodeDisplay => {
        //             let expr = instance.props.get("expression").and_then(|v| match v {
        //                 PropValue::String(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { CodeDisplay { expression: expr } }
        //         }
        //         ComponentName::ExpressionMetadata => {
        //             let expr = instance.props.get("expression").and_then(|v| match v {
        //                 PropValue::String(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ExpressionMetadata { expression: expr } }
        //         }
        //         ComponentName::SimilaritySection => {
        //             let expr = instance.props.get("expression").and_then(|v| match v {
        //                 PropValue::String(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { SimilaritySection { expression: expr, state } }
        //         }
        //         ComponentName::VectorSpace => {
        //             let state = instance.props.get("state").and_then(|v| match v {
        //                 PropValue::SignalPasswordAppState(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { VectorSpace { state } }
        //         }
        //         ComponentName::MemesFooter => rsx! { MemesFooter {} },
        //         ComponentName::MetaMemeOperations => rsx! { MetaMemeOperations {} },
        //         ComponentName::Notification => rsx! { Notification {} },
        //         ComponentName::Notification2 => rsx! { Notification2 {} },
        //         ComponentName::PageNotFound => {
        //             let route = instance.props.get("route").and_then(|v| match v {
        //                 PropValue::StringVec(v) => Some(v.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { PageNotFound { route } }
        //         }
        //         ComponentName::QueryAccountDialog => {
        //             let show_query_dialog = instance.props.get("show_query_dialog").and_then(|v| match v {
        //                 PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { QueryAccountDialog { show_query_dialog } }
        //         }
        //         ComponentName::ReceiveSol => {
        //             let show_receive_modal = instance.props.get("show_receive_modal").and_then(|v| match v {
        //                 PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { ReceiveSol { show_receive_modal } }
        //         }
        //         ComponentName::SendSol => {
        //             let show_send_modal = instance.props.get("show_send_modal").and_then(|v| match v {
        //                 PropValue::SignalBool(s) => Some(*s),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { SendSol { show_send_modal } }
        //         }
        //         ComponentName::StylingAndEmojis => rsx! { StylingAndEmojis {} },
        //         ComponentName::TransactionButtons => {
        //             let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //                 PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { TransactionButtons { on_menu_change } }
        //         }
        //         ComponentName::WikidataMemeView => {
        //             let meme = instance.props.get("meme").and_then(|v| match v {
        //                 PropValue::WikidataMeme(m) => Some(m.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { WikidataMemeView { meme } }
        //         }
        //         ComponentName::WikidataMemeExplorer => rsx! { WikidataMemeExplorer {} },
        //         ComponentName::WorkflowStepView => {
        //             let step = instance.props.get("step").and_then(|v| match v {
        //                 PropValue::WorkflowStep(s) => Some(s.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { WorkflowStepView { step } }
        //         }
        //         ComponentName::WorkflowMemeView => {
        //             let workflow = instance.props.get("workflow").and_then(|v| match v {
        //                 PropValue::WorkflowMeme(w) => Some(w.clone()),
        //                 _ => None,
        //             }).unwrap_or_default();
        //             rsx! { WorkflowMemeView { workflow } }
        //         }
        //         ComponentName::WorkflowMemeExplorer => rsx! { WorkflowMemeExplorer {} },
        _ => rsx! { div {"FIXME"}},
    }
}

fn component_name(component: &ComponentName) -> &'static str {
    match component {
        ComponentName::Header => "Header",
        ComponentName::ConnectWalletModalModal => "Connect Wallet Modal",
        ComponentName::NavWalletItem => "Nav Wallet Item",
        ComponentName::ActiveAccountDropDown => "Active Account Dropdown",
        ComponentName::PingCluster => "Ping Cluster",
        ComponentName::PasswordApp => "Password App",
        ComponentName::PasswordAppHeader => "Password App Header",
        ComponentName::PasswordErrorMessage => "Password Error Message",
        ComponentName::LoginScreen => "Login Screen",
        ComponentName::PasswordMainApp => "Password Main App",
        ComponentName::PasswordList => "Password List",
        ComponentName::AddPasswordForm => "Add Password Form",
        ComponentName::PasswordDetail => "Password Detail",
        ComponentName::WelcomeScreen => "Welcome Screen",
        ComponentName::Accounts => "Accounts",
        ComponentName::ClusterSuccess => "Cluster Success",
        ComponentName::TokenAccountCard => "Token Account Card",
        ComponentName::TxCard => "Tx Card",
        ComponentName::AirdropName => "Airdrop",
        ComponentName::Clusters => "Clusters",
        ComponentName::ClusterInfo => "Cluster Info",
        ComponentName::AddClusterModal => "Add Cluster Modal",
        ComponentName::QueryCoinDialog => "Query Coin Dialog",
        ComponentName::ComponentMemeView => "Component Meme View",
        ComponentName::MemeCategoryView => "Meme Category View",
        ComponentName::ComponentMemeExplorer => "Component Meme Explorer",
        ComponentName::ConnectionButtons => "Connection Buttons",
        ComponentName::ConnectWalletFirst => "Connect Wallet First",
        ComponentName::CoreButtons => "Core Buttons",
        ComponentName::CryptoButtons => "Crypto Buttons",
        ComponentName::CryptoFrontendApp => "Crypto Frontend App",
        ComponentName::CryptoAppHeader => "Crypto App Header",
        ComponentName::CardHeader => "Card Header",
        ComponentName::InputField => "Input Field",
        ComponentName::TextAreaField => "Text Area Field",
        ComponentName::ActionButton => "Action Button",
        ComponentName::CryptoErrorMessage => "Crypto Error Message",
        ComponentName::SuccessMessage => "Success Message",
        ComponentName::EncryptionForm => "Encryption Form",
        ComponentName::DecryptionForm => "Decryption Form",
        ComponentName::Dashboard => "Dashboard",
        ComponentName::Extras => "Extras",
        ComponentName::SignMessage => "Sign Message",
        ComponentName::SignTx => "Sign Tx",
        ComponentName::SignInWithSolana => "Sign In With Solana",
        ComponentName::Footer => "Footer",
        ComponentName::GitParser2 => "Git Parser",
        ComponentName::ManagementButtons => "Management Buttons",
        ComponentName::MemeManagement => "Meme Management",
        ComponentName::Memes => "Memes",
        ComponentName::MemeCardHeader => "Meme Card Header",
        ComponentName::InputSection => "Input Section",
        ComponentName::ExpressionTypeSelector => "Expression Type Selector",
        ComponentName::ExpressionInputs => "Expression Inputs",
        ComponentName::MetadataInputs => "Metadata Inputs",
        ComponentName::CreateButton => "Create Button",
        ComponentName::SearchInput => "Search Input",
        ComponentName::ExpressionList => "Expression List",
        ComponentName::ExpressionCard => "Expression Card",
        ComponentName::CodeDisplay => "Code Display",
        ComponentName::ExpressionMetadata => "Expression Metadata",
        ComponentName::SimilaritySection => "Similarity Section",
        ComponentName::VectorSpace => "Vector Space",
        ComponentName::MemesFooter => "Memes Footer",
        ComponentName::MetaMemeOperations => "Meta Meme Operations",
        ComponentName::Notification => "Notification",
        ComponentName::Notification2 => "Notification 2",
        ComponentName::PageNotFound => "Page Not Found",
        ComponentName::QueryAccountDialogName => "Query Account Dialog",
        ComponentName::ReceiveSol => "Receive Sol",
        ComponentName::SendSol => "Send Sol",
        ComponentName::StylingAndEmojis => "Styling And Emojis",
        ComponentName::TransactionButtons => "Transaction Buttons",
        ComponentName::WikidataMemeView => "Wikidata Meme View",
        ComponentName::WikidataMemeExplorer => "Wikidata Meme Explorer",
        ComponentName::WorkflowStepView => "Workflow Step View",
        ComponentName::WorkflowMemeView => "Workflow Meme View",
        ComponentName::WorkflowMemeExplorer => "Workflow Meme Explorer",
    }
}

fn get_component_categories() -> Vec<(&'static str, Vec<ComponentName>)> {
    vec![
        (
            "Header",
            vec![
                ComponentName::Header,
                ComponentName::ConnectWalletModalModal,
                ComponentName::NavWalletItem,
                ComponentName::ActiveAccountDropDown,
                ComponentName::PingCluster,
            ],
        ),
        (
            "Password Manager",
            vec![
                ComponentName::PasswordApp,
                ComponentName::PasswordAppHeader,
                ComponentName::PasswordErrorMessage,
                ComponentName::LoginScreen,
                ComponentName::PasswordMainApp,
                ComponentName::PasswordList,
                ComponentName::AddPasswordForm,
                ComponentName::PasswordDetail,
                ComponentName::WelcomeScreen,
            ],
        ),
        (
            "Accounts",
            vec![
                ComponentName::Accounts,
                ComponentName::ClusterSuccess,
                ComponentName::TokenAccountCard,
                ComponentName::TxCard,
            ],
        ),
        (
            "Clusters",
            vec![
                ComponentName::Clusters,
                ComponentName::ClusterInfo,
                ComponentName::AddClusterModal,
            ],
        ),
        ("Airdrop", vec![ComponentName::AirdropName]),
        ("Coins", vec![ComponentName::QueryCoinDialog]),
        (
            "Memes",
            vec![
                ComponentName::ComponentMemeView,
                ComponentName::MemeCategoryView,
                ComponentName::ComponentMemeExplorer,
                ComponentName::Memes,
                ComponentName::MemeCardHeader,
                ComponentName::InputSection,
                ComponentName::ExpressionTypeSelector,
                ComponentName::ExpressionInputs,
                ComponentName::MetadataInputs,
                ComponentName::CreateButton,
                ComponentName::SearchInput,
                ComponentName::ExpressionList,
                ComponentName::ExpressionCard,
                ComponentName::CodeDisplay,
                ComponentName::ExpressionMetadata,
                ComponentName::SimilaritySection,
                ComponentName::VectorSpace,
                ComponentName::MemesFooter,
                ComponentName::WikidataMemeView,
                ComponentName::WikidataMemeExplorer,
                ComponentName::WorkflowStepView,
                ComponentName::WorkflowMemeView,
                ComponentName::WorkflowMemeExplorer,
            ],
        ),
        (
            "Buttons",
            vec![
                ComponentName::ConnectionButtons,
                ComponentName::CoreButtons,
                ComponentName::CryptoButtons,
                ComponentName::ManagementButtons,
                ComponentName::TransactionButtons,
            ],
        ),
        (
            "Crypto Frontend",
            vec![
                ComponentName::CryptoFrontendApp,
                ComponentName::CryptoAppHeader,
                ComponentName::CardHeader,
                ComponentName::InputField,
                ComponentName::TextAreaField,
                ComponentName::ActionButton,
                ComponentName::CryptoErrorMessage,
                ComponentName::SuccessMessage,
                ComponentName::EncryptionForm,
                ComponentName::DecryptionForm,
            ],
        ),
        ("Dashboard", vec![ComponentName::Dashboard]),
        (
            "Extras",
            vec![
                ComponentName::Extras,
                ComponentName::SignMessage,
                ComponentName::SignTx,
                ComponentName::SignInWithSolana,
            ],
        ),
        ("Footer", vec![ComponentName::Footer]),
        ("Git", vec![ComponentName::GitParser2]),
        (
            "Meme Management",
            vec![
                ComponentName::MemeManagement,
                ComponentName::MetaMemeOperations,
            ],
        ),
        (
            "Notifications",
            vec![ComponentName::Notification, ComponentName::Notification2],
        ),
        ("Page Not Found", vec![ComponentName::PageNotFound]),
        (
            "Transactions",
            vec![
                ComponentName::QueryAccountDialogName,
                ComponentName::ReceiveSol,
                ComponentName::SendSol,
            ],
        ),
        ("Styling", vec![ComponentName::StylingAndEmojis]),
        ("Connection", vec![ComponentName::ConnectWalletFirst]),
    ]
}

fn get_component_props(component: &ComponentName) -> Vec<(&'static str, &'static str)> {
    match component {
        ComponentName::ConnectWalletModalModal => {
            vec![("show_modal", "bool"), ("show_connecting", "bool")]
        }
        ComponentName::NavWalletItem => vec![("show_modal", "bool"), ("show_connecting", "bool")],
        ComponentName::ActiveAccountDropDown => {
            vec![("show_modal", "bool"), ("shortened_address", "string")]
        }
        ComponentName::PasswordAppHeader => vec![("app_state", "password_app_state")],
        ComponentName::PasswordErrorMessage => vec![("message", "string")],
        ComponentName::LoginScreen => vec![("app_state", "password_app_state")],
        ComponentName::PasswordMainApp => vec![("app_state", "password_app_state")],
        ComponentName::PasswordList => vec![("app_state", "password_app_state")],
        ComponentName::AddPasswordForm => vec![("app_state", "password_app_state")],
        ComponentName::PasswordDetail => vec![("app_state", "password_app_state")],
        ComponentName::ClusterSuccess => {
            vec![("address", "string"), ("shortened_address", "string")]
        }
        ComponentName::TokenAccountCard => vec![("mint", "string"), ("ata_address", "string")],
        ComponentName::TxCard => vec![("tx", "string"), ("timestamp", "string")],
        ComponentName::AirdropName => vec![("show_airdrop_modal", "bool")],
        ComponentName::ClusterInfo => vec![("connections", "pub use_connections")],
        ComponentName::AddClusterModal => vec![
            ("show_add_entry_modal", "bool"),
            ("connections", "pub use_connections"),
        ],
        ComponentName::ComponentMemeView => vec![("component_meme", "component_meme")],
        ComponentName::MemeCategoryView => vec![("category", "meme_category")],
        ComponentName::ConnectionButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CoreButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CryptoButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CardHeader => vec![("title", "string")],
        ComponentName::InputField => vec![("label", "string")],
        ComponentName::TextAreaField => vec![("label", "string")],
        ComponentName::ActionButton => vec![("label", "string")],
        ComponentName::CryptoErrorMessage => vec![("message", "string")],
        ComponentName::SuccessMessage => vec![("message", "string")],
        ComponentName::MemeCardHeader => {
            vec![("expression", "string"), ("state", "password_app_state")]
        }
        ComponentName::InputSection => vec![("state", "password_app_state")],
        ComponentName::ExpressionTypeSelector => vec![("state", "password_app_state")],
        ComponentName::ExpressionInputs => vec![("state", "password_app_state")],
        ComponentName::MetadataInputs => vec![("state", "password_app_state")],
        ComponentName::CreateButton => vec![("state", "password_app_state")],
        ComponentName::SearchInput => vec![("state", "password_app_state")],
        ComponentName::ExpressionList => vec![("state", "password_app_state")],
        ComponentName::ExpressionCard => {
            vec![("expression", "string"), ("state", "password_app_state")]
        }
        ComponentName::CodeDisplay => vec![("expression", "string")],
        ComponentName::ExpressionMetadata => vec![("expression", "string")],
        ComponentName::SimilaritySection => {
            vec![("expression", "string"), ("state", "password_app_state")]
        }
        ComponentName::VectorSpace => vec![("state", "password_app_state")],
        ComponentName::PageNotFound => vec![("route", "string_vec")],
        ComponentName::QueryAccountDialogName => vec![("show_query_dialog", "bool")],
        ComponentName::ReceiveSol => vec![("show_receive_modal", "bool")],
        ComponentName::SendSol => vec![("show_send_modal", "bool")],
        ComponentName::TransactionButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::WikidataMemeView => vec![("meme", "wikidata_meme")],
        ComponentName::WorkflowStepView => vec![("step", "workflow_step")],
        ComponentName::WorkflowMemeView => vec![("workflow", "workflow_meme")],
        _ => vec![],
    }
}
