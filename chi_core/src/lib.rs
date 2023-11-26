use ariadne::{Color, Label, Report, ReportKind, Source};

mod error;
mod eval;
mod lexer;
mod parser;
mod pretty;

#[cfg(test)]
mod eval_tests;
#[cfg(test)]
mod parser_tests;

pub use error::Error;
pub use eval::eval;
pub use parser::{parse, Expr, Program};

/// A high-level function that runs the parser, evaluator and also generates nice errors reports
/// for more control, see `parse` and `eval`.
pub fn run(source: &str, printer: Printer) -> Result<String, String> {
    // Only the most recent commit of ariadne handles empty sources correctly, so we ignore empty files
    if source.is_empty() {
        return Err("Empty file".into());
    }

    match parse(source) {
        Ok(program) => match eval(program) {
            // TODO: Add nicer evaulation errors, also using ariadne
            Err(eval_error) => Err(format!(r#"<span class="error">{eval_error}</span>"#)),
            Ok(value) => Ok(match printer {
                Printer::Concrete => pretty::concrete(&value),
                Printer::Abstract => pretty::abstr(&value),
                Printer::Debug => format!("{value:#?}"),
            }),
        },
        Err(parse_errors) => {
            let mut output = Vec::<u8>::new();
            for error in parse_errors {
                // MAJOR HACK: for some reason I get spans that end before they start
                // (when trailing whitespace/comments and let bindings involved?)
                // so if that is the case, we will just flip them
                let span = if error.span().start > error.span().end {
                    error.span().end..error.span().start
                } else {
                    error.span().start..error.span().end
                };

                Report::build(ReportKind::Error, (), span.start)
                    .with_message(error.to_string())
                    .with_label(
                        Label::new(span)
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

#[derive(Clone, Copy)]
pub enum Printer {
    Concrete,
    Abstract,
    Debug,
}

impl TryFrom<&str> for Printer {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "concrete" => Ok(Printer::Concrete),
            "abstract" => Ok(Printer::Abstract),
            "debug" => Ok(Printer::Debug),
            _ => Err(()),
        }
    }
}
