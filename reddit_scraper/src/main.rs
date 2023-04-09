use scraper::{Html, Selector};
use std::error::Error;
use reqwest::header::{USER_AGENT, HeaderMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the user agent to avoid Reddit's bot detection
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".parse().unwrap());

    // Fetch the HTML content of the top posts of r/rust
    let response = reqwest::Client::builder()
        .default_headers(headers)
        .build()?
        .get("https://www.reddit.com/r/rust/top/?t=day")
        .send()
        .await?
        .text()
        .await?;

    // Parse the HTML using the scraper crate
    let document = Html::parse_document(&response);
    let title_selector = Selector::parse("h3._eYtD2XCVieq6emjKBH3m").unwrap();

    // Extract the titles of the posts into a vector
    let mut titles = Vec::new();
    for title_element in document.select(&title_selector) {
        let title = title_element.text().collect::<String>();
        titles.push(title);
    }

    // Print the vector of titles
    println!("{:?}", titles);

    Ok(())
}