use scraper::{Html, Selector};

static HN_URL: &str = "https://news.ycombinator.com";

fn parse_points(text: &str) -> u32 {
    match text.split_whitespace().next() {
        Some(points_str) => points_str.parse::<u32>().unwrap(),
        None => 0
    }
}

fn main() {
    let content = reqwest::blocking::get(HN_URL)
        .expect("Failed to fetch HN website.")
        .text()
        .unwrap();
    let parsed = Html::parse_document(&content);
    let title_selector = Selector::parse("td.title a.storylink").unwrap();
    let score_selector = Selector::parse("span.score").unwrap();
    let scores = parsed.select(&score_selector).collect::<Vec<_>>();
    println!("Scores len: {}", scores.len());
    for (i, title) in parsed.select(&title_selector).enumerate() {        
        println!("{} [{}]", title.inner_html(), parse_points(&scores[i].inner_html()));
    }
}
