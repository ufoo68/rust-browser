use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
  HashToken(String),
  Delim(char),
  Number(f64),
  Colon,
  SemiColon,
  OpenParenthesis,
  CloseParenthesis,
  OpenCurly,
  CloseCurly,
  Ident(String),
  StringToken(String),
  AtKeyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssTokenizer {
  pos: usize,
  input: Vec<char>
}

impl CssTokenizer {
  pub fn new(css: String) -> Self {
    Self {
      pos: 0,
      input: css.chars().collect()
    }
  }

  fn consume_string_token(&mut self) -> String {
    let mut s = String::new();

    loop {
      if self.pos >= self.input.len() {
        return s;
      }

      self.pos += 1;
      let c = self.input[self.pos];
      match c {
        '"' | '\'' => break,
        _ => s.push(c),
      }
    }

    s
  }

  fn consume_numeric_token(&mut self) -> f64 {
    let mut num = 0f64;
    let mut floating = false;
    let mut floating_digit = 1f64;

    loop {
      if self.pos >= self.input.len() {
        return num;
      }

      let c = self.input[self.pos];

      match c {
        '0'..='9' => {
          if floating {
            floating_digit *= 1f64 / 10f64;
            num += (c.to_digit(10).unwrap() as f64) * floating_digit;
          } else {
            num = num * 10.0 + (c.to_digit(10).unwrap() as f64);
          }
          self.pos += 1;
        }
        '.' => {
          floating = true;
          self.pos += 1;
        }
        _ => break,
      }
    }

    num
  }

  fn consume_ident_token(&mut self) -> String {
    let mut s = String::new();
    s.push(self.input[self.pos]);

    loop {
      self.pos += 1;
      let c = self.input[self.pos];
      match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
          s.push(c);
        }
        _ => {
          break;
        }
      }
    }
    
    s
  }
}

impl Iterator for CssTokenizer {
  type Item = CssToken;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if self.pos >= self.input.len() {
        return None;
      }

      let c = self.input[self.pos];

      let token = match c {
        '(' => CssToken::OpenParenthesis,
        ')' => CssToken::CloseParenthesis,
        '{' => CssToken::OpenCurly,
        '}' => CssToken::CloseCurly,
        ':' => CssToken::Colon,
        ';' => CssToken::SemiColon,
        '.' => CssToken::Delim('.'),
        ',' => CssToken::Delim(','),
        ' ' | '\n' => {
          self.pos += 1;
          continue;
        }
        '"' | '\'' => {
          let s = self.consume_string_token();
          CssToken::StringToken(s)
        }
        '0'..='9' => {
          let t = CssToken::Number(self.consume_numeric_token());
          self.pos -= 1;
          t
        }
        '#' => {
          let value = self.consume_ident_token();
          self.pos -= 1;
          CssToken::HashToken(value)
        }
        '-' => {
          let t = CssToken::Ident(self.consume_ident_token());
          self.pos -= 1;
          t
        }
        '@' => {
          if self.input[self.pos + 1].is_ascii_alphabetic()
            && self.input[self.pos + 2].is_alphabetic()
            && self.input[self.pos + 3].is_alphabetic()
          {
            self.pos += 1;
            let t = CssToken::AtKeyword(self.consume_ident_token());
            self.pos -= 1;
            t
          } else {
            CssToken::Delim('@')
          }
        }
        'a'..='z' | 'A'..='Z' | '_' => {
          let t = CssToken::Ident(self.consume_ident_token());
          self.pos -= 1;
          t
        }
        _ => {
          unimplemented!("CssTokenizer::next() not implemented for character: {}", c);
        }
      };

      self.pos += 1;
      return Some(token);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use alloc::string::ToString;

  #[test]
  fn test_empty() {
    let style = "".to_string();
    let mut t = CssTokenizer::new(style);
    assert_eq!(t.next(), None);
  }

  #[test]
  fn test_one_rule() {
    let style = "p { color: red; }".to_string();
    let mut t = CssTokenizer::new(style);
    assert_eq!(t.next(), Some(CssToken::Ident("p".to_string())));
    assert_eq!(t.next(), Some(CssToken::OpenCurly));
    assert_eq!(t.next(), Some(CssToken::Ident("color".to_string())));
    assert_eq!(t.next(), Some(CssToken::Colon));
    assert_eq!(t.next(), Some(CssToken::Ident("red".to_string())));
    assert_eq!(t.next(), Some(CssToken::SemiColon));
    assert_eq!(t.next(), Some(CssToken::CloseCurly));
    assert_eq!(t.next(), None);
  }

  #[test]
  fn test_id_selector() {
    let style = "#id { color: red; }".to_string();
    let mut t = CssTokenizer::new(style);
    assert_eq!(t.next(), Some(CssToken::HashToken("#id".to_string())));
    assert_eq!(t.next(), Some(CssToken::OpenCurly));
    assert_eq!(t.next(), Some(CssToken::Ident("color".to_string())));
    assert_eq!(t.next(), Some(CssToken::Colon));
    assert_eq!(t.next(), Some(CssToken::Ident("red".to_string())));
    assert_eq!(t.next(), Some(CssToken::SemiColon));
    assert_eq!(t.next(), Some(CssToken::CloseCurly));
    assert_eq!(t.next(), None);
  }

  #[test]
  fn test_multiple_rules() {
    let style = "p { color: red; } div { color: blue; }".to_string();
    let mut t = CssTokenizer::new(style);
    assert_eq!(t.next(), Some(CssToken::Ident("p".to_string())));
    assert_eq!(t.next(), Some(CssToken::OpenCurly));
    assert_eq!(t.next(), Some(CssToken::Ident("color".to_string())));
    assert_eq!(t.next(), Some(CssToken::Colon));
    assert_eq!(t.next(), Some(CssToken::Ident("red".to_string())));
    assert_eq!(t.next(), Some(CssToken::SemiColon));
    assert_eq!(t.next(), Some(CssToken::CloseCurly));
    assert_eq!(t.next(), Some(CssToken::Ident("div".to_string())));
    assert_eq!(t.next(), Some(CssToken::OpenCurly));
    assert_eq!(t.next(), Some(CssToken::Ident("color".to_string())));
    assert_eq!(t.next(), Some(CssToken::Colon));
    assert_eq!(t.next(), Some(CssToken::Ident("blue".to_string())));
    assert_eq!(t.next(), Some(CssToken::SemiColon));
    assert_eq!(t.next(), Some(CssToken::CloseCurly));
    assert_eq!(t.next(), None);
  }
}