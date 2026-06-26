use dioxus::prelude::*;

use crate::models::Piece;

#[component]
pub fn CategoryColumn(
    title: &'static str,
    pieces: Vec<(usize, Piece)>,
    selected_piece: Signal<Option<usize>>,
) -> Element {
    rsx! {
        div { class: "category-column",
            h2 { "{title}" }

            for (index , piece) in pieces {
                button {
                    key: "{piece.link}",
                    onclick: move |_| selected_piece.set(Some(index)),
                    "{piece.title}"
                }
            }
        }
    }
}