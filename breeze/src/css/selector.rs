pub struct CssSelector {
  pub parts: Vec<CssSelectorPart>,
}

pub enum CssSelectorPart {
  Id(String),
  Class(String),
  Element(String),
  PseudoClass(Box<CssSelectorPart>),
}
