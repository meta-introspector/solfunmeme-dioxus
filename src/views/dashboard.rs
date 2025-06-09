use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "flex flex-col justify-around items-center w-full m-h-[100%] h-full p-5",
            h1 {class:"text-black dark:text-gray-300 text-6xl", "gm"}

            h2 {class:"dark:text-gray-300 text-2xl" , ""}

            div {class:"text-center flex flex-col justify-between items-center",
                h3 {class:"text-black dark:text-gray-300 text-md", 
                    "Welcome to Solfunmeme Dashboard"
                }
                p {class:"text-black dark:text-gray-300 text-md",
                    "This is a work in progress, please be patient as we build this out."
                     ul {class:"text-black dark:text-gray-300 text-md",
                        li {"We are currently working on the following features:"       }
                        li {"1. Create a new contract on devnet."}
                        li {"2. import contract from mainnet via helius api."}
                        li {"3. import holders from mainnet via helius api."}
                        li {"4. import transactions from mainnet via helius api."}
                        li {"5. import token metadata from mainnet via helius api."}
                        li {"6. create new contract on devnet."}
                        li {"7. provision access to ip address on devnet via aws security groups."}
                        li {"8. setup aws credentials."}
                        li {"9. allow deployment of aws resources via terrafom replacement in browser/wasm via rust replacement."}
                        li {"10. deploy contract on devnet via solana api via our frontend."}
                        li {"11. allow swapping of tokens on devnet via our frontend."}
                        li {"12. allow for zkp transactions on devnet via our frontend."}
                        li {"13. use zkml proofs to prove inference correctness and convergence."}
                        li {"14. creation of lean oracles using llm zkp and zkml."}
                        li {"15. allow for zkml inference on devnet via our frontend."}
                        li {"16. generate images."}
                        li {"17. allow for quality control of images via llm and user control."}
                        li {"17. importing of events from twitter, discord, telegram, and github. (universal inbox)"}
                        li {"18. create emoji grammar language for memes, metamemes, and memes of memes, and meta programs, proof engines, lean4 expressions"}
                        li {"19. allow for creation of memes, metamemes, and memes of memes, and meta programs, proof engines, lean4 expressions."}
                        li {"20. allow for execution of the emojiprograms, proof engines, lean4 expressions."}

                    }
                }                                
            }
        }
    }
}
