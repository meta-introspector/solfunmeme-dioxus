use dioxus::prelude::*;
use crate::AccountState;

// Dialog state: open/closed, input address, loading, and fetched result
#[component]
pub fn QueryAccountDialog(show_query_dialog: Signal<bool>) -> Element {
    let mut input_address = use_signal(|| String::new());
    let mut error_message = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    // Optional: Store the fetched account info
    let mut account_info = use_signal(|| None::<AccountState>);

    let on_query = move |_| {
        let address = input_address.read().trim().to_string();
        if address.is_empty() {
            error_message.set(Some("Address cannot be empty".to_string()));
            return;
        }
        error_message.set(None);
        loading.set(true);

        // Spawn async fetching
        let address_clone = address.clone();
        spawn(async move {
            match crate::accounts_runner(&address_clone).await {
                Ok(account_state) => {
                    account_info.set(Some(account_state));
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to fetch: {e:?}")));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        if *show_query_dialog.read() {
            div { 
                div {
		    class: "shrink-0 select-none text-base text-gray-500 sm:text-sm/6",
                    h2 {  "Query Solana Account" }
                    input {
                        r#type: "text",
			class: "focus:outline-none bg-transparent border-b-2 border-white block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                        value: "{input_address}",
                        oninput: move |evt| input_address.set(evt.value().clone()),
                        placeholder: "Enter account address",
                    }
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
                        onclick: on_query,
                        disabled: *loading.read(),
                        "Query"
                    }
                    button {
                        class: "ml-2 text-gray-500 px-4 py-2",
                        onclick: move |_| show_query_dialog.set(false),
                        "Close"
                    }
                    if let Some(err) = error_message.read().as_ref() {
                        p { class: "text-red-500 mt-2", "{err}" }
                    }
                    if *loading.read() {
                        p { class: "mt-2", "Loading..." }
                    }
                    if let Some(account) = account_info.read().as_ref() {
                        // Render your account info here, or reuse your existing account view components
                        div { class: "mt-4",
                            p { "Balance: {account.balance}" }
                            // List tokens and transactions, etc.
                        }
                    }
                }
            }
        }
    }
}
