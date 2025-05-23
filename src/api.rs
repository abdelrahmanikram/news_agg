use anyhow::{Result, Context};
use reqwest::header::{USER_AGENT, HeaderMap};
use crate::models::{ApiResponse, Article};

pub async fn fetch_news(api_key: &str, country: &str, category: Option<&str>, query: Option<&str>) -> Result<Vec<Article>> {
       let mut url = format!("https://newsapi.org/v2/top-headlines?country={}&apiKey={}", country, api_key);

    if let Some(cat) = category {
        url.push_str(&format!("&category={}", cat));
    }
    if let Some(q) = query {
        url.push_str(&format!("&q={}", q));
    }

    let client = reqwest::Client::new();

    let resp = client
        .get(&url)
        .header(USER_AGENT, "news_aggregator_app/0.1")
        .send()
        .await
        .context("Failed to make request to News API")?
        .text()
        .await
        .context("Failed to read response text")?;

    let parsed: ApiResponse = serde_json::from_str(&resp)
        .context("Failed to parse JSON response")?;
    Ok(parsed.articles)
}