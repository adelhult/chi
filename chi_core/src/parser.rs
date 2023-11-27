use std::fmt;

use crate::lexer::Token;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub struct Constructor(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(pub(crate) String);

#[derive(Debug, PartialEq, Clone)]
pub struct Branch(
    pub(crate) Constructor,
    pub(crate) Vec<Variable>,
    pub(crate) Expr,
);

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Apply(Box<Self>, Box<Self>),
    Lambda(Variable, Box<Self>),
    Case(Box<Self>, Vec<Branch>),
    Rec(Variable, Box<Self>),
    Var(Variable),
    Const(Constructor, Vec<Self>),
}

/// A layer on top of the Chi language that
/// allows Chi expressions to be assigned to meta variables
#[derive(Debug, PartialEq, Clone)]
pub enum Program {
    Let(Variable, Expr, Box<Self>),
    Expr(Expr),
}

pub fn parse(source: &str) -> Result<Program, Vec<Rich<'_, Token<'_>, SimpleSpan, &str>>> {
    let token_iter = Token::lexer(source)
        .spanned()
        // Convert lexer errors into a Token::Error
        .map(|(token, span)| (token.unwrap_or(Token::Error), span.into()));

    let end_of_input: SimpleSpan = (source.len()..source.len()).into();
    let token_stream = Stream::from_iter(token_iter).spanned(end_of_input);

    program_parser().parse(token_stream).into_result()
}

fn program_parser<'a, I>() -> impl Parser<'a, I, Program, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let constructor_name = select! { Token::ConstName(name) => Constructor(name.to_string()) };
    let var_name = select! { Token::VarName(name) => Variable(name.to_string())};

    let expr = recursive(|expr| {
        let var = var_name.map(|var| Expr::Var(var));

        let args = expr
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>();

        let constructor = constructor_name
            .then(args.delimited_by(just(Token::LParen), just(Token::RParen)))
            .map(|(name, args)| Expr::Const(name, args));

        let vars = var_name
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>();

        let branch = constructor_name
            .then(vars.delimited_by(just(Token::LParen), just(Token::RParen)))
            .then_ignore(just(Token::Arrow))
            .then(expr.clone())
            .map(|((constructor, vars), e)| Branch(constructor, vars, e));

        let case = just(Token::Case)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Of))
            .then(
                branch
                    .separated_by(just(Token::Semicolon))
                    .allow_trailing()
                    .collect::<Vec<Branch>>()
                    .delimited_by(just(Token::LCurly), just(Token::RCurly)),
            )
            .map(|(e, branches)| Expr::Case(Box::new(e), branches));

        let lambda = just(Token::Backslash)
            .ignore_then(var_name)
            .then_ignore(just(Token::Period))
            .then(expr.clone())
            .map(|(var, e)| Expr::Lambda(var, Box::new(e)));

        let rec = just(Token::Rec)
            .ignore_then(var_name)
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .map(|(var, e)| Expr::Rec(var, Box::new(e)));

        let atom = var.or(constructor).or(case).or(lambda).or(rec).or(expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen)));

        let apply = atom.clone().foldl(atom.clone().repeated(), |a, b| {
            Expr::Apply(Box::new(a), Box::new(b))
        });

        apply
    });

    let program = recursive(|program| {
        let let_ = just(Token::Let)
            .ignore_then(var_name)
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .then_ignore(just(Token::Semicolon))
            .then(program)
            .map(|((name, e), rest)| Program::Let(name, e, Box::new(rest)));

        let_.or(expr.map(|e| Program::Expr(e)))
    });

    program.then_ignore(end())
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
