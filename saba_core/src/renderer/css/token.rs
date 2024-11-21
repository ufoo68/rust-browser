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
        _ => {
          unimplemented!("CssTokenizer::next() not implemented for character: {}", c);
        }
      };

      self.pos += 1;
      return Some(token);
    }
  }
}