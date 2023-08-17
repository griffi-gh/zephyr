use rustc_hash::{FxHashMap, FxHashSet};
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
pub struct ElementNodeQueryCache {
  pub id: Option<String>,
  pub classes: FxHashSet<String>
}

#[derive(Debug, Default)]
pub struct ElementNode {
  pub tag_name: String,
  pub attributes: FxHashMap<String, String>,
  pub query_cache: ElementNodeQueryCache,
  pub children: Vec<SharedNode>,
  pub parent: Option<WeakNode>,
}

impl ElementNode {
  pub fn attribute(&self, name: &str) -> Option<&str> {
    self.attributes.get(name.to_lowercase().as_str()).map(|x| x.as_str())
  }

  pub fn id(&self) -> Option<&str> {
    self.query_cache.id.as_deref()
  }

  pub fn classes(&self) -> &FxHashSet<String> {
    &self.query_cache.classes
  }

  //TODO: remove useless clones in set_attribute/process_attribute_change

  pub fn set_attribute(&mut self, key: String, value: String) {
    let prev = self.attributes.insert(key.clone(), value.clone());
    self.process_attribute_change(&key, &value, prev.as_deref());
  }

  fn process_attribute_change(&mut self, key: &str, to: &str, from: Option<&str>) {
    if let Some(prev) = from {
      if prev == to { return }
    }
    match key.to_ascii_lowercase().as_str() {
      "class" => {
        self.query_cache.classes.clear();
        self.query_cache.classes.extend(to.trim().split(' ').map(|x| x.to_ascii_lowercase()));
      },
      "id" => {
        self.query_cache.id = to.is_empty().then(|| to.trim().to_string());
      },
      _ => ()
    }
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
  pub fn set_parent(&mut self, parent: Option<WeakNode>) {
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
