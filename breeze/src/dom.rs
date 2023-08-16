use crate::parsers::html::{Parser as HtmlParser, Rule as HtmlRule};
use std::rc::{Rc, Weak};
use rustc_hash::FxHashMap;
use pest::{Parser, error::Error as PestError};
use thiserror::Error;

pub struct DomNode {
  pub tag: String,
  pub attributes: FxHashMap<String, String>,
  pub children: Vec<Rc<Tree>>,
  pub parent: Option<Weak<Tree>>,
}

impl DomNode {
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

pub struct TextNode {
  pub text: String,
  pub parent: Option<Weak<Tree>>,
}

pub enum Tree {
  Node(DomNode),
  Text(TextNode),
}

impl Default for Tree {
  fn default() -> Self {
    Self::Node(DomNode {
      tag: "root".to_string(),
      attributes: FxHashMap::default(),
      children: vec![],
      parent: None,
    })
  }
}

pub struct Dom {
  root: Rc<Tree>
}

#[derive(Error, Debug)]
pub enum HtmlError {
  #[error("failed to parse html")]
  ParseError(#[from] PestError<HtmlRule>),
}

impl Dom {
  pub fn parse(html: &str) -> Result<Self, HtmlError> {
    let mut tree = Tree::default();

    let mut pairs = HtmlParser::parse(HtmlRule::dom, html)?;
    let dom = pairs.next().unwrap(); //Never fails
    for thingy in dom.into_inner() {
      match thingy.as_rule() {
        HtmlRule::tree => {
          for node in thingy.into_inner() {
            match node.as_rule() {
              _ => todo!()
            }
          }
        },
        HtmlRule::EOI
        | HtmlRule::COMMENT
        | HtmlRule::WHITESPACE
        | HtmlRule::preamble => (),
        _ => panic!()
      }
    }
    Ok(todo!())
  }
}
