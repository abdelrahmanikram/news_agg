use news_agg::models::{ApiResponse, Article};
use serde_json::json;

#[test]
fn test_parse_api_response() {
    let data = json!({
        "status": "ok",
        "totalResults": 1,
        "articles": [{
            "title": "Test Title",
            "description": "Test Description",
            "url": "https://test.com",
            "publishedAt": "2020-01-01T12:34:56Z"
        }]
    });

    let raw = data.to_string();
    let resp: ApiResponse = serde_json::from_str(&raw).unwrap();

    assert_eq!(resp.status, "ok");
    assert_eq!(resp.totalResults, 1);
    assert_eq!(resp.articles.len(), 1);
    assert_eq!(resp.articles[0].title, "Test Title");
}