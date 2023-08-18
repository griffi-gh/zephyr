// macro_rules! define {
//   ($name: ident, $path: literal) => {
//     pub mod $name {
//       #[derive(::pest_derive::Parser)]
//       #[grammar = $path]
//       pub struct Parser;
//     }
//   };
// }

// define!(html, "../grammar/html.pest");
// define!(css,  "../grammar/css.pest");
