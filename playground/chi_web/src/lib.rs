mod utils;

use chi_core;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: &str, printer: String) -> Result<String, String> {
    utils::set_panic_hook();
    let printer = printer.as_str().try_into().unwrap();
    chi_core::run(source, printer)
}
