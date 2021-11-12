use scraper::{Html, Selector};

static HN_URL: &str = "https://news.ycombinator.com";

fn main() {
    better_panic::install();
    let content = reqwest::blocking::get(HN_URL)
        .expect("Failed to fetch HN website.")
        .text()
        .unwrap();
    let parsed = Html::parse_document(&content);
    let row_selector = Selector::parse("tr.athing").unwrap();
    let title_selector = Selector::parse("td.title a.titlelink").unwrap();
    for story in parsed.select(&row_selector) {
        let title = story.select(&title_selector).next().unwrap().inner_html();
        println!("{}", title);
    }
}
