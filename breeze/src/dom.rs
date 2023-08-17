use rustc_hash::FxHashMap;
use thiserror::Error;

mod parse;
mod shared;
pub use shared::{SharedNode, WeakNode, SharedClone};

#[derive(Error, Debug)]
pub enum DomPushError {
  #[error("text nodes can't have children")]
  NodeInfertile,

  #[error("node already has parent")]
  AlreadyHasParent,
}

#[derive(Debug, Default)]
pub struct ElementNode {
  pub tag_name: String,
  pub attributes: FxHashMap<String, String>,
  pub children: Vec<SharedNode>,
  pub parent: Option<WeakNode>,
}

impl ElementNode {
  pub fn attribute(&self, name: &str) -> Option<&str> {
    self.attributes.get(name.to_lowercase().as_str()).map(|x| x.as_str())
  }

  pub fn id(&self) -> Option<&str> {
    self.attribute("name")
  }

  pub fn classes(&self) -> Option<impl Iterator<Item = &str>> {
    Some(self.attribute("class")?.split(' '))
  }
}

#[derive(Debug, Default)]
pub struct TextNode {
  pub text: String,
  pub parent: Option<WeakNode>,
}

// NOTE: Implementing Clone for Node may cause issues with Deref?

#[derive(Debug)]
pub enum Node {
  Element(ElementNode),
  Text(TextNode),
}

impl Node {
  pub fn parent(&self) -> Option<&WeakNode> {
    match self {
      Node::Element(element) => element.parent.as_ref(),
      Node::Text(text) => text.parent.as_ref(),
    }
  }
  fn set_parent(&mut self, parent: Option<WeakNode>) {
    match self {
      Node::Element(element) => element.parent = parent,
      Node::Text(text) => text.parent = parent,
    }
  }
}


impl SharedNode {
  pub fn root() -> Self {
    Node::Element(ElementNode {
      tag_name: "root".into(),
      ..Default::default()
    }).into()
  }

  pub fn push(&self, node: SharedNode) -> Result<(), DomPushError> {
    if node.0.borrow().parent().is_some() {
      return Err(DomPushError::AlreadyHasParent);
    }
    if let Node::Element(self_node) = &mut *self.0.borrow_mut() {
      node.0.borrow_mut().set_parent(Some(self.shared_clone().into()));
      self_node.children.push(node.shared_clone());
    } else {
      return Err(DomPushError::NodeInfertile);
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct Dom {
  pub tree: SharedNode
}
