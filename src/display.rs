use crate::models::Article;

pub fn display_articles(articles: &[Article]) {
    for article in articles {
        println!("\nTitle: {}", article.title);
        if let Some(desc) = &article.description {
            println!("Description: {}", desc);
        }
        println!("URL: {}", article.url);
        println!("Published at: {}", article.publishedAt);
        println!("--------------------------------------");
    }
}
