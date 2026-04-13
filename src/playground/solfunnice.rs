use dioxus::prelude::*;

/// Stubbed out for Android/mobile build — full animation version requires dioxus-motion
#[component]
pub fn SolFunNiceApp() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center w-full h-full bg-rich-black text-white p-8",
            h1 { class: "text-true-blue text-4xl font-bold mb-4", "SOLFUNMEME" }
            p { class: "text-gray-400 text-lg", "Zero Ontology System" }
        }
    }
}
