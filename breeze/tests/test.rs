use breeze::dom::Dom;

#[test]
pub fn _0() {
  println!("{:#?}", Dom::parse(r#"
    <p>This is a cat!</p>
    <img src="foo.png"/>
    <quirky lol="\"" owo="\\" x="\'" y='\"' />
  "#));
}
