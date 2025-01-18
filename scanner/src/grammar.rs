// This file is auto-generated by the preprocessor
// The preprocessor reads the .lex file and generates the Rust code for logos
// To regenerate this file, run the preprocessor with the following command:
// cargo run -p preprocessor
        
use logos::Logos;
use std::fmt;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[0-9]+\.[0-9]+(e[-+]?[0-9]+)?", |lex| lex.slice().parse().map_err(|_| ()))]
        REAL(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
        INT(String),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
        VAR(String),
    #[regex(r"\+")]
        ADD,
    #[regex(r"-")]
        SUB,
    #[regex(r"\*")]
        MUL,
    #[regex(r"/")]
        DIV,
    #[regex(r"//")]
        INTDIV,
    #[regex(r"\^")]
        POW,
    #[regex(r"\(")]
        LPAREN,
    #[regex(r"\)")]
        RPAREN,
    #[regex(r"\[")]
        LBRACKET,
    #[regex(r"\]")]
        RBRACKET,
    #[regex(r"==")]
        EQ,
    #[regex(r"!=")]
        NE,
    #[regex(r"<=")]
        LE,
    #[regex(r">=")]
        GE,
    #[regex(r"<")]
        LT,
    #[regex(r">")]
        GT,
    #[regex(r"=")]
        ASSIGN,
    #[regex(r"list")]
        LIST,
    #[regex(r"\s+", logos::skip, priority = 1)]
        WHITESPACE,
    #[regex(r"\n")]
        NEWLINE,
    #[allow(dead_code)]
    ERR,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::REAL(s) => write!(f, "{}", s),
            Self::INT(s) => write!(f, "{}", s),
            Self::VAR(s) => write!(f, "{}", s),
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MUL => write!(f, "*"),
            Self::DIV => write!(f, "/"),
            Self::INTDIV => write!(f, "//"),
            Self::POW => write!(f, "^"),
            Self::LPAREN => write!(f, "("),
            Self::RPAREN => write!(f, ")"),
            Self::LBRACKET => write!(f, "["),
            Self::RBRACKET => write!(f, "]"),
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
            Self::LE => write!(f, "<="),
            Self::GE => write!(f, ">="),
            Self::LT => write!(f, "<"),
            Self::GT => write!(f, ">"),
            Self::ASSIGN => write!(f, "="),
            Self::LIST => write!(f, "list"),
            Self::WHITESPACE => write!(f, "<whitespace>"),
            Self::ERR => write!(f, "<error>"),
            Self::NEWLINE => write!(f, "<newline>"),
        }
    }
}
