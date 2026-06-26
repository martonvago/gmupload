#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Vers,
    Novella,
    Mese,
}

impl Category {
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Vers => "vers",
            Category::Novella => "novella",
            Category::Mese => "mese",
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