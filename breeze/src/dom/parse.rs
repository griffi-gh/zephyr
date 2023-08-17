use super::{Dom, Node, ElementNode, TextNode, SharedNode, SharedClone};
use rustc_hash::FxHashMap;
use pest::{Parser, iterators::Pair, error::Error as PestError};
use thiserror::Error;

mod html_parser {
  #[derive(pest_derive::Parser)]
  #[grammar = "../grammar/html.pest"]
  pub struct HtmlParser;
}
use html_parser::{HtmlParser, Rule as HtmlRule};

#[derive(Error, Debug)]
pub enum DomParseError {
  #[error("failed to parse html")]
  ParseError(#[from] Box<PestError<HtmlRule>>),
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
                        let &(chr, mut as_str) = &str_pair.as_str().split_at(1);
                        as_str = &as_str[..(as_str.len() - 1)];
                        as_str.replace("\\\\", "\\").replace(
                          if chr == "\"" { "\\\"" } else if chr == "'" { "\\'" } else { unreachable!() },
                          if chr == "\"" { "\"" } else if chr == "'" { "'" } else { unreachable!() },
                        )
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

              // ensure correctness of the parsed tree?
              // let closing_tag = node_subtype_inner.next().unwrap();
              // assert!(closing_tag.as_rule() == HtmlRule::closing_tag);
            }
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

    let mut pairs = HtmlParser::parse(HtmlRule::dom, html).map_err(Box::new)?;
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
