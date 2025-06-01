use dioxus::prelude::*;

use crate::{
    fetch_parser::send_sol_req, Loader, NotificationInfo, SendSvg, UserSvg, ACTIVE_CONNECTION,
    GLOBAL_MESSAGE,
};

#[component]
pub fn SendSol(show_send_modal: Signal<bool>) -> Element {
    let mut loading = use_signal(|| false);
    let mut address = use_signal(|| Option::default());
    let mut lamports = use_signal(|| 0u64);

    let mut public_key_bytes = [0u8; 32];

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        public_key_bytes = wallet_account.public_key();
    }

    if *show_send_modal.read() {
        rsx! {
            div { class: "fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick:move|_|{show_send_modal.set(false)},
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
                        div { class: "flex text-true-blue dark:text-white w-full items-center justify-center",
                            span{class:"w-[30px] flex mb-10 mr-2",{SendSvg()}}
                            span{class:"flex mb-10 mr-2 text-3xl","Send Lamports"}
                        }
                        div { class: "flex flex-col w-3/5 mt-2 rounded-3x",
                            div { class: "flex w-full items-center rounded-xl p-1 bg-transparent",
                                div { class: "shrink-0 select-none text-base text-true-blue dark:text-white sm:text-sm/6 mb-10","SOL" }
                                input {
                                    oninput: move |event| {
                                        if let Ok(data) = event.data.value().as_str().parse::<u64>(){
                                            lamports.set(data);
                                        }
                                    },
                                    class: "focus:outline-none mb-10 bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id: "lamports",
                                    min: "0",
                                    name: "lamports",
                                    placeholder: "50000",
                                    r#type: "number",
                                }
                            }
                            div { class: "flex items-center rounded-xl p-1 bg-transparent",
                                div { class: "shrink-0 select-none text-base text-gray-500 sm:text-sm/6", span {class:"flex w-[20px]", {UserSvg()}  } }
                                input {
                                    oninput: move |event| {
                                        let data = event.data.value();
                                        address.write().replace(data);
                                    },
                                    class: "w-full focus:outline-none bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id: "address",
                                    min: "0",
                                    name: "address",
                                    r#type: "text",
                                    placeholder: "Enter Recipient Address",
                                }
                            }
                        }
                        div { class: "flex w-full items-center justify-center mt-4",
                            button {disabled:*loading.read() && address.read().is_none(),
                                onclick:move|_|{
                                    spawn(async move {
                                        loading.set(true);

                                        if let Err(error) = send_sol_req(&address.read().as_ref().cloned().unwrap_or_default(),
                                            *lamports.read(),
                                            public_key_bytes
                                        ).await {
                                            GLOBAL_MESSAGE.write().push_back(
                                                NotificationInfo::error(format!("SEND SOL ERROR: {:?}", error))
                                            );
                                        }

                                        loading.set(false);
                                        show_send_modal.set(false);

                                        GLOBAL_MESSAGE.write().push_back(NotificationInfo::new("Sent"));

                                        show_send_modal.set(false);
                                    });
                                },
                                class:if *loading.read() {""}else{"bg-true-blue hover:bg-cobalt-blue"},
                                class:"flex text-sm mb-10  text-white text-black px-4 py-1 items-center justify-center rounded-full",
                                if *loading.read() {
                                    {Loader()} span {class:"text-true-blue", "Sending SOL..."}
                                }else {
                                    "SEND SOL"
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
