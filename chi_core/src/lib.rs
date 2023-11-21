mod eval;
mod lexer;
mod parser;

#[cfg(test)]
mod eval_tests;
#[cfg(test)]
mod parser_tests;

pub use eval::eval;
pub use parser::{parse, Expr};

/*
TODO:
- Fix application bug
- Add evaluator
- Add wasm bindings
- Extend with a meta-language/preprocessor
*/
