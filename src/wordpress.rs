use crate::models::{Category, Piece};

#[derive(Debug, serde::Deserialize)]
struct PieceResponse {
    title: String,
    link: String,
    content: String,
    category: CategoryResponse,
}

#[derive(Debug, serde::Deserialize)]
enum CategoryResponse {
    #[serde(rename = "versek")]
    Versek,

    #[serde(rename = "haikuk")]
    Haikuk,

    #[serde(rename = "novellak")]
    Novellak,

    #[serde(rename = "mesek")]
    Mesek,
}

impl From<CategoryResponse> for Category {
    fn from(category: CategoryResponse) -> Self {
        match category {
            CategoryResponse::Versek => Category::Poem,
            CategoryResponse::Haikuk => Category::Poem,
            CategoryResponse::Novellak => Category::Novella,
            CategoryResponse::Mesek => Category::FairyTale,
        }
    }
}

impl From<PieceResponse> for Piece {
    fn from(post: PieceResponse) -> Self {
        Self {
            title: post.title,
            link: post.link,
            content: post.content,
            category: post.category.into(),
        }
    }
}

pub async fn fetch_pieces() -> Result<Vec<Piece>, reqwest::Error> {
    let response = reqwest::get(
        "https://beszeloszavak.hu/wp-json/gmupload/v1/pieces",
    )
    .await?;

    response.error_for_status()?
        .json::<Vec<PieceResponse>>()
        .await
        .map(|responses| responses.into_iter().map(Piece::from).collect())
}