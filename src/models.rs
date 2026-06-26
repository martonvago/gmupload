#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    Poem,
    Novella,
    FairyTale,
}

impl Category {
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Poem => "vers",
            Category::Novella => "novella",
            Category::FairyTale => "mese",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub title: String,
    pub link: String,
    pub content: String,
    pub category: Category,
}