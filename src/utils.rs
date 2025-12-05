use std::env;

use reqwest::header::COOKIE;

pub fn get_input(day: u32) -> reqwest::Result<String> {
    let cookie = format!(
        "session={}",
        env::var("SESSION_COOKIE").expect("Missing SESSION_COOKIE environment variable!")
    );
    let client = reqwest::blocking::Client::new();
    client
        .get(format!("https://adventofcode.com/2025/day/{}/input", day))
        .header(COOKIE, cookie)
        .send()?
        .text()
}
