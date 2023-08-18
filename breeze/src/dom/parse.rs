use super::{Dom, Node, ElementNode, TextNode, SharedNode, SharedClone};
use rustc_hash::FxHashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomParseError {
  
}

impl Dom {
  //This function should never panic if the implementation is correct!
  pub fn parse(html: &str) -> Result<Self, DomParseError> {
    todo!()
  }
}
