use crate::{dom::{SharedNode, ElementNode, Node}, layout::ComputedElementLayout};
use nalgebra::Vector2;

///Internal function\
///Panics if node is a Text node
fn element_node(node: &SharedNode) -> &ElementNode {
  match &*node.0.borrow() {
    Node::Element(node) => node,
    _ => unreachable!()
  }
}

pub trait ElementInterface {
  

  /// Compute node's internal content size\
  /// Defaults to a 10x10 square if no children\
  /// Should never get called for text nodes\
  /// (they cant have ElementInterface anyway, 
  /// if you need to do that you're doing sth *very* wrong)
  fn compute_content_size(&self, node: &SharedNode) -> Option<Vector2<f32>> {
    element_node(node).children.is_empty().then_some(Vector2::new(10., 10.))
  }

   //compute_layout_
}

pub struct DefaultNode;
impl ElementInterface for DefaultNode {}
