
pub fn parse_lines(text: String) -> Vec<String> {
  let lines = text.split("\n")
    .map(|s| s.to_string())
    .filter(|s| s.len() > 0)
    .collect();

  lines
}
