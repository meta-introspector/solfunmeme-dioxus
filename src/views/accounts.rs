use dioxus::prelude::*;
use wallet_adapter::Cluster;

use crate::{
    format_timestamp, link_target_blank, trunk_cluster_name,
    utils::{format_address_url, format_tx_url, get_cluster_svg},
    views::{ReceiveSol, SendSol},
    Airdrop, AirdropSvg, AtaSvg, BalanceSvg, CheckSvg, ErrorSvg, Loader, MintSvg, NotificationInfo,
    ReceiveSvg, SendSvg, SignatureSvg, SignaturesResponse, TimestampSvg, TokenAccountResponse,
    UserSvg, WalletSvg, ACCOUNT_STATE, ACTIVE_CONNECTION, CLUSTER_NET_STATE, CLUSTER_STORAGE,
    GLOBAL_MESSAGE, LOADING,
};

use super::ConnectWalletFirst;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ClusterNetState {
    Success,
    #[default]
    Waiting,
    Failure,
}

#[derive(Debug, Default, PartialEq)]
pub struct AccountState {
    pub balance: String,
    pub token_accounts: Vec<TokenAccountResponse>,
    pub transactions: Vec<SignaturesResponse>,
}

impl AccountState {
    pub fn token_accounts_is_empty(&self) -> bool {
        self.token_accounts.is_empty()
    }

    pub fn transactions_is_empty(&self) -> bool {
        self.token_accounts.is_empty()
    }

    pub fn token_accounts(&self) -> &[TokenAccountResponse] {
        self.token_accounts.as_slice()
    }

    pub fn transactions(&self) -> &[SignaturesResponse] {
        self.transactions.as_slice()
    }
}

#[component]
pub fn Accounts() -> Element {
    let mut address = String::default();
    let mut public_key_bytes = [0u8; 32];
    let mut shortened_address = String::default();

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        address = wallet_account.address().to_string();
        shortened_address = wallet_account
            .shorten_address()
            .unwrap_or_default()
            .to_string();
        public_key_bytes = wallet_account.public_key();
    }

    if ACTIVE_CONNECTION.read().connected_account().is_ok() {
        if *CLUSTER_NET_STATE.read() == ClusterNetState::Success {
            rsx! {
                ClusterSuccess {
                    address,
                    shortened_address,
                    public_key_bytes
                }
            }
        } else if *CLUSTER_NET_STATE.read() == ClusterNetState::Waiting {
            rsx! {"Loading account info..."}
        } else {
            rsx! {"CLUSTER NETWORK UNREACHABLE"}
        }
    } else {
        rsx! {ConnectWalletFirst {}}
    }
}

