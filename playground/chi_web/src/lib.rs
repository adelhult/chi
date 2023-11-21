mod utils;

use chi_core::{eval, parse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: &str) -> String {
    let program = parse(source);
    let expr = program.map(eval);
    format!("{expr:?}")
}
