use dioxus::prelude::*;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use crate::{
    fetch_parser::request_airdrop, AirdropSvg, Loader, NotificationInfo, ACTIVE_CONNECTION,
    GLOBAL_MESSAGE,
};

#[component]
pub fn Airdrop(show_airdrop_modal: Signal<bool>) -> Element {
    let mut loading = use_signal(|| false);
    let mut lamports = use_signal(|| 0u64);

    let mut address = String::default();

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        address = wallet_account.address().to_string();
    }

    if *show_airdrop_modal.read() {
        rsx! {
            div { class: "fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#0b0414] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick: move |_| {
                                show_airdrop_modal.set(false);
                            },
                            class: "wallet-adapter-modal-button-close w-[30px] items-center justify-center",
                            "data-dioxus-id": "65",
                            svg {
                                fill: "none",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "m15 9.00004-6 5.99996m6 0-6-5.99996m3 11.99996c4.9706 0 9-4.0294 9-9 0-4.97056-4.0294-9-9-9-4.97056 0-9 4.02944-9 9 0 4.9706 4.02944 9 9 9z",
                                    stroke: "#a6c1ee",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                }
                            }
                        }
                    }
                    div { class: "overflow-y-scroll max-h-[90%] w-full mb-5 items-center justify-center flex flex-col",
                        div { class: "flex w-full items-center justify-center text-2xl", span{class:"w-[50px] mr-2",{AirdropSvg()}} "Request Airdrop" }
                        div { class: "mt-2 rounded-3x",
                            div { class: "flex items-center rounded-xl p-1 bg-transparent",
                                div { class: "shrink-0 select-none text-base text-gray-500 sm:text-sm/6","SOL" }
                                input {
                                    oninput: move |event| {
                                        let data = event.data.value().as_str().parse::<u64>().unwrap_or(1);
                                        lamports.set(data * LAMPORTS_PER_SOL);
                                    },
                                    class: "focus:outline-none bg-transparent border-b-2 border-white block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id: "airdrop",
                                    min: "0",
                                    name: "airdrop",
                                    placeholder: "2",
                                    r#type: "number",
                                    value: "2",
                                }
                            }
                        }
                        div { class: "flex-w-full items-center justify-center mt-4",
                            button {disabled:*loading.read(),
                                onclick:move|_|{
                                    let address = address.clone();
                                    spawn(async move {
                                        loading.set(true);

                                        if request_airdrop(*lamports.read(), &address).await.is_err() {
                                            GLOBAL_MESSAGE.write().push_back(
                                                NotificationInfo::error("REQUEST AIRDROP ERROR: You might have reached your daily limit.")
                                            );
                                        }else {
                                            GLOBAL_MESSAGE.write().push_back(
                                                NotificationInfo::new("REQUESTED AIRDROP")
                                            );
                                        }

                                        show_airdrop_modal.set(false);
                                    });
                                },
                                class:if *loading.read() {"bg-rich-black"}else{"bg-true-blue hover:bg-cobalt-blue"},
                                class:"flex text-sm  text-white px-4 py-2 items-center justify-center rounded-full",
                                if *loading.read() {
                                    {Loader()} "Requesting Airdrop..."
                                }else {
                                    "REQUEST AIRDROP"
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
