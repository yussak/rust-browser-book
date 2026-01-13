use core::cell::RefCell;

use alloc::rc::{Rc, Weak};

use crate::renderer::dom::node::{Node, NodeKind};

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
    pub fn new(node: Rc<RefCell<Node>>, parent_obj: &Option<Rc<RefCell<LayoutObject>>>) -> Self {
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
            point: LayoutPoint::new(0, 0),
            size: LayoutSize::new(0, 0),
        }
    }

    pub fn kind(&self) -> LayoutObjectKind {
        self.kind
    }

    pub fn node_kind(&self) -> NodeKind {
        self.node.borrow().kind().clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<LayoutObject>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.first_child.as_ref().cloned()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<LayoutObject>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.next_sibling.as_ref().cloned()
    }

    pub fn parent(&self) -> Weak<RefCell<Self>> {
        self.parent.clone()
    }

    pub fn style(&self) -> ComputedStyle {
        self.style.clone()
    }

    pub fn point(&self) -> LayoutPoint {
        self.point
    }

    pub fn size(&self) -> LayoutSize {
        self.size
    }
}
