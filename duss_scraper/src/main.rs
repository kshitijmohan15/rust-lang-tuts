use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;

const BASE_URL: &str = "https://www.studiomasterprofessional.com";
const HEADER_NAV_LINK_SELECTOR: &str = "a.header-nav-link";
const H4_SELECTOR: &str = ".desc.product-name h4";
const A_SELECTOR: &str = "a";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Make the initial request to the base URL and handle any errors
    let response = reqwest::get(BASE_URL).await?;
    let response_text = response.text().await?;
    let doc = Html::parse_document(&response_text);

    // Parse the header nav link selector and handle any errors
    let headers_selector = Selector::parse(HEADER_NAV_LINK_SELECTOR)?;
    let headers_elements = doc.select(&headers_selector);

    for header_element in headers_elements {
        let header_text = header_element.text().collect::<String>();
        let slug = header_text.to_lowercase().trim().replace(" ", "-");

        // Make the dynamic request to the product URL and handle any errors
        let dyn_response = reqwest::get(format!("{}/products/{}", BASE_URL, slug)).await?;
        let dyn_response_text = dyn_response.text().await?;
        let dyn_doc = Html::parse_document(&dyn_response_text);

        // Parse the h4 and a selectors and handle any errors
        let h4_selector = Selector::parse(H4_SELECTOR)?;
        let a_selector = Selector::parse(A_SELECTOR)?;

        let mut element_array = vec![];
        for h4_element in dyn_doc.select(&h4_selector) {
            if let Some(a_element) = h4_element.select(&a_selector).next() {
                element_array.push(a_element.text().collect::<String>());
            }
        }
        println!("{}: {:?}", header_text, element_array);
    }

    Ok(())
}
