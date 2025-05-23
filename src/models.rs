use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub totalResults: u32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub publishedAt: String,
}