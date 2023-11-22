use ariadne::{Color, Label, Report, ReportKind, Source};

mod eval;
mod lexer;
mod parser;

#[cfg(test)]
mod eval_tests;
#[cfg(test)]
mod parser_tests;

pub use eval::eval;
pub use parser::{parse, Expr, Program};

/// A high-level function that runs the parser, evaluator and also generates nice errors reports
/// for more control, see `parse` and `eval`.
pub fn run(source: &str) -> Result<String, String> {
    // SEMI-HACK: only the most recent commit of ariadne handles empty sources correctly
    if source.is_empty() {
        return Err("Empty file".into());
    }

    // HACK: for some reason I get invalid spans when there is a perser error with trailing whitespace,
    let source = source.trim_end();

    match parse(source) {
        Ok(program) => match eval(program) {
            // TODO: Add nicer evaulation errors, also using ariadne
            Err(eval_error) => Err(format!("{eval_error:?}")),
            // TODO: Add pretty printer for expressions (and the option to choose between abstract and concrete syntax)
            Ok(value) => Ok(format!("{value:#?}")),
        },
        Err(parse_errors) => {
            let mut output = Vec::<u8>::new();
            for error in parse_errors {
                Report::build(ReportKind::Error, (), error.span().start)
                    .with_message(error.to_string())
                    .with_label(
                        Label::new(error.span().into_range())
                            .with_message(error.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .write_for_stdout(Source::from(source), &mut output)
                    .unwrap();
            }
            Err(std::str::from_utf8(&output).unwrap().to_string())
        }
    }
}

/*
TODO:
- Add wasm bindings
*/
