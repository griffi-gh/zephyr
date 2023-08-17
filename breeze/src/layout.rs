use nalgebra::Vector2;
use crate::dom::Dom;

// trait ComputeLayout/Measure???

pub struct ComputedLayout {
  dimensions: Vector2<f32>,
}

impl Dom {
  pub fn compute_layout(&self) -> ComputedLayout {
    todo!()
  }
}
