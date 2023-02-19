type TokenType = String;

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    ILLEGAL, // ILLEGAL
    EOF,     // EOF

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,   // 12343456

    // Operators
    ASSIGN, // =
    PLUS,   // +

    // Delimiters
    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }

    // Keywords
    FUNCTION, // FUNCTION
    LET,      // LET
}

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
        "fn" => TokenKind::FUNCTION,
        "let" => TokenKind::LET,
        _ => TokenKind::IDENT,
    }
}
