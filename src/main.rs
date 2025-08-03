use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub href: Url,
    pub add_date: u64,
    pub icon: Option<Vec<u8>>, // dekodiertes Icon
}

fn main() -> std::io::Result<()> {
    let bookmarks = load_bookmarks_from_file("assets/bookmarks.html")?;
    save_to_json(&bookmarks, "assets/bookmarks.json")?;
    Ok(())
}


fn load_bookmarks_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Bookmark>> {
    let html = fs::read_to_string(path)?;
    Ok(parse_bookmarks(&html))
}

fn parse_bookmarks(html: &str) -> Vec<Bookmark> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|element| {
            let href = element.value().attr("href")?;
            let add_date = element.value().attr("add_date")?.parse().ok()?;
            let title = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            let icon_data = element.value().attr("icon");
            let icon = icon_data.and_then(|data_url| {
                data_url
                    .strip_prefix("data:image/png;base64,")
                    .and_then(|b64| STANDARD.decode(b64).ok())
            });

            Some(Bookmark {
                title,
                href: Url::parse(href).ok()?,
                add_date,
                icon,
            })
        })
        .collect()
}

fn save_to_json(bookmarks: &[Bookmark], path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(bookmarks)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
