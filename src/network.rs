use anyhow::Result;
use reqwest::Url;

/// Temporary solution for network requests
pub fn request(url: String) -> Result<String> {
    let url = url.parse::<Url>()?;

    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(html)
}
