use dioxus::prelude::*;

use crate::{NotificationInfo, SignMessageSvg, ACTIVE_CONNECTION, GLOBAL_MESSAGE, WALLET_ADAPTER};

#[component]
pub fn SignMessage() -> Element {
    let message = "Solana Foundation is awesome!";

    let mut solana_signmessage = false;

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        solana_signmessage = wallet_account.solana_sign_message();
    }

    rsx! {
        div { class:"flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 justify-around rounded-lg dark:shadow-2xl shadow-sm border dark:border-none",
            div {class:"w-full flex flex-col items-center text-center text-true-blue justify-center mb-10",
                div{class:"w-[80px] flex flex-col", {SignMessageSvg()}}
                div{class:"w-full text-sm", "Sign Message"}
            }
            div { class:"text-lg text-center", {message}}

            div { class:"flex items-center justify-center",
                if solana_signmessage {
                    button{
                        class: "bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                        onclick: move |_| {
                            spawn(async move {
                                if let Err(error) = WALLET_ADAPTER.read().sign_message(message.as_bytes()).await{
                                    GLOBAL_MESSAGE.write().push_back(
                                        NotificationInfo::error(
                                            format!("SIGN MESSAGE ERROR: {error:?}")
                                        )
                                    );
                                }else {
                                    GLOBAL_MESSAGE.write().push_back(
                                        NotificationInfo::new("Sign Message Successful")
                                    );
                                }
                            });
                        },
                        "SIGN MESSAGE"
                    }
                }else {
                    div{
                        class:"w-full items-center justify-center",
                        "SIGN MESSAGE UNSUPPORTED"
                    }
                }
            }
        }
    }
}
