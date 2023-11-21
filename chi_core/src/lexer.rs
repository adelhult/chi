use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    // Note: for parsing to be recoverable it should not fail at the lexing stage, so we replace the Err(()) values with Token::Error
    Error,

    #[regex(r"[A-Z]([a-zA-Z0-9_])*")]
    ConstName(&'a str),

    #[regex(r"[a-z_]([a-zA-Z0-9_])*")]
    VarName(&'a str),

    #[token("case")]
    Case,

    #[token("of")]
    Of,

    #[token("rec")]
    Rec,

    // Used in the meta languages
    #[token("let")]
    Let,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(r"\")]
    Backslash,

    #[token(".")]
    Period,

    #[token("=")]
    Equals,

    #[token("->")]
    Arrow,

    #[regex(r"--[^\n]*")]
    Comment,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ConstName(name) => write!(f, "{name}"),
            Token::VarName(name) => write!(f, "{name}"),
            Token::Case => write!(f, "case"),
            Token::Of => write!(f, "of"),
            Token::Rec => write!(f, "rec"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Backslash => write!(f, "\\"),
            Token::Period => write!(f, "."),
            Token::Equals => write!(f, "="),
            Token::Arrow => write!(f, "->"),
            Token::Error => write!(f, "<error>"),
            Token::Comment => write!(f, "<comment>"),
            Token::Let => write!(f, "let"),
        }
    }
}
