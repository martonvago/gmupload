use dioxus::prelude::*;

mod models;
mod wordpress;
mod post_template;
mod components;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

use crate::components::{CategoryColumn, PostButton};
use crate::models::Category;
use crate::post_template::build_post;

#[component]
fn App() -> Element {
    let pieces = wordpress::demo_pieces();
    let selected_piece = use_signal(|| None::<usize>);

    let selected = selected_piece();

    let text = if let Some(index) = selected {
        build_post(&pieces[index])
    } else {
        String::new()
    };

    let poems = pieces
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, piece)| piece.category == Category::Vers)
        .collect::<Vec<_>>();

    let novellas = pieces
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, piece)| piece.category == Category::Novella)
        .collect::<Vec<_>>();

    let fairy_tales = pieces
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, piece)| piece.category == Category::Mese)
        .collect::<Vec<_>>();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }

        main { id: "app",
            h1 { "Facebook Post Helper" }
            p { "Choose a piece to prepare a Facebook post." }

            div { id: "columns",
                CategoryColumn { title: "Poems", pieces: poems, selected_piece }

                CategoryColumn { title: "Novellas", pieces: novellas, selected_piece }

                CategoryColumn {
                    title: "Fairy Tales",
                    pieces: fairy_tales,
                    selected_piece,
                }
            }
            if let Some(index) = selected_piece() {
                p { "Selected: {pieces[index].title}" }

                pre { "{build_post(&pieces[index])}" }
            }

            PostButton { disabled: selected.is_none(), text }
        }
    }
}