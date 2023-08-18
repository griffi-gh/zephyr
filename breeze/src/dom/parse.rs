use super::{Dom, Node, ElementNode, TextNode, SharedNode, SharedClone};
use rustc_hash::FxHashMap;
use pest::{Parser, iterators::Pair, error::Error as PestError};
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