#[component]
fn ClusterSuccess(
    address: String,
    shortened_address: String,
    public_key_bytes: [u8; 32],
) -> Element {
    let mut show_send_modal = use_signal(|| false);
    let mut show_airdrop_modal = use_signal(|| false);
    let mut show_receive_modal = use_signal(|| false);
    let mut refreshing = use_signal(|| false);

    let check_balance = || {
        let balance = ACCOUNT_STATE
            .read()
            .balance
            .as_str()
            .parse::<f64>()
            .unwrap_or_default();

        balance == f64::default()
    };

    let clone_address = address.clone();

    use_effect(move || {
        *ACCOUNT_STATE.write() = AccountState::default();
        let clone_address = clone_address.clone();

        spawn(async move {
            fetch_account_state(None, None, &clone_address).await;
        });
    });

    rsx! {
        div {class:"flex w-full h-full mt-4 mb-10 flex-col items-center",
            div {
                class:"shadow-sm p-5 w-full flex flex-col items-center mb-10 justify-center",
                div{class:"text-center w-full",
                    if LOADING.read().is_none(){
                        span{class:"text-3xl", {ACCOUNT_STATE.read().balance.as_str()} " SOL"}
                    }else {
                        span{class:"mr-2", {Loader()}}  span{class:"text-sm","Loading Balance..."}
                    }
                }
                div { class:"flex w-full items-center justify-center",
                    span {class:"flex w-[20px] mr-1", {WalletSvg()}}
                    {link_target_blank(&format_address_url(&address), &shortened_address)}
                }
                div {class:"w-full flex gap-4 flex-wrap items-center justify-center",
                    if !check_balance() {
                        button {
                            onclick:move|_|{show_send_modal.set(true)},
                            class:"flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                            span{class:"w-[25px] flex mr-1", {SendSvg()}} "Send"
                        }
                    }
                    button {
                        onclick:move|_|{show_receive_modal.set(true)},
                        class:"flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                        span{class:"w-[25px] flex mr-1", {ReceiveSvg()}} "Receive"
                    }
                    if CLUSTER_STORAGE.read().active_cluster().cluster() != Cluster::MainNet{
                        button {
                            onclick:move|_|{show_airdrop_modal.set(true)},
                            class:"flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                            span{class:"w-[25px] flex mr-1", {AirdropSvg()}} "Airdrop"
                        }
                    }
                    button{
                        onclick:move|_|{
                            let address = address.clone();
                            spawn(async move {
                                refreshing.set(true);
                                fetch_account_state(Some("REFRESHED ACCOUNTS"), Some("REFRESH ERROR"), &address).await ;
                                refreshing.set(false);
                            });

                        },
                        disabled:*refreshing.read(),
                        class:if *refreshing.read(){"dark:bg-rich-black bg-white"} else {"bg-true-blue hover:bg-cobalt-blue"},
                        class:"flex items-center text-sm text-white px-5 py-2 mt-5 rounded-full",
                        span{class:"w-[25px] flex mr-1", {WalletSvg()}} if *refreshing.read() {
                            {Loader()}
                        }else {"Refresh"}
                    }
                }
            }
            div { class:"flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center",
                div{class:"flex items-center text-true-blue dark:text-white justify-center",
                    span{class:"w-[30px] mr-1", {AtaSvg()}}

                    if LOADING.read().is_none(){
                        if ACCOUNT_STATE.read().token_accounts_is_empty(){
                            span{class:"text-sm", "No Token Accounts Found" }
                        }else {
                            "Token Accounts"
                        }
                    }
                    else {
                        span{class:"mr-2", {Loader()}} "Loading Token Accounts..."
                    }
                }

                for token_account in ACCOUNT_STATE.read().token_accounts() {
                    TokenAccountCard{
                        mint: token_account.mint(),
                        ata_address: token_account.ata_address(),
                        token_balance: token_account.balance(),
                        state: token_account.state()
                    }
                }
            }
            div { class:"flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center",
                div{class:"flex mb-5 items-center text-true-blue dark:text-white justify-center",
                    span{class:"w-[30px] mr-1", {SignatureSvg()}}
                    if LOADING.read().is_none() {
                        if ACCOUNT_STATE.read().transactions_is_empty(){
                            span{class:"text-sm", "No Transactions Found"}
                        }else {
                            "Transactions"
                        }
                    }else {
                        span{class:"mr-2", {Loader()}} "Loading Transactions..."
                    }
                }
                div { class:"flex w-full gap-4 flex-wrap items-center justify-center",
                    for tx in ACCOUNT_STATE.read().transactions() {
                        TxCard {
                            tx: tx.signature.clone(),
                            timestamp: tx.block_time,
                            state: tx.confirmation_status.clone(),
                            succeeded: tx.err.is_none(), address: &address}
                    }
                }
            }
        }

        SendSol{show_send_modal}
        ReceiveSol{show_receive_modal}
        if CLUSTER_STORAGE.read().active_cluster().cluster() != Cluster::MainNet{
            Airdrop{show_airdrop_modal}
        }
    }
}

