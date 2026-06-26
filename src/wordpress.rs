use crate::models::{Category, Piece};

pub fn demo_pieces() -> Vec<Piece> {
    vec![
        Piece {
            title: "Spring Morning".to_string(),
            link: "https://example.com/spring-morning".to_string(),
            content: "This is the poem text for Spring Morning.".to_string(),
            category: Category::Vers,
        },
        Piece {
            title: "The Little Fox".to_string(),
            link: "https://example.com/the-little-fox".to_string(),
            content: "Once upon a time, there was a little fox...".to_string(),
            category: Category::Mese,
        },
        Piece {
            title: "A Short Memory".to_string(),
            link: "https://example.com/a-short-memory".to_string(),
            content: "This is a short novella-style piece.".to_string(),
            category: Category::Novella,
        },
    ]
}