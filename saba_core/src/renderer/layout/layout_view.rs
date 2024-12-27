use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

use crate::{constants::CONTENT_AREA_WIDTH, display_items::DisplayItem, renderer::{css::cssom::StyleSheet, dom::{api::get_target_element_node, node::{ElementKind, Node}}}};

use super::layout_object::{create_layout_object, LayoutObject, LayoutObjectKind, LayoutPoint, LayoutSize};


#[derive(Debug, Clone)]
pub struct LayoutView {
  root: Option<Rc<RefCell<LayoutObject>>>,
}

impl LayoutView {
  pub fn new(
    root: Rc<RefCell<Node>>,
    cssom: &StyleSheet,
  ) -> Self {
    let body_root = get_target_element_node(Some(root), ElementKind::Body);

    let mut tree = Self {
      root: build_layout_tree(&body_root, &None, cssom),
    };

    tree.update_layout();

    tree
  }

  pub fn root(&self) -> Option<Rc<RefCell<LayoutObject>>> {
    self.root.clone()
  }

  pub fn update_layout(&mut self) {
    Self::calculate_node_size(&self.root, LayoutSize::new(CONTENT_AREA_WIDTH, 0));

    Self::calculate_node_position(
      &self.root,
      LayoutPoint::new(0, 0),
      LayoutObjectKind::Block,
      None,
      None,
    );
  }

  fn calculate_node_size(node: &Option<Rc<RefCell<LayoutObject>>>, parent_size: LayoutSize) {
    if let Some(n) = node {
      if n.borrow().kind() == LayoutObjectKind::Block {
        n.borrow_mut().compute_size(parent_size);
      }

      let first_child = n.borrow().first_child();
      Self::calculate_node_size(&first_child, n.borrow().size());

      let next_sibling = n.borrow().next_sibling();
      Self::calculate_node_size(&next_sibling, parent_size);

      n.borrow_mut().compute_size(parent_size);
    }
  }

  fn calculate_node_position(
    node: &Option<Rc<RefCell<LayoutObject>>>,
    parent_point: LayoutPoint,
    previous_sibling_kind: LayoutObjectKind,
    previous_sibling_point: Option<LayoutPoint>,
    previous_sibling_size: Option<LayoutSize>,
  ) {
    if let Some(n) = node {
      n.borrow_mut().compute_position(
        parent_point,
        previous_sibling_kind,
        previous_sibling_point,
        previous_sibling_size,
      );

      let first_child = n.borrow().first_child();
      Self::calculate_node_position(
        &first_child,
        n.borrow().point(),
        LayoutObjectKind::Block,
        None,
        None,
      );

      let next_sibling = n.borrow().next_sibling();
      Self::calculate_node_position(
        &next_sibling,
        parent_point,
        n.borrow().kind(),
        Some(n.borrow().point()),
        Some(n.borrow().size()),
      );
    }
  }

  fn paint_node(node: &Option<Rc<RefCell<LayoutObject>>>, display_items: &mut Vec<DisplayItem>) {
    match node {
      Some(n) => {
        display_items.extend(n.borrow_mut().paint());
        let first_child = n.borrow().first_child();
        Self::paint_node(&first_child, display_items);

        let next_sibling = n.borrow().next_sibling();
        Self::paint_node(&next_sibling, display_items);
      }
      None => {}
    }
  }

  pub fn paint(&self) -> Vec<DisplayItem> {
    let mut display_items = Vec::new();
    Self::paint_node(&self.root, &mut display_items);
    display_items
  }
}

fn build_layout_tree(
  node: &Option<Rc<RefCell<Node>>>,
  parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
  cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>>
{
  let mut target_node = node.clone();
  let mut layout_object = create_layout_object(node, parent_obj, cssom);
  while layout_object.is_none() {
    if let Some(n) = target_node {
      target_node = n.borrow().next_sibling().clone();
      layout_object = create_layout_object(&target_node, parent_obj, cssom);
    } else {
      return layout_object;
    }
  }

  if let Some(n) = target_node {
    let original_first_child = n.borrow().first_child();
    let original_next_sibling = n.borrow().next_sibling();
    let mut first_child = build_layout_tree(&original_first_child, &layout_object, cssom);
    let mut next_sibling = build_layout_tree(&original_next_sibling, &None, cssom);

    if first_child.is_none() && original_first_child.is_some() {
      let mut original_dom_node = original_first_child.expect("original_first_child is not None").borrow().next_sibling();
      loop {
        first_child = build_layout_tree(&original_dom_node, &layout_object, cssom);
        if first_child.is_none() && original_dom_node.is_some() {
          original_dom_node = original_dom_node.expect("original_dom_node is not None").borrow().next_sibling();
          continue;
        }

        break;
      }
    }

    if next_sibling.is_none() && original_next_sibling.is_some() {
      let mut original_dom_node = original_next_sibling.expect("original_next_sibling is not None").borrow().next_sibling();
      loop {
        next_sibling = build_layout_tree(&original_dom_node, &None, cssom);
        if next_sibling.is_none() && original_dom_node.is_some() {
          original_dom_node = original_dom_node.expect("original_dom_node is not None").borrow().next_sibling();
          continue;
        }

        break;
      }
    }

    let obj = match layout_object {
      Some(ref obj) => obj,
      None => panic!("layout_object is None"),
    };
    obj.borrow_mut().set_first_child(first_child);
    obj.borrow_mut().set_next_sibling(next_sibling);
  }

  layout_object
}

#[cfg(test)]
mod texts {
  use alloc::string::{String, ToString};

  use super::*;

  use crate::renderer::css::cssom::CssParser;
  use crate::renderer::css::token::CssTokenizer;
  use crate::renderer::dom::api::get_style_content;
  use crate::renderer::html::parser::HtmlParser;
  use crate::renderer::html::token::HtmlTokenizer;

  fn create_layout_view(html: String) -> LayoutView {
    let t = HtmlTokenizer::new(html);
    let window = HtmlParser::new(t).construct_tree();
    let dom = window.borrow().document();
    let style = get_style_content(dom.clone());
    let css_tokenizer = CssTokenizer::new(style);
    let cssom = CssParser::new(css_tokenizer).parse_stylesheet();
    LayoutView::new(dom, &cssom)
  }

  #[test]
  fn test_empty() {
    let layout_view = create_layout_view("".to_string());
    assert_eq!(None, layout_view.root());
  }
}