#[component]
fn TokenAccountCard(
    mint: String,
    ata_address: String,
    token_balance: String,
    state: String,
) -> Element {
    let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();
    let cluster_image = get_cluster_svg(cluster);

    let cluster_name = trunk_cluster_name(CLUSTER_STORAGE.read().active_cluster().name());

    let shortened_mint_address = wallet_adapter::Utils::shorten_base58(&mint)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Mint Address"));
    let shortened_ata_address = wallet_adapter::Utils::shorten_base58(&ata_address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Owner Address"));

    rsx! {
        div { class: "flex flex-col items-start p-4 w-[250px] m-5 rounded-lg bg-true-blue",
            div {class:"flex w-full items-center",
                span{class:"w-[28px] pr-2", {cluster_image()}}
                h5 { class: "flex text-2xl",
                    {cluster_name}
                }
            }
            div { class: "flex flex-col w-full",
                div { class: "flex w-full items-start flex-col mt-2.5",
                    div {class:"w-full justify-between  flex",
                        div { class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                            {cluster.chain()}
                        }
                        div { class: "bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                            {state}
                        }
                    }

                    div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",

                        div {class:"flex items-center",
                            div{class:"w-1/5", {MintSvg()} }
                            div{class:"w-4/5 flex text-sm pl-2", {link_target_blank(&format_address_url(&mint), &shortened_mint_address) } }
                        }
                    }

                    div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",

                        div {class:"flex items-center",
                            div{class:"w-1/5", {AtaSvg()} }
                            div{class:"w-4/5 flex text-sm pl-2", {link_target_blank(&format_address_url(&ata_address), &shortened_ata_address) } }
                        }
                    }

                    div { class: "text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between",
                        div {class:"flex items-center",
                             div{class:"w-2/5", {BalanceSvg()} }
                             div{class:"w-3/5 flex text-[12px] p-1", {token_balance} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TxCard(
    tx: String,
    timestamp: Option<i64>,
    state: Option<String>,
    succeeded: bool,
    address: String,
) -> Element {
    let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();
    let cluster_image = get_cluster_svg(cluster);

    let cluster_name = trunk_cluster_name(CLUSTER_STORAGE.read().active_cluster().name());

    let shortened_address = wallet_adapter::Utils::shorten_base58(&address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let shortened_tx = wallet_adapter::Utils::shorten_base58(&tx)
        .map(|tx| tx.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let succeeded = if succeeded { CheckSvg() } else { ErrorSvg() };

    rsx! {
        div { class: "flex rounded-lg flex-col items-start p-5 w-[250px] bg-true-blue",
            div {class:"flex items-center",
                span{class:"w-[28px] pr-2", {cluster_image()}}
                h5 { class: "text-2xl font-semibold tracking-tight",
                    {cluster_name}
                }
            }
            div { class: "flex flex-col w-full",
                div { class: "flex w-full items-start flex-col mt-2.5",
                    div {class:"w-full justify-between items-start  flex",
                        div { class: "flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                            {cluster.chain()}
                        }
                        if let Some(state_inner) = state {
                            div { class: "flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                {state_inner.to_uppercase()}
                            }
                        }
                    }

                    div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",
                        div {class:"flex items-center",
                            span { class: "w-[20px]", {UserSvg()} }
                            span { class: "flex text-sm pl-2",
                            {link_target_blank(&format_address_url(&address), &shortened_address)}
                            }
                        }
                    }

                    div { class: "text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between",

                        div {class:"flex items-center",
                            span { class: "w-[20px]", {SignatureSvg()} }
                            span { class: "flex text-sm pl-2", {link_target_blank(&format_tx_url(&tx), &shortened_tx) } }
                            div {class:"flex items-center",
                                span { class: "ml-2 w-[15px]", {succeeded} }
                            }
                        }
                    }

                    if let Some(timestamp) = timestamp {
                        div { class: "text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between",
                            div {class:"flex items-center",
                                span { class: "w-[30px]", {TimestampSvg()} }
                                span { class: "flex text-[12px] p-1", {format_timestamp(timestamp)} }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn fetch_account_state(
    success_msg: Option<&str>,
    error_msg: Option<&str>,
    address: &str,
) {
    LOADING.write().replace(());

    match crate::accounts_runner(address).await {
        Ok(value) => {
            *ACCOUNT_STATE.write() = value;
            if let Some(success_msg) = success_msg {
                GLOBAL_MESSAGE
                    .write()
                    .push_back(NotificationInfo::new(success_msg));
            }
        }
        Err(error) => {
            if let Some(error_msg) = error_msg {
                GLOBAL_MESSAGE
                    .write()
                    .push_back(NotificationInfo::error(format!("{error_msg}: {:?}", error)));
            }
        }
    }

    LOADING.write().take();
}
