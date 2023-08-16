pub mod html {
  #[derive(pest_derive::Parser)]
  #[grammar = "../grammar/html.pest"]
  pub struct Parser;
}
