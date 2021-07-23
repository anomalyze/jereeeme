#![no_main]
use libfuzzer_sys::fuzz_target;
use std::fs::File;
use std::io::BufReader;
use std::fs::read_dir;
use std::io::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub content: String,
    pub uri: String,
}

pub fn summarize(path: &str) -> Result<Article, std::io::Error> {
    let f = File::open(path)?;
    let mut buf = BufReader::new(f);
    // create a temporary file to store unwanted lines.
    // haven't really found another way to do this.
    let mut _tmp = String::new();
    let mut title = String::new();
    let mut date = String::new();
    let mut raw_content = Vec::new();
    buf.read_line(&mut _tmp)?;
    buf.read_line(&mut title)?;
    buf.read_line(&mut date)?;
    buf.read_line(&mut _tmp)?;
    buf.read_line(&mut _tmp)?;
    title = title.split("title: ").collect::<String>();
    date = date.split("date: ").collect::<String>();
    buf.read_to_end(&mut raw_content)?;
    // pop the newline character off, so we can parse date correctly
    date.pop();
    date = NaiveDate::parse_from_str(&date, "%Y%m%d")
        .expect("Unable to parse date")
        .format("%d %B %Y")
        .to_string();
    let content = markdown_to_html(
        &String::from_utf8(raw_content).unwrap().to_owned(),
        &ComrakOptions::default(),
    );

    Ok(Article {
       title,
       date,
       content,
       uri: path.to_string(),
    })
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = summarize(s);
    }
});
