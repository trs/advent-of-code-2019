extern crate reqwest;

use reqwest::Client;

pub fn get(url: &str) -> Result<Vec<String>, reqwest::Error> {
  let session = format!("session={}", dotenv!("SESSION"));
  let mut res = Client::new()
    .get(url)
    .header("Cookie", session)
    .send()?;

  let text = res.text()?;
  let lines = text.split("\n")
    .map(|s| s.to_string())
    .filter(|s| s.len() > 0)
    .collect();

  Ok(lines)
}
