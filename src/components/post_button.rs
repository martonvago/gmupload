use dioxus::prelude::*;
use dioxus_clipboard::prelude::*;

#[component]
pub fn PostButton(disabled: bool, text: String) -> Element {
    let mut status = use_signal(String::new);
    let mut clipboard = use_clipboard();

    rsx! {
        button {
            disabled,
            onclick: move |_| {
                match clipboard.set(text.clone()) {
                    Ok(_) => status.set("Vágólapra másolva.".to_string()),
                    Err(_) => status.set("Nem sikerült másolni.".to_string()),
                }
            },
            "Szöveg másolása"
        }

        if !status().is_empty() {
            p { "{status}" }
        }
    }
}