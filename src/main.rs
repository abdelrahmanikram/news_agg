mod api;
mod models;
mod display;

use anyhow::Result;
use std::env;
use api::fetch_news;
use display::display_articles;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "News Aggregator")]
#[command(about = "Fetches latest headlines from NewsAPI", long_about = None)]
struct Args {
    #[arg(short = 'c', long, default_value = "us")]
    country: String,

    #[arg(short = 'g', long)]
    category: Option<String>,

    #[arg(short, long)]
    query: Option<String>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let rt = tokio::runtime::Runtime::new()?;
    let api_key = env::var("NEWS_API_KEY")
        .expect("Please set the NEWS_API_KEY environment variable");

    info!("Fetching news for country: {}", args.country);
    let articles = rt.block_on(fetch_news(&api_key, &args.country, args.category.as_deref(), args.query.as_deref()))?;
    display_articles(&articles);

    Ok(())
}
