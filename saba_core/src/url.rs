use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
  pub fn new(url: String) -> Self {
    Self {
      url,
      host: "".to_string(),
      port: "".to_string(),
      path: "".to_string(),
      searchpart: "".to_string(),
    }
  }

  pub fn parse(&mut self) -> Result<Self, String> {
    if !self.is_http() {
      return Err("Only HTTP scheme is supported.".to_string());
    }

    self.host = self.extract_host();
    self.port = self.extract_port();
    self.path = self.extract_path();
    self.searchpart = self.extract_searchpart();

    Ok(self.clone())
  }

  fn is_http(&mut self) -> bool {
    if self.url.contains("http://") {
      return true;
    }
    false
  }

  fn extract_host(&self) -> String {
    let url_parts: Vec<&str> = self
      .url
      .trim_start_matches("http://")
      .splitn(2, "/")
      .collect();

    if let Some(index) = url_parts[0].find(':') {
      url_parts[0][..index].to_string()
    } else {
      url_parts[0].to_string()
    }
  }

  fn extract_port(&self) -> String {
    let url_parts: Vec<&str> = self
      .url
      .trim_start_matches("http://")
      .splitn(2, "/")
      .collect();

    if let Some(index) = url_parts[0].find(':') {
      url_parts[0][index + 1..].to_string()
    } else {
      "80".to_string()
    }
  }

  fn extract_path(&self) -> String {
    let url_parts: Vec<&str> = self
      .url
      .trim_start_matches("http://")
      .splitn(2, "/")
      .collect();

    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    path_and_searchpart[0].to_string()
  }

  fn extract_searchpart(&self) -> String {
    let url_parts: Vec<&str> = self
      .url
      .trim_start_matches("http://")
      .splitn(2, "/")
      .collect();

    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
    if path_and_searchpart.len() < 2 {
      return "".to_string();
    }

    path_and_searchpart[1].to_string()
  }

  pub fn host(&self) -> String {
    self.host.clone()
  }

  pub fn port(&self) -> String {
    self.port.clone()
  }

  pub fn path(&self) -> String {
    self.path.clone()
  }

  pub fn searchpart(&self) -> String {
    self.searchpart.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_url_parse() {
    let mut url = Url::new("http://example.com:8080/path/to/resource?searchpart".to_string());
    let parsed_url = url.parse().unwrap();

    assert_eq!(parsed_url.host(), "example.com");
    assert_eq!(parsed_url.port(), "8080");
    assert_eq!(parsed_url.path(), "path/to/resource");
    assert_eq!(parsed_url.searchpart(), "searchpart");
  }

  #[test]
  fn test_no_scheme() {
    let mut url = Url::new("example.com:8080/path/to/resource?searchpart".to_string());
    let parsed_url = url.parse();

    assert_eq!(parsed_url, Err("Only HTTP scheme is supported.".to_string()));
  }

  #[test]
  fn test_no_port() {
    let mut url = Url::new("http://example.com/path/to/resource?searchpart".to_string());
    let parsed_url = url.parse().unwrap();

    assert_eq!(parsed_url.host(), "example.com");
    assert_eq!(parsed_url.port(), "80");
    assert_eq!(parsed_url.path(), "path/to/resource");
    assert_eq!(parsed_url.searchpart(), "searchpart");
  }

  #[test]
  fn test_no_searchpart() {
    let mut url = Url::new("http://example.com:8080/path/to/resource".to_string());
    let parsed_url = url.parse().unwrap();

    assert_eq!(parsed_url.host(), "example.com");
    assert_eq!(parsed_url.port(), "8080");
    assert_eq!(parsed_url.path(), "path/to/resource");
    assert_eq!(parsed_url.searchpart(), "");
  }

  #[test]
  fn test_no_path_and_searchpart() {
    let mut url = Url::new("http://example.com:8080".to_string());
    let parsed_url = url.parse().unwrap();

    assert_eq!(parsed_url.host(), "example.com");
    assert_eq!(parsed_url.port(), "8080");
    assert_eq!(parsed_url.path(), "");
    assert_eq!(parsed_url.searchpart(), "");
  }

  #[test]
  fn test_no_path_and_searchpart_and_port() {
    let mut url = Url::new("http://example.com".to_string());
    let parsed_url = url.parse().unwrap();

    assert_eq!(parsed_url.host(), "example.com");
    assert_eq!(parsed_url.port(), "80");
    assert_eq!(parsed_url.path(), "");
    assert_eq!(parsed_url.searchpart(), "");
  }

  #[test]
  fn test_unsupported_scheme() {
    let mut url = Url::new("https://example.com".to_string());
    let parsed_url = url.parse();

    assert_eq!(parsed_url, Err("Only HTTP scheme is supported.".to_string()));
  }
}