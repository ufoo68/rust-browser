use alloc::string::String;
use alloc::vec::Vec;
use crate::error::Error;
use alloc::format;
use alloc::string::ToString;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
  pub fn new(name: String, value: String) -> Self {
    Self {
      name,
      value,
    }
  }
}

impl HttpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    let preprocessed_response = raw_response.replace("\r\n", "\n");

    let (status_line, remaining) = match preprocessed_response.split_once("\n") {
      Some((s, r)) => (s, r),
      None => {
        return Err(Error::Network(format!("invalid http response: {}", preprocessed_response)));
      },
    };

    let (headers, body) = match remaining.split_once("\n\n") {
      Some((h, b)) => {
        let mut headers = Vec::new();
        for header in h.split('\n') {
          let splitted_header: Vec<&str> = header.splitn(2, ':').collect();
          headers.push(Header::new(
            String::from(splitted_header[0].trim()),
            String::from(splitted_header[1].trim()),
          ));
        }
        (headers, b)
      },
      None => (Vec::new(), remaining),
    };

    let statuses: Vec<&str> = status_line.split(' ').collect();

    Ok(Self {
      version: statuses[0].to_string(),
      status_code: statuses[1].parse().unwrap_or(404),
      reason: statuses[2].to_string(),
      headers,
      body: body.to_string(),
    })
  }

  pub fn version(&self) -> String {
    self.version.clone()
  }

  pub fn status_code(&self) -> u32 {
    self.status_code.clone()
  }

  pub fn reason(&self) -> String {
    self.reason.clone()
  }

  pub fn headers(&self) -> Vec<Header> {
    self.headers.clone()
  }

  pub fn body(&self) -> String {
    self.body.clone()
  } 

  pub fn header_value(&self, name: &str) -> Result<String, Error> {
    for header in &self.headers {
      if header.name == name {
        return Ok(header.value.clone());
      }
    }
    
    Err(Error::Network(format!("Header not found: {}", name)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_http_response_new() {
    let raw_response = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n<html></html>";
    let response = HttpResponse::new(raw_response.to_string()).unwrap();

    assert_eq!(response.version(), "HTTP/1.1");
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.reason(), "OK");
    assert_eq!(response.headers().len(), 1);
    assert_eq!(response.headers()[0].name, "Content-Type");
    assert_eq!(response.headers()[0].value, "text/html");
    assert_eq!(response.body(), "<html></html>");
  }

  #[test]
  fn test_http_response_header_value() {
    let raw_response = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n<html></html>";
    let response = HttpResponse::new(raw_response.to_string()).unwrap();

    assert_eq!(response.header_value("Content-Type").unwrap(), "text/html");
    assert!(response.header_value("Content-Length").is_err());
  }

  #[test]
  fn test_invalid() {
    let raw = "HTTP/1.1 200 OK";
    let response = HttpResponse::new(raw.to_string());
    assert!(response.is_err());
  }
}