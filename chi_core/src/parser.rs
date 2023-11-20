use crate::lexer::Token;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

#[derive(Debug, PartialEq)]
pub struct ConstName(pub(crate) String);

#[derive(Debug, PartialEq)]
pub struct VarName(pub(crate) String);

#[derive(Debug, PartialEq)]
pub struct Branch(
    pub(crate) ConstName,
    pub(crate) Vec<VarName>,
    pub(crate) Expr,
);

#[derive(Debug, PartialEq)]
pub enum Expr {
    Apply(Box<Self>, Box<Self>),
    Lambda(VarName, Box<Self>),
    Case(Box<Self>, Vec<Branch>),
    Rec(VarName, Box<Self>),
    Var(VarName),
    Const(ConstName, Vec<Self>),
}

pub fn parse(source: &str) -> Result<Expr, Vec<Rich<'_, Token<'_>, SimpleSpan, &str>>> {
    let token_iter = Token::lexer(source)
        .spanned()
        // Convert lexer errors into a Token::Error
        .map(|(token, span)| (token.unwrap_or(Token::Error), span.into()));

    let end_of_input: SimpleSpan = (source.len()..source.len()).into();
    let token_stream = Stream::from_iter(token_iter).spanned(end_of_input);

    expr_parser()
        .then_ignore(end())
        .parse(token_stream)
        .into_result()
}

fn expr_parser<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let constructor_name = select! { Token::ConstName(name) => ConstName(name.to_string()) };
    let var_name = select! { Token::VarName(name) => VarName(name.to_string())};

    recursive(|expr| {
        let var = var_name.map(|var| Expr::Var(var));

        let args = expr
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>();

        let constructor = constructor_name
            .then(args.delimited_by(just(Token::LParen), just(Token::RParen)))
            .map(|(name, args)| Expr::Const(name, args));

        let vars = var_name
            .clone()
            .separated_by(just(Token::Comma))
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

        let apply = atom.clone().foldl(expr.clone().repeated(), |a, b| {
            Expr::Apply(Box::new(a), Box::new(b))
        });

        apply.or(atom)
    })
}
