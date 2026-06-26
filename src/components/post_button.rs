use dioxus::prelude::*;

#[component]
pub fn PostButton(disabled: bool, text: String) -> Element {
    let mut status = use_signal(String::new);

    rsx! {
        button {
            disabled,

            onclick: move |_| {
                let text = text.clone();
                spawn(async move {
                    if let Err(_) = crate::facebook::compose_post(&text).await {
                        status.set("Nem sikerült megnyitni a Facebookot.".to_string());
                    }
                });

                status.set("Facebook megnyitása...".to_string());
            },

            "Megosztás Facebookon"
        }

        if !status().is_empty() {
            p { "{status}" }
        }
    }
}