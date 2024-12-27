use core::cell::RefCell;

use alloc::{format, rc::Rc, string::{String, ToString}};

use crate::{error::Error, renderer::dom::node::{ElementKind, Node, NodeKind}};

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
  background_color: Option<Color>,
  color: Option<Color>,
  display: Option<DisplayType>,
  font_size: Option<FontSize>,
  text_decoration: Option<TextDecoration>,
  height: Option<f64>,
  width: Option<f64>,
}

impl ComputedStyle {
  pub fn new() -> Self {
    Self {
      background_color: None,
      color: None,
      display: None,
      font_size: None,
      text_decoration: None,
      height: None,
      width: None,
    }
  }

  pub fn set_background_color(&mut self, background_color: Color) {
    self.background_color = Some(background_color);
  }

  pub fn background_color(&self) -> Color {
    self.background_color.clone().expect("background_color is not set")
  }

  pub fn set_color(&mut self, color: Color) {
    self.color = Some(color);
  }

  pub fn color(&self) -> Color {
    self.color.clone().expect("color is not set")
  }

  pub fn set_display(&mut self, display: DisplayType) {
    self.display = Some(display);
  }

  pub fn display(&self) -> DisplayType {
    self.display.clone().expect("display is not set")
  }

  pub fn set_font_size(&mut self, font_size: FontSize) {
    self.font_size = Some(font_size);
  }

  pub fn font_size(&self) -> FontSize {
    self.font_size.clone().expect("font_size is not set")
  }

  pub fn set_text_decoration(&mut self, text_decoration: TextDecoration) {
    self.text_decoration = Some(text_decoration);
  }

  pub fn text_decoration(&self) -> TextDecoration {
    self.text_decoration.clone().expect("text_decoration is not set")
  }

  pub fn set_height(&mut self, height: f64) {
    self.height = Some(height);
  }

  pub fn height(&self) -> f64 {
    self.height.expect("height is not set")
  }

  pub fn set_width(&mut self, width: f64) {
    self.width = Some(width);
  }

  pub fn width(&self) -> f64 {
    self.width.expect("width is not set")
  }

  pub fn defaulting(&mut self, node: &Rc<RefCell<Node>>, parent_style: Option<ComputedStyle>) {
    if let Some(parent_style) = parent_style {
      if self.background_color.is_none() && parent_style.background_color() != Color::white() {
        self.background_color = Some(parent_style.background_color());
      }
      if self.color.is_none() && parent_style.color() != Color::black() {
        self.color = Some(parent_style.color());
      }
      if self.font_size.is_none() && parent_style.font_size() != FontSize::Medium {
        self.font_size = Some(parent_style.font_size());
      }
      if self.text_decoration.is_none() && parent_style.text_decoration() != TextDecoration::None {
        self.text_decoration = Some(parent_style.text_decoration());
      }
    }

    if self.background_color.is_none() {
      self.background_color = Some(Color::white());
    }
    if self.color.is_none() {
      self.color = Some(Color::black());
    }
    if self.display.is_none() {
      self.display = Some(DisplayType::default(node));
    }
    if self.font_size.is_none() {
      self.font_size = Some(FontSize::default(node));
    }
    if self.text_decoration.is_none() {
      self.text_decoration = Some(TextDecoration::default(node));
    }
    if self.height.is_none() {
      self.height = Some(0.0);
    }
    if self.width.is_none() {
      self.width = Some(0.0);
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
  name: Option<String>,
  code: String,
}

impl Color {
  pub fn from_name(name: &str) -> Result<Self, Error> {
    let code = match name {
      "black" => "#000000",
      "white" => "#FFFFFF",
      _ => return Err(Error::UnexpectedInput(format!("unexpected color name: {}", name))),
    };

    Ok(Self {
      name: Some(name.to_string()),
      code: code.to_string(),
    })
  }

  pub fn from_code(code: &str) -> Result<Self, Error> {
    if code.chars().nth(0) != Some('#') || code.len() != 7 {
      return Err(Error::UnexpectedInput(format!("unexpected color code: {}", code)));
    }

    let name = match code {
      "#000000" => "black",
      "#FFFFFF" => "white",
      _ => return Err(Error::UnexpectedInput(format!("unexpected color code: {}", code))),
    };
    Ok(Self {
      name: Some(name.to_string()),
      code: code.to_string(),
    })
  }

  pub fn white() -> Self {
    Self {
      name: Some("white".to_string()),
      code: "#FFFFFF".to_string(),
    }
  }

  pub fn black() -> Self {
    Self {
      name: Some("black".to_string()),
      code: "#000000".to_string(),
    }
  }

  pub fn code_u32(&self) -> u32 {
    u32::from_str_radix(&self.code[1..], 16).expect("failed to parse color code")
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontSize {
  Medium,
  XLarge,
  XXLarge,
}

impl FontSize {
  fn default(node: &Rc<RefCell<Node>>) -> Self {
    match node.borrow().kind() {
      NodeKind::Element(element) => match element.kind() {
        ElementKind::H1 => Self::XXLarge,
        ElementKind::H2 => Self::XLarge,
        ElementKind::P => Self::Medium,
        _ => Self::Medium,
      },
      _ => Self::Medium,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayType {
  Block,
  Inline,
  DisplayNone,
}

impl DisplayType {
  fn default(node: &Rc<RefCell<Node>>) -> Self {
    match node.borrow().kind() {
      NodeKind::Document => DisplayType::Block,
      NodeKind::Element(e) => {
        if e.is_block_element() {
          DisplayType::Block
        } else {
          DisplayType::Inline
        }
      }
      NodeKind::Text(_) => DisplayType::Inline,
    }
  }

  pub fn from_str(s: &str) -> Result<Self, Error> {
    match s {
      "block" => Ok(Self::Block),
      "inline" => Ok(Self::Inline),
      "none" => Ok(Self::DisplayNone),
      _ => Err(Error::UnexpectedInput(format!("unexpected display type: {}", s))),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextDecoration {
  None,
  Underline,
}

impl TextDecoration {
  fn default(node: &Rc<RefCell<Node>>) -> Self {
    match node.borrow().kind() {
      NodeKind::Element(e) => match e.kind() {
        ElementKind::A => TextDecoration::Underline,
        _ => TextDecoration::None,
      }
      _ => TextDecoration::None,
    }
  }
}