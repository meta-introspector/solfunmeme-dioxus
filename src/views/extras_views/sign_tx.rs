use dioxus::prelude::*;
use solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction};
use wallet_adapter::Utils;

use crate::{
    NotificationInfo, SignTxSvg, ACTIVE_CONNECTION, CLUSTER_STORAGE, GLOBAL_MESSAGE, WALLET_ADAPTER,
};

#[component]
pub fn SignTx() -> Element {
    let lamports = 500_000_000u64;

    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        public_key = wallet_account.public_key();
    }

    rsx! {
        div { class:"flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none",
            div {class:"w-full flex flex-col items-center text-center text-true-blue justify-center mb-10",
                div{class:"w-[80px] flex flex-col", {SignTxSvg()}}
                div{class:"w-full text-sm", "Sign Transaction"}
            }
            div { class:"text-lg text-center",
            "Sign transfer of " {lamports.to_string()} " lamports!"
            }

        div { class:"flex items-center justify-center",
                button{
                    class: "bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                    onclick: move |_| {
                        spawn(async move {
                            let pubkey = Pubkey::new_from_array(public_key);
                            let recipient_pubkey = Pubkey::new_from_array(Utils::public_key_rand());

                            let instr = system_instruction::transfer(&pubkey, &recipient_pubkey, lamports);
                            let tx = Transaction::new_with_payer(&[instr], Some(&pubkey));
                            let tx_bytes = bincode::serialize(&tx).unwrap();
                            let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();

                            match WALLET_ADAPTER.read().sign_transaction(&tx_bytes, Some(cluster)).await{
                                Err(error) => GLOBAL_MESSAGE.write().push_back(
                                        NotificationInfo::error(
                                            format!("SIGN MESSAGE ERROR: {error:?}")
                                        )
                                    ),
                                Ok(output) => {
                                    if let Err(error) = bincode::deserialize::<Transaction>(&output[0]){
                                        GLOBAL_MESSAGE.write().push_back(
                                            NotificationInfo::error(
                                                format!("SIGN TX ERROR: {error:?}")
                                            )
                                        );
                                    }else {
                                        GLOBAL_MESSAGE.write().push_back(
                                            NotificationInfo::new("Sign Transaction Successful")
                                        );
                                    }
                                }
                            }
                        });
                    },
                    "SIGN TRANSACTION"
                }
            }
        }
    }
}
