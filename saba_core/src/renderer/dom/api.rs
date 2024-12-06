use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

use super::node::{Element, ElementKind, Node, NodeKind};
use alloc::string::ToString;

pub fn get_target_element_node(
  node: Option<Rc<RefCell<Node>>>,
  element_kind: ElementKind,
) -> Option<Rc<RefCell<Node>>> {
  match node {
    Some(n) => {
      if n.borrow().kind() == NodeKind::Element(Element::new(&element_kind.to_string(), Vec::new())) {
        return Some(n.clone());
      }
      let result1 = get_target_element_node(n.borrow().first_child(), element_kind);
      let result2 = get_target_element_node(n.borrow().next_sibling(), element_kind);
      if result1.is_none() && result2.is_none() {
        return None;
      }
      if result1.is_none() {
        return result2;
      }
      result1
    }
    None => None,
  }
}