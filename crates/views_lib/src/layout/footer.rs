use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "flex p-3 items-center justify-center w-full ",
            div{p { class: "flex flex-wrap md:flex-row lg:flex-row items-center justify-center",
                "for "
                a { href: "https://solfunmeme.com", class: "ml-2 mr-2 underline", " SOLFUNMEME" }
            }}
        }
    }
} 