#![allow(non_snake_case)]

use dioxus::prelude::*;

mod views;
use views::*;

mod model;
use model::*;

mod header;
use header::*;

mod utils;
use utils::*;

mod fetch_parser;
use fetch_parser::*;

mod svg_assets;
pub(crate) use svg_assets::*;

//mod dioxus_adapter;
//pub(crate) use dioxus_adapter::*;

mod fetch_util;
pub(crate) use fetch_util::*;

mod app;
pub(crate) use app::LOGO;
pub(crate) use app::Route;


//use dioxus::prelude::*;
//use gloo_timers::callback::Timeout;

use crate::{
    model::NotificationInfo, Footer, Header
};

//use crate::views::connections::Connections;
//use crate::model::AdapterCluster;
use crate::model::{AccountState};
//const FAVICON: Asset = asset!("/assets/favicon.png");
//const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
//pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

//use crate::model::storage::WALLET_ADAPTER;
//use crate::model::storage::{GLOBAL_MESSAGE, ACCOUNT_STATE, ACTIVE_CONNECTION};
//use crate::model::storage::{ClusterStore, NotificationInfo};
//use crate::model::adaptercluster::AdapterCluster;
//use crate::views::dashboard::Dashboard;
//use crate::views::accounts::Accounts;
//use crate::views::clusters::Clusters;
//use crate::views::extras::Extras;


mod password_manager;
use password_manager::App;





fn main() {
    launch(App);
}
