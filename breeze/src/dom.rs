use crate::parsers::html::{Parser as HtmlParser, Rule as HtmlRule};
use std::{rc::{Rc, Weak}, cell::RefCell, vec};
use rustc_hash::FxHashMap;
use pest::{Parser, iterators::Pair, error::Error as PestError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomParseError {
  #[error("failed to parse html")]
  ParseError(#[from] PestError<HtmlRule>),
}

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

#[derive(Debug)]
pub struct Dom {
  pub tree: SharedNode
}

impl Dom {
  //This function should never panic if the implementation is correct!
  pub fn parse(html: &str) -> Result<Self, DomParseError> {
    fn parse_tree(parent_node: &SharedNode, tree_thingy: Pair<HtmlRule>) {
      assert_eq!(tree_thingy.as_rule(), HtmlRule::tree);
      for node_thingy in tree_thingy.into_inner() {

        assert_eq!(node_thingy.as_rule(), HtmlRule::node);
        let node_subtype = node_thingy.into_inner().next().unwrap();
        let node_subtype_rule = node_subtype.as_rule();

        match node_subtype_rule {
          //Full tags: <b>...</b>, and void tags: <img>, <div/>
          HtmlRule::full_tag | HtmlRule::void_tag => {
            let mut node_subtype_inner = node_subtype.into_inner();

            let opening_tag_or_ident = node_subtype_inner.next().unwrap();

            //parse opening tag (get name and attributes)
            let (tag_name, attributes) = {
              let (attributes_pairs, tag_name) = match opening_tag_or_ident.as_rule() {
                HtmlRule::opening_tag => {
                  let mut opening_tag_inner = opening_tag_or_ident.into_inner();
                  let tag_name_pair = opening_tag_inner.next().unwrap();
                  assert_eq!(tag_name_pair.as_rule(), HtmlRule::ident);
                  let tag_name = tag_name_pair.as_str().into();
                  let attributes_pair = opening_tag_inner.next();
                  (attributes_pair, tag_name)
                },
                HtmlRule::ident | HtmlRule::void_tag_name => {
                  let tag_name = opening_tag_or_ident.as_str().into();
                  let attributes_pair = node_subtype_inner.next();
                  (attributes_pair, tag_name)
                },
                _ => unreachable!()
              };
              
              //parse attributes
              let mut attributes = FxHashMap::default();
              if let Some(attributes_pairs) = attributes_pairs {
                for attribute_pair in attributes_pairs.into_inner() {
                  assert_eq!(attribute_pair.as_rule(), HtmlRule::attribute);
                  let mut attribute_inner = attribute_pair.into_inner();

                  let attribute_ident = attribute_inner.next().unwrap();
                  assert_eq!(attribute_ident.as_rule(), HtmlRule::ident);
                  let attribute_name = attribute_ident.as_str().to_owned();

                  let attribute_value: String = if let Some(str_pair) = attribute_inner.next() {
                    match str_pair.as_rule() {
                      HtmlRule::value_proper => {
                        let mut as_str = &str_pair.as_str()[1..];
                        as_str = &as_str[..(as_str.len() - 1)];
                        as_str.replace("\\\\", "\\")
                      },
                      HtmlRule::value_naked => str_pair.as_str().into(),
                      _ => unreachable!()
                    }
                  } else {
                    "".into()
                  };

                  attributes.insert(attribute_name, attribute_value);
                }
              }
              (tag_name, attributes)
            };
            
            //Create and insert new node
            let new_node: SharedNode = Node::Element(ElementNode {
              tag_name, attributes,
              ..Default::default()
            }).into();
            println!("NODE {:?}", new_node);
            parent_node.push(new_node.shared_clone()).unwrap();

            //If full tag, recursively parse children
            if node_subtype_rule == HtmlRule::full_tag {
              let ptree = node_subtype_inner.next().unwrap();
              assert!(ptree.as_rule() == HtmlRule::tree);
              parse_tree(&new_node, ptree);
            }

            // ensure correctness of the parsed tree?
            // let closing_tag = node_subtype_inner.next().unwrap();
            // assert!(closing_tag.as_rule() == HtmlRule::closing_tag);
          },

          //Text nodes
          HtmlRule::text_node => {
            let new_node: SharedNode = Node::Text(TextNode {
              text: node_subtype.as_str().into(),
              ..Default::default()
            }).into();
            println!("TEXT {:?}", new_node);
            parent_node.push(new_node).unwrap();
          },

          _ => unreachable!(),
        }
      }
    }

    let tree = SharedNode::root();

    let mut pairs = HtmlParser::parse(HtmlRule::dom, html)?;
    let dom = pairs.next().unwrap(); //Never fails
    for thingy in dom.into_inner() {
      match thingy.as_rule() {
        HtmlRule::tree => parse_tree(&tree, thingy),
        HtmlRule::EOI | HtmlRule::preamble => (),
        _ => unreachable!(),
      }
    }

    Ok(Self { tree })
  }
}
