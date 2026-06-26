use dioxus::prelude::*;
use dioxus_clipboard::prelude::*;

#[component]
pub fn PostButton(disabled: bool, text: String) -> Element {
    let mut clipboard = use_clipboard();

    rsx! {
        button {
            disabled,
            onclick: move |_| {
                clipboard.set(text.clone());
            },
            "Copy Facebook Post"
        }
    }
}