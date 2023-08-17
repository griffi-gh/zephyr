use breeze::dom::{Dom, InnerHtml};

#[test]
pub fn _0() {
  let dom = Dom::parse(r#"
    <p>This is a cat!</p>
    <img src="foo.png"/>
    <quirky lol="\"" owo="\\" x="\'" y='\"' />
    <button type="button" id="register-button" class="rounded stylish-button">Register</button>
  "#).unwrap();
  println!("{:#?}", dom);
  println!("{}", dom.tree.0.borrow().inner_html());
}
