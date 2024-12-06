use crate::renderer::css::token::CssTokenizer;
use core::iter::Peekable;
use alloc::vec::Vec;
use crate::alloc::string::ToString;
use crate::alloc::string::String;
use crate::renderer::css::token::CssToken;

#[derive(Debug, Clone)]
pub struct CssParser {
  t: Peekable<CssTokenizer>,
}

impl CssParser {
  pub fn new(t: CssTokenizer) -> Self {
    Self {
      t: t.peekable(),
    }
  }

  pub fn parse_stylesheet(&mut self) -> StyleSheet {
    let mut sheet = StyleSheet::new();
    sheet.set_rules(self.consume_list_of_rules());
    sheet
  }

  fn consume_list_of_rules(&mut self) -> Vec<QualifiedRule> {
    let mut rules = Vec::new();

    loop {
      let token = match self.t.peek() {
        Some(t) => t,
        None => return rules,
      };
      match token {
        CssToken::AtKeyword(_keyword) => {
          let _rule = self.consume_qualified_rule();
        }
        _ => {
          let rule = self.consume_qualified_rule();
          match rule {
            Some(r) => rules.push(r),
            None => return rules,
          }
        }
      }
    }
  }

  fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
    let mut rule = QualifiedRule::new();
    loop {
      let token = match self.t.peek() {
        Some(t) => t,
        None => return None,
      };

      match token {
        CssToken::OpenCurly => {
          assert_eq!(self.t.next(), Some(CssToken::OpenCurly));
          rule.set_declarations(self.consume_list_of_declarations());
          return Some(rule);
        }
        _ => {
          rule.set_selector(self.consume_selector());
        }
      }
    }
  }

  fn consume_selector(&mut self) -> Selector {
    let token = match self.t.next() {
      Some(t) => t,
      None => panic!("Unexpected end of input"),
    };

    match token {
      CssToken::HashToken(value) => Selector::IdSelector(value[1..].to_string()),
      CssToken::Delim(delim) => {
        if delim == '.' {
          return Selector::ClassSelector(self.consume_ident());

        }
        panic!("Unexpected token: {:?}", token);
      }
      CssToken::Ident(ident) => {
        if self.t.peek() == Some(&CssToken::Colon) {
          while self.t.peek() != Some(&CssToken::OpenCurly) {
            self.t.next();
          }
        }
        Selector::TypeSelector(ident.to_string())
      }
      CssToken::AtKeyword(_keyword) => {
        while self.t.peek() != Some(&CssToken::OpenCurly) {
          self.t.next();
        }
        Selector::UnknownSelector
      }
      _ => {
        self.t.next();
        Selector::UnknownSelector
      }
    }
  }

  fn consume_list_of_declarations(&mut self) -> Vec<Declaration> {
    let mut declarations = Vec::new();

    loop {
      let token = match self.t.peek() {
        Some(t) => t,
        None => return declarations,
      };

      match token {
        CssToken::CloseCurly => {
          assert_eq!(self.t.next(), Some(CssToken::CloseCurly));
          return declarations;
        }
        CssToken::SemiColon => {
          assert_eq!(self.t.next(), Some(CssToken::SemiColon));
        }
        CssToken::Ident(ref _ident) => {
          if let Some(declaration) = self.consume_declaration() {
            declarations.push(declaration);
          }
        }
        _ => {
          self.t.next();
        }
      }
    }
  }

  fn consume_declaration(&mut self) -> Option<Declaration> {
    if self.t.peek().is_none() {
      return None;
    }

    let mut declaration = Declaration::new();
    declaration.set_property(self.consume_ident());
    match self.t.next() {
      Some(token) => match token {
        CssToken::Colon => {}
        _ => return None,
      },
      None => return None,
    }

    declaration.set_value(self.consume_component_value());
    Some(declaration)
  }

  fn consume_ident(&mut self) -> String {
    let token = match self.t.next() {
      Some(t) => t,
      None => panic!("Unexpected end of input"),
    };

    match token {
      CssToken::Ident(ident) => ident.to_string(),
      _ => panic!("Unexpected token: {:?}", token),
    }
  }

  fn consume_component_value(&mut self) -> ComponentValue {
    self.t.next().expect("should have a token")
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
  pub rules: Vec<QualifiedRule>,
}

impl StyleSheet {
  pub fn new() -> Self {
    Self {
      rules: Vec::new(),
    }
  }

  pub fn set_rules(&mut self, rules: Vec<QualifiedRule>) {
    self.rules = rules;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedRule {
  pub selector: Selector,
  pub declarations: Vec<Declaration>,
}

impl QualifiedRule {
  pub fn new() -> Self {
    Self {
      selector: Selector::TypeSelector("".to_string()),
      declarations: Vec::new(),
    }
  }

  pub fn set_selector(&mut self, selector: Selector) {
    self.selector = selector;
  }

  pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
    self.declarations = declarations;
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
  TypeSelector(String),
  ClassSelector(String),
  IdSelector(String),
  UnknownSelector,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
  pub property: String,
  pub value: ComponentValue,
}

impl Declaration {
  pub fn new() -> Self {
    Self {
      property: String::new(),
      value: ComponentValue::Ident(String::new()),
    }
  }

  pub fn set_property(&mut self, property: String) {
    self.property = property;
  }

  pub fn set_value(&mut self, value: ComponentValue) {
    self.value = value;
  }
}

pub type ComponentValue = CssToken;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    let css = String::from("");
    let tokenizer = CssTokenizer::new(css);
    let mut parser = CssParser::new(tokenizer);
    let sheet = parser.parse_stylesheet();
    assert_eq!(sheet.rules.len(), 0);
  }

  #[test]
  fn test_one_rule() {
    let css = String::from("h1 { color: red; }");
    let tokenizer = CssTokenizer::new(css);
    let mut parser = CssParser::new(tokenizer);
    let sheet = parser.parse_stylesheet();
    assert_eq!(sheet.rules.len(), 1);
    let rule = &sheet.rules[0];
    assert_eq!(rule.selector, Selector::TypeSelector("h1".to_string()));
    assert_eq!(rule.declarations.len(), 1);
    let declaration = &rule.declarations[0];
    assert_eq!(declaration.property, "color");
    assert_eq!(declaration.value, CssToken::Ident("red".to_string()));
  }
}