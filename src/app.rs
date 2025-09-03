use crate::header::Header;
use dioxus::prelude::*;

use crate::{
    model::NotificationInfo,
    views::{footer::Footer, notification::Notification, page_not_found::PageNotFound},
};

use crate::{model::AccountState, views::connections::Connections};

// FIXME this is used, [link { rel: "icon", href: FAVICON }](https://github.com/meta-introspector/solfunmeme-dioxus/blob/46e454980e624cf09ea65a39739708ba04e75f70/src/playground/app.rs#L114-L115)
#[allow(dead_code)]
pub const FAVICON: Asset = asset!("/assets/favicon.png");

// FIXME this is used, [link { rel: "stylesheet", href: TAILWIND_CSS }]( [TAILWIND_CSS](https://github.com/meta-introspector/solfunmeme-dioxus/blob/46e454980e624cf09ea65a39739708ba04e75f70/src/playground/app.rs#L113)
#[allow(dead_code)]
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

use crate::model::storage::{ACCOUNT_STATE, ACTIVE_CONNECTION, GLOBAL_MESSAGE, WALLET_ADAPTER};
//use crate::model::storage::{ClusterStore, NotificationInfo};
//use crate::model::adaptercluster::AdapterCluster;
use crate::views::{
    accounts::Accounts, clusters::Clusters, dashboard::Dashboard, extras::Extras,
    source_browser::SourceBrowser, emoji_matrix_view::EmojiMatrixView,
};

#[component]
pub(crate) fn MainApp() -> Element {
    let wallet_event_listener = WALLET_ADAPTER.read().events().clone();

    // FIXME: This is commented out because the `ClusterStore` is not implemented yet.
    // The `ClusterStore` should be implemented to manage clusters.

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
    let fi = FAVICON.to_string();
    let tw = TAILWIND_CSS.to_string();
    rsx! {
        document::Link { rel: "icon", href: fi }
        document::Link { rel: "stylesheet", href: tw }
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

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route { 
    #[layout(Header)]
        #[route("/")]
        Dashboard(),
        #[route("/accounts")]
        Accounts(),
    //#[route("/test")]
        //TestMenuApp(),
        #[route("/clusters")]
        Clusters(),
        #[route("/extras")]
        Extras(),
        #[route("/source")]
        SourceBrowser(),
        #[route("/emoji-matrix")]
        EmojiMatrixView(),
        // #[feature ("git2")]
        // #[route("/git-parser")]
        // GitParser2(),
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
