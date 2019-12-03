extern crate reqwest;

use reqwest::Client;

pub fn get(url: &str) -> Result<String, reqwest::Error> {
  let session = format!("session={}", dotenv!("SESSION"));
  let mut res = Client::new()
    .get(url)
    .header("Cookie", session)
    .send()?;

  let text = res.text()?;

  Ok(text.trim().to_string())
}
