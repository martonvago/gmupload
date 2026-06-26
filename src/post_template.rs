use crate::models::Piece;

pub fn build_post(piece: &Piece) -> String {
    format!(
        "{} /{}/\n\n{}\n\nA teljes írás a weboldalamon:\n{}",
        piece.title,
        piece.category.display_name(),
        piece.content,
        piece.link,
    )
}