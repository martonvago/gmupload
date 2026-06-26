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

    let poems = pieces_for_category(&pieces, Category::Poem);
    let novellas = pieces_for_category(&pieces, Category::Novella);
    let fairy_tales = pieces_for_category(&pieces, Category::FairyTale);

    let selected = selected_piece();
    let text = if let Some(index) = selected {
        build_post(&pieces[index])
    } else {
        String::new()
    };

    rsx! {
        document::Stylesheet { href: MAIN_CSS }

        main { id: "app",
            h1 { "Írás megosztása" }
            p { "Kattints a címre, amit meg szeretnél osztani." }

            div { id: "columns",
                CategoryColumn { title: "Versek", pieces: poems, selected_piece }

                CategoryColumn {
                    title: "Novellák",
                    pieces: novellas,
                    selected_piece,
                }

                CategoryColumn {
                    title: "Mesék",
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

fn pieces_for_category(
    pieces: &[models::Piece],
    category: Category,
) -> Vec<(usize, models::Piece)> {
    pieces
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, piece)| piece.category == category)
        .collect()
}