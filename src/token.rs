#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Illegal, // illegal
    Eof,     // eof

    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 12343456

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Lt,       // <
    Gt,       // >

    // Delimiters
    Comma,     // ,
    Semicolon, // ;

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Keywords
    Function, // fn
    Let,      // let
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Return,   // return

    Eq,    // ==
    NotEq, // !=

    Default,
}

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident,
    }
}
