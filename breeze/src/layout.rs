use nalgebra::Vector2;
use crate::dom::{Dom, Node};

// trait ComputeLayout/Measure???

#[derive(Debug)]
pub struct ComputedElementLayout {
  pub position: Vector2<f32>,
  pub size: Vector2<f32>,
}

impl Dom {
  pub fn compute_layout(&self) {
    todo!()
  }
}

impl Node {
  pub fn compute_layout(&self) {
    todo!()
  }
}
