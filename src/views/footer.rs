use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "flex p-3 items-center justify-center w-full border-t border-[rgba(220,20,60,0.2)] bg-rich-black",
            div{p { class: "flex flex-wrap md:flex-row lg:flex-row items-center justify-center text-gray-600 text-xs",
                "for "
                a { href: "https://solfunmeme.com", class: "ml-2 mr-2 underline text-true-blue", " SOLFUNMEME" }
            }}
        }
    }
}
