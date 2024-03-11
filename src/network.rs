use anyhow::Result;
use reqwest::Url;

/// Temporary solution for network requests
pub fn request(url: &str) -> Result<String> {
    let url = match url.starts_with("http") {
        true => url.parse::<Url>()?,
        false => format!("https://{}", url).parse::<Url>()?,
    };

    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(html)
}
