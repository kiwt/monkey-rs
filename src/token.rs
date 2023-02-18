type TokenType = String;

pub struct Token {
    pub token_kind: TokenKind,
    pub literal: String,
}

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
