use std::{fmt::Debug, rc::Rc, cell::RefCell};
use partialdebug::placeholder::PartialDebug;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::elements::ElementInterface;

mod shared;
mod errors;
mod parse;

pub use shared::{SharedNode, WeakNode, SharedClone};
pub use errors::DomPushError;

pub trait InnerHtml {
  fn inner_html(&self) -> String;
}

/// Do not change values in this struct directly
#[derive(Debug, Default)]
pub struct ElementNodeCache {
  pub id: Option<String>,
  pub classes: FxHashSet<String>
}

#[derive(Default, PartialDebug)]
pub struct ElementNode {
  pub element: Option<Rc<RefCell<dyn ElementInterface>>>,
  pub tag_name: String,
  pub attributes: FxHashMap<String, String>,
  pub cache: ElementNodeCache,
  pub children: Vec<SharedNode>,
  pub parent: Option<WeakNode>,
}

impl ElementNode {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_with_tag(tag: String) -> Self {
    let mut this = Self::new();
    this.set_tag(tag);

    this
  }

  pub fn new_with_tag_and_attributes(tag: String, attributes: FxHashMap<String, String>) -> Self {
    let mut this = Self::new_with_tag(tag);
    //XXX: should process_attribute_change be called after setting this.attributes? (requires clone)
    for (k, v) in &attributes {
      this.process_attribute_change(k, Some(v), None);
    }
    this.attributes = attributes;
    this
  }

  pub fn set_tag(&mut self, tag: String) {
    //TODO add some stuff here?
    #[allow(clippy::match_single_binding)] {
    self.element = match tag.as_str() {
      _ => None
    };}
    self.tag_name = tag;
  }

  pub fn attribute(&self, name: &str) -> Option<&str> {
    self.attributes.get(name.to_lowercase().as_str()).map(|x| x.as_str())
  }

  pub fn id(&self) -> Option<&str> {
    self.cache.id.as_deref()
  }

  pub fn classes(&self) -> &FxHashSet<String> {
    &self.cache.classes
  }

  
  //THIS IS TERRIBLY SLOW!
  //TODO: fix and optimize set_attribute_list,
  // pub fn set_attribute_list(&mut self, mut attributes: FxHashMap<String, String>) {
  //   swap(&mut self.attributes, &mut attributes);
  //   // XXX: Maybe (collect*2 + union) is faster then (collect + extend)?
  //   let mut keys = attributes.keys().cloned().collect::<FxHashSet<_>>();
  //   keys.extend(self.attributes.keys().cloned());
  //   for key in keys {
  //     let old_value = attributes.get(&key).map(|x| x.as_str());
  //     let new_value = self.attributes.get(&key).map(|x| x.as_str());
  //     self.process_attribute_change(&key, new_value, old_value);
  //   }
  // }
    
  //TODO: remove needless clones in set_attribute/process_attribute_change, maybe clean up?

  pub fn set_attribute(&mut self, key: &str, value: Option<String>) {
    let lc_key = key.to_ascii_lowercase();
    if let Some(value) = value {
      let prev = self.attributes.insert(lc_key.clone(), value.clone());
      self.process_attribute_change(&lc_key, Some(&value), prev.as_deref());
    } else {
      let prev = self.attributes.remove(&lc_key);
      self.process_attribute_change(&lc_key, None, prev.as_deref());
    }
  }

  fn process_attribute_change(&mut self, key: &str, to: Option<&str>, prev: Option<&str>) {
    if prev == to { return }
    match key {
      "class" => {
        self.cache.classes.clear();
        if let Some(to) = to {
          self.cache.classes.extend(to.trim().split(' ').map(|x| x.to_ascii_lowercase()));
        }
      },
      "id" => {
        self.cache.id = to.and_then(|to| (!to.trim().is_empty()).then(|| to.trim().to_string()));
      },
      _ => ()
    }
  }
}

impl InnerHtml for ElementNode {
  fn inner_html(&self) -> String {
    format!(
      "<{}{}{}>{}{}{}{}",
      self.tag_name,
      self.attributes.iter().map(|(k, v)| {
        format!(" {}=\"{}\"", k, v.replace('\\', "\\\\").replace('"', "\\\""))
      }).collect::<String>(),
      if self.children.is_empty() { "/" } else { "" },
      self.children.iter().map(|x| x.0.borrow().inner_html()).collect::<String>(),
      if self.children.is_empty() { "" } else { "</" },
      if self.children.is_empty() { "" } else { self.tag_name.as_str() },
      if self.children.is_empty() { "" } else { ">" },
    )
  }
}

#[derive(Debug, Default)]
pub struct TextNode {
  pub text: String,
  pub parent: Option<WeakNode>,
}

impl InnerHtml for TextNode {
  fn inner_html(&self) -> String {
    //TODO: this can lead tho xss:
    self.text.clone()
  }
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

impl InnerHtml for Node {
  fn inner_html(&self) -> String {
    match self {
      Node::Element(element) => element.inner_html(),
      Node::Text(text) => text.inner_html(),
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
