use dioxus::prelude::*;

use crate::{
    utils::copied_address, CopySvg, NotificationInfo, ReceiveSvg, ACTIVE_CONNECTION, GLOBAL_MESSAGE,
};

#[component]
pub fn ReceiveSol(show_receive_modal: Signal<bool>) -> Element {
    let mut address = String::default();
    let mut shortened_address = String::default();

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        address = wallet_account.address().to_string();
        shortened_address = wallet_account
            .shorten_address()
            .unwrap_or_default()
            .to_string();
    }

    let qrcode = if let Ok(qr) = crate::address_qrcode(&address) {
        qr
    } else {
        rsx! {
            div { class:"text-black dark:text-white", }
        }
    };

    let address_inner = address.clone();

    if *show_receive_modal.read() {
        rsx! {
            div {
                class: "fixed overflow-y-hidden min-h-screen z-10 flex flex-col w-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick:move|_|{show_receive_modal.set(false)},
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
                    div { class: "overflow-y-scroll w-full mb-5 items-center justify-center flex flex-col",
                        div { class: "flex w-full items-center justify-center text-2xl", span{class:"w-[30px] mr-2",{ReceiveSvg()}} "Receive SOL" }
                        div{class:"mb-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue cursor-pointer",
                            onclick:move|_| {
                                let address_inner = address_inner.clone();

                                spawn(async move {
                                    if let Err(error) = copied_address(&address_inner).await {
                                        GLOBAL_MESSAGE.write().push_back(NotificationInfo::error(format!("COPY ERROR: {:?}", error)));
                                    } else {
                                        GLOBAL_MESSAGE.write().push_back(NotificationInfo::new("Copied to clipboard"));
                                    }
                                });
                            },
                            div { class:"flex justify-left items-center px-2 py-1 text-white rounded-full ",
                                span {class:"flex p-2 w-[30px]", {CopySvg()} }
                                span { {shortened_address} }
                            }
                        }
                        div{class:"w-[200px] rounded-xl flex mt-5 mb-5 bg-white", {qrcode}}
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
