use dioxus::prelude::*;


use wallet_adapter::{ConnectionInfo, WalletAdapter};
use crate::{model::{cluster_store::ClusterStore, AccountState, NotificationInfo}, model::ClusterNetState};
use std::collections::VecDeque;


pub(crate) static WALLET_ADAPTER: GlobalSignal<WalletAdapter> =
    Signal::global(|| WalletAdapter::init().unwrap());

pub(crate) static CLUSTER_STORAGE: GlobalSignal<ClusterStore> =
    Signal::global(|| ClusterStore::new(Vec::default()));

pub(crate) static GLOBAL_MESSAGE: GlobalSignal<VecDeque<NotificationInfo>> =
    Signal::global(|| VecDeque::default());

pub(crate) static ACCOUNT_STATE: GlobalSignal<AccountState> =
    Signal::global(|| AccountState::default());

pub(crate) static LOADING: GlobalSignal<Option<()>> = Signal::global(|| Option::default());

pub(crate) static CLUSTER_NET_STATE: GlobalSignal<ClusterNetState> =
    Signal::global(|| ClusterNetState::default());

pub(crate) static ACTIVE_CONNECTION: GlobalSignal<ConnectionInfo> =
    Signal::global(|| ConnectionInfo::default());