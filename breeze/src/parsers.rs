use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/html.pest"]
pub struct HtmlParser;
