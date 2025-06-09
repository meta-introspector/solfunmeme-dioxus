
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

use crate::{
    model::NotificationInfo, Footer, Header
};

use crate::views::connections::Connections;
//use crate::model::AdapterCluster;
use crate::model::{AccountState};
const FAVICON: Asset = asset!("/assets/favicon.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

use crate::model::storage::WALLET_ADAPTER;
use crate::model::storage::{GLOBAL_MESSAGE, ACCOUNT_STATE, ACTIVE_CONNECTION};
//use crate::model::storage::{ClusterStore, NotificationInfo};
//use crate::model::adaptercluster::AdapterCluster;
use crate::views::dashboard::Dashboard;
use crate::views::accounts::Accounts;
use crate::views::clusters::Clusters;
use crate::views::extras::Extras;


#[component]
pub(crate) fn App() -> Element {
    let wallet_event_listener = WALLET_ADAPTER.read().events().clone();

    // let clusters = vec![
    //     AdapterCluster::devnet(),
    //     AdapterCluster::mainnet(),
    //     AdapterCluster::testnet(),
    //     AdapterCluster::localnet(),
    // ];
    //    if CLUSTER_STORAGE.write().add_clusters(clusters).is_err() {}     // FIXME: add default clusters


    spawn(async move {
        while let Ok(wallet_event) = wallet_event_listener.recv().await {
            *ACCOUNT_STATE.write() = AccountState::default();

            let connection_info = (*WALLET_ADAPTER.read().connection_info().await).clone();
            *ACTIVE_CONNECTION.write() = connection_info;

            GLOBAL_MESSAGE
                .write()
                .push_back(NotificationInfo::new(wallet_event));
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title {"Rust Wallet Adapter"}

        div { class: "w-full flex min-h-screen font-[sans-serif] dark:bg-rich-black bg-white text-black dark:text-white",

            Notification {}


            div { class: "flex flex-col w-full min-h-full justify-between items-center",
                Router::<Route> {}
                 Connections {}
                Footer{}
            }
        }
    }
}

#[component]
fn Notification() -> Element {
    if GLOBAL_MESSAGE.read().is_empty() {
        return rsx! {};
    }

    let message_index = |key: u32| {
        let messages = GLOBAL_MESSAGE.read();
        let found_message = messages
            .iter()
            .enumerate()
            .find(|(_, value)| value.key() == key)
            .map(|(index, _value)| index);
        drop(messages);

        found_message
    };

    let timer_callback = |secs: u32, key: u32| {
        // Start a timeout for each notification
        spawn(async move {
            let timeout = Timeout::new(secs * 1000, move || {
                message_index(key).map(|index| GLOBAL_MESSAGE.write().remove(index));
            });
            timeout.forget();
        });
    };

    let mut key = Some(0u32);

    rsx! {
        div {
            class: "cursor-pointer fixed z-1000 top-4 right-4 flex flex-col space-y-2 min-w-[300px] shadow-lg",
            for notification_info in GLOBAL_MESSAGE.read().clone().iter() {
                {key.replace(notification_info.key());}
                {timer_callback(notification_info.secs(), notification_info.key())}

                div {
                    onclick:move|_|{
                        if let Some(key_inner) = key {
                            message_index(key_inner).map(|index| GLOBAL_MESSAGE.write().remove(index));
                        }
                        key.take();
                    },
                    key: "{notification_info.key()}",
                    class: "flex border dark:border-none items-center opacity-0 translate-y-4 animate-fade-in w-full max-w-xs p-2 space-x-2 text-gray-600 bg-white divide-x divide-gray-200 rounded-lg shadow-sm dark:text-gray-400 dark:divide-gray-700 dark:bg-gray-800",
                    div { class:"flex w-[30px]",
                        svg {
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "m10 20h4a2 2 0 0 1 -4 0zm8-4v-6a6 6 0 0 0 -5-5.91v-1.09a1 1 0 0 0 -2 0v1.09a6 6 0 0 0 -5 5.91v6l-2 2h16z",
                                fill: "#0060df",
                            }
                        }
                    }
                    div { class: "ps-4 text-sm font-normal", "{notification_info.message()}" }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route { 
    #[layout(Header)]
        #[route("/")]
        Dashboard(),
        #[route("/accounts")]
        Accounts(),
        #[route("/clusters")]
        Clusters(),
        #[route("/extras")]
        Extras(),
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
