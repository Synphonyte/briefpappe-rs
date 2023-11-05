use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collection {
    pub title: String,
    pub tags: Vec<String>,
    pub timestamp: u64,
    pub slug: String,
    pub papers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum CollectionFilter {
    #[default]
    All,
    Tag(String),
}
