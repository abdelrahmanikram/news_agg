# News Aggregator

A real-time CLI news aggregator built in Rust using the [NewsAPI](https://newsapi.org/) to fetch latest headlines by country, category, or search query.

---

## Features

- Fetch top headlines from NewsAPI
- Filter by country, category, and query keywords
- Output news articles with title, description, URL, and publish date

---
## Usage

Get a free API key from [https://newsapi.org](https://newsapi.org) and set it as an environment variable:

Example usage:
```bash
export NEWS_API_KEY=your_api_key_here
cargo run -- -c us -g technology
```
---

## Installation

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/yourusername/news_aggregator.git
cd news_aggregator
cargo build --release
```