use crate::{dom::{SharedNode, ElementNode, Node}, layout::ComputedElementLayout};
use nalgebra::Vector2;

///Internal function\
///Panics if node is a Text node
fn element_node<'a>(node: &'a SharedNode) -> &'a ElementNode {
  let x = node.0.borrow();
  match &*x {
    Node::Element(node) => node,
    _ => unreachable!()
  }
}

pub trait ElementInterface {
  /// Compute node's internal content size\
  /// 
  /// Defaults to a 10x10 square (if no children)\
  /// You may want to change that, for example, for images you should return the image size here.
  /// 
  /// Should never get called for text nodes\
  /// (they cant have ElementInterface anyway, 
  /// if you need to do that you're doing sth *very* wrong)\
  /// 
  /// if regular box layout w/children, should return None
  /// 
  /// not guaranteed to match the actual size after the layout stage.\
  /// this is not exact and it can and will be overriden by styles.
  fn compute_content_size(&self, node: &SharedNode) -> Option<Vector2<f32>> {
    element_node(node).children.is_empty().then_some(Vector2::new(10., 10.))
  }

  //compute_layout_
}

pub struct DefaultNode;
impl ElementInterface for DefaultNode {}
