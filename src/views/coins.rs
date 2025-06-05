use dioxus::prelude::*;
#[component]
//show_query_dialog: Signal<bool>
pub fn QueryCoinDialog() -> Element {
    rsx! {
       // if *show_query_dialog.read() {
            div {
                class: "fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50",
                div {
                    class: "bg-white dark:bg-gray-800 p-6 rounded shadow-lg",
                    h2 { "Query Coin" }
                    // Add your coin query form here
                    button {
                        class: "mt-4 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
          //              onclick: move |_| show_query_dialog.set(false),
          //              "Close"
                    }
                }
            }
        //}
    }
}
