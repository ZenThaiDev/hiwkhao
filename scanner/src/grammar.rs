// This file is auto-generated by the preprocessor
// The preprocessor reads the .lex file and generates the Rust code for logos
// To regenerate this file, run the preprocessor with the following command:
// cargo run -p preprocessor
        
use logos::Logos;
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
            #[regex(r"[0-9]+\.[0-9]+(e[-+]?[0-9]+)?")]
        REAL,
            #[regex(r"[0-9]+")]
        INT,
            #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
        VAR,
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
            #[regex(r"\s+", logos::skip)]
        WHITESPACE,
            ERR,
}
