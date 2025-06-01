use dioxus::prelude::*;

use crate::{
    ErrorSvg, MintSvg, NotificationInfo, SiwsSvg, TimestampSvg, UserSvg, ACTIVE_CONNECTION,
    GLOBAL_MESSAGE, WALLET_ADAPTER,
};
use wallet_adapter::SigninInput;

#[component]
pub fn SignInWithSolana() -> Element {
    let community = "JamiiDAO";
    let user_id = "X48K48";

    let mut address = String::default();
    let mut public_key = [0u8; 32];
    let mut solana_signin = false;

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        address = wallet_account.address().to_string();
        public_key = wallet_account.public_key();
        solana_signin = wallet_account.solana_signin();
    }

    // Check if wallet supported SIWS
    let (signin_input, nonce, public_key) = if solana_signin {
        let mut signin_input = SigninInput::new();
        signin_input.set_nonce();
        let nonce = signin_input.nonce().unwrap().clone();

        let message = String::new()
            + "Community: "
            + community
            + "USER ID: "
            + user_id
            + "SESSION: "
            + nonce.as_str();

        signin_input
            .set_domain(WALLET_ADAPTER.read().window())
            .unwrap()
            .set_statement(&message)
            .set_chain_id(wallet_adapter::Cluster::DevNet)
            // NOTE: Some wallets require this field or the wallet adapter
            // will return an error `MessageResponseMismatch` which is as
            // a result of the sent message not corresponding with the signed message
            .set_address(&address)
            .unwrap();

        (signin_input, nonce, public_key)
    } else {
        (SigninInput::default(), String::default(), [0u8; 32])
    };

    rsx! {
        div { class:"flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none",
            div { class:"flex w-full flex-col",
                div{class:"flex w-full text-true-blue items-center justify-center text-6xl",
                    span{class:"flex w-[40px]",{SiwsSvg()}}
                    "SIWS"
                }
                div{class:"flex text-true-blue w-full justify-center text-sm", "Sign In With Solana"}
                div{class:"flex mt-5 flex-col w-full justify-center text-lg",
                    div{class:"flex w-full text-lg", span{class:"flex w-[20px] mr-2", {MintSvg()}} "Community: " {community}}
                    div{class:"flex w-full text-lg", span{class:"flex w-[20px] mr-2", {UserSvg()}} "User ID: " {user_id} }
                    div{class:"flex  text-lg mt-5 text-true-blue dark:text-blue-yonder  w-full",
                       if solana_signin {
                            div{class:"flex items-center w-full",
                                span{class:"flex w-[25px]", {TimestampSvg()}}
                                {truncate_nonce(&nonce)}
                            }
                       }else {
                            span {class:"flex text-sm w-[20px] mr-1", {ErrorSvg()}  }
                            "WALLET DOES NOT SUPPORT SIWS"
                       }
                    }

                    if solana_signin {
                        div {class:"flex w-full justify-center items-center",
                            button {
                                onclick:move|_|{
                                        let signin_input = signin_input.clone();
                                        spawn(async move {
                                            if let Err(error) = WALLET_ADAPTER.read().sign_in(&signin_input, public_key)
                                            .await{
                                                GLOBAL_MESSAGE.write().push_back(
                                                        NotificationInfo::error(
                                                            format!("SIWS ERROR: {error:?}")
                                                        )
                                                    );
                                            }else {
                                                GLOBAL_MESSAGE.write().push_back(
                                                        NotificationInfo::new("SIWS Successful")
                                                    );
                                            }
                                        });
                                },
                                class: "bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                                "SIGN IN"
                            }
                        }
                    }
                }

            }
        }
    }
}

fn truncate_nonce(value: &str) -> String {
    let value = String::from("SESSION: ") + value;

    if value.len() <= 20 {
        value.to_string()
    } else {
        value.chars().take(20).collect::<String>() + "..."
    }
}
