pub(crate) mod consts;
pub(crate) mod grammar;
pub(crate) mod layout;
pub mod dom;

pub struct BreezeInstance {
  pub dom: dom::Dom,
}
