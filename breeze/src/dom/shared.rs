use super::Node;
use std::{rc::{Rc, Weak}, cell::RefCell};

pub trait SharedClone {
  fn shared_clone(&self) -> Self;
}

#[repr(transparent)]
#[derive(Debug)]
pub struct SharedNode(pub Rc<RefCell<Node>>);

impl From<Node> for SharedNode {
  fn from(value: Node) -> Self {
    Self(Rc::new(RefCell::new(value)))
  }
}

impl TryFrom<WeakNode> for SharedNode {
  type Error = ();
  fn try_from(value: WeakNode) -> Result<Self, Self::Error> {
    value.0.upgrade().ok_or(()).map(SharedNode)
  }
}

impl SharedClone for SharedNode {
  fn shared_clone(&self) -> Self {
    Self(Rc::clone(&self.0))
  }
}


#[repr(transparent)]
#[derive(Debug)]
pub struct WeakNode(pub Weak<RefCell<Node>>);

impl From<SharedNode> for WeakNode {
  fn from(node: SharedNode) -> Self {
    WeakNode(Rc::downgrade(&node.0))
  }
}

impl SharedClone for WeakNode {
  fn shared_clone(&self) -> Self {
    Self(Weak::clone(&self.0))
  }
}
