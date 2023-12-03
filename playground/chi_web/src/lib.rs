mod utils;

use chi_core::{pretty, Coder, Printer};
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(source: &str, printer: String) -> Result<String, String> {
    utils::set_panic_hook();
    let printer = printer.as_str().try_into().unwrap();
    match chi_core::run(source, printer) {
        Ok((output, coder)) => {
            if printer == Printer::Concrete {
                let defined_symbols: String = coder
                    .defined_symbols()
                    .into_iter()
                    .map(|(symbol, expr)| {
                        format!("<li>⌜<u>{symbol}</u>⌝ = {}", pretty::concrete(&expr))
                    })
                    .collect();

                Ok(format!(
                    r#"<ul class="symbols">{defined_symbols}</ul>{output}"#
                ))
            } else {
                Ok(output)
            }
        }
        Err(error) => Err(error),
    }
}
