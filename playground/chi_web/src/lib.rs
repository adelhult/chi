mod utils;

use chi_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: &str) -> Result<String, String> {
    utils::set_panic_hook();
    chi_core::run(source)
}
