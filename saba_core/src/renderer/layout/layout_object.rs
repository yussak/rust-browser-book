use core::cell::RefCell;

use alloc::rc::{Rc, Weak};

use crate::renderer::dom::node::Node;

#[derive(Debug, Clone)]
// 描画に必要な情報をすべて持った構造体
pub struct LayoutObject {
    kind: LayoutObjectKind,
    node: Rc<RefCell<Node>>,
    first_child: Option<Rc<RefCell<LayoutObject>>>,
    next_sibling: Option<Rc<RefCell<LayoutObject>>>,
    parent: Weak<RefCell<LayoutObject>>,
    style: ComputedStyle,
    point: LayoutPoint,
    size: LayoutSize,
}

impl LayoutObject {
  pub fn new(node: Rc<RefCell<Node>>>, parent_obj: &Option<Rc<RefCell<LayoutObject>>>) -> Self {
    let parent = match parent_obj {
      Some(p) => Rc::downgrade(p),
      None => Weak::new(),
    };

    Self {
      kind: LayoutObjectKind::Block,
      node: node.clone(),
      first_child: None,
      next_sibling: None,
      parent,
      style: ComputedStyle::new(),
      point: LayoutPoint::new(0,0),
      size:LayoutSize::new(0,0),
    }
  }
}