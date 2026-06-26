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
                if let Err(_) = clipboard.set(text.clone()) {
                    status.set("Nem sikerült a szöveget a vágólapra másolni.".to_string());
                    return;
                }

                if let Err(_) = open::that("https://www.facebook.com") {
                    status
                        .set(
                            "Nem sikerült megnyitni a Facebookot.".to_string(),
                        );
                    return;
                }
                status
                    .set(
                        "A Facebook megnyílt. Nyomj Ctrl+V-t a szöveg beillesztéséhez."
                            .to_string(),
                    );
            },

            "Megosztás Facebookon"
        }

        if !status().is_empty() {
            p { "{status}" }
        }
    }
}