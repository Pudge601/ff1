use result_scraper::{scrape_race_results, ScrapeError};
use std::env;

fn main() {
    let uri = env::args()
        .nth(1)
        .expect("Usage: result-scraper <race_uri>");
    match scrape_race_results(&uri) {
        Ok(results) => {
            for result in results {
                println!("Found driver result: {result:?}");
            }
        }
        Err(ScrapeError::RequestFailure(err)) => {
            println!("Failed to request race results: {err:?}");
        }
        Err(ScrapeError::ParseError) => {
            println!("Failed to parse race results");
        }
    }
}
