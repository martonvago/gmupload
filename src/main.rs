use dioxus::prelude::*;

use crate::components::{CategoryColumn, PostButton};
use crate::models::{Category, Piece};
use crate::post_template::build_post;

mod components;
mod models;
mod post_template;
mod wordpress;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        main { id: "app", AppContent {} }
    }
}

#[component]
fn AppContent() -> Element {
    let pieces_resource = use_resource(|| async {
        wordpress::fetch_pieces().await
    });
    let pieces = pieces_resource.read();

    let Some(pieces) = pieces.as_ref() else {
        return rsx! {
            p { "Várj egy picit..." }
        };
    };

    let poems = pieces_for_category(pieces, Category::Poem);
    let novellas = pieces_for_category(pieces, Category::Novella);
    let fairy_tales = pieces_for_category(pieces, Category::FairyTale);

    let selected_piece = use_signal(|| None::<usize>);
    let selected = selected_piece();
    let post_text = if let Some(index) = selected {
        build_post(&pieces[index])
    } else {
        String::new()
    };

    rsx! {
        h1 { "Írás megosztása" }
        p { "Kattints a címre, amit meg szeretnél osztani." }
        div { id: "columns",
            CategoryColumn { title: "Versek", pieces: poems, selected_piece }

            CategoryColumn { title: "Novellák", pieces: novellas, selected_piece }

            CategoryColumn { title: "Mesék", pieces: fairy_tales, selected_piece }
        }

        if let Some(index) = selected {
            p { "Kiválasztva: {pieces[index].title}" }
            pre { "{post_text}" }
        }

        PostButton { disabled: selected.is_none(), text: post_text }
    }
    }


fn pieces_for_category(
    pieces: &[Piece],
    category: Category,
) -> Vec<(usize, Piece)> {
    pieces
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, piece)| piece.category == category)
        .collect()
}