use std::process::id;

use crate::token::{lookup_ident, Token, TokenKind};
#[derive(PartialEq)]
pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current ch
    ch: u8,               // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn new_token(token_kind: TokenKind, ch: u8) -> Token {
        Token {
            token_kind,
            literal: String::from_utf8(vec![ch]).unwrap(),
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn is_letter(ch: &u8) -> bool {
        let ch = char::from(*ch);
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn is_digit(ch: &u8) -> bool {
        let ch = char::from(*ch);
        '0' <= ch && ch <= '9'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok;
        match self.ch {
            b'=' => tok = Self::new_token(TokenKind::ASSIGN, self.ch),
            b';' => tok = Self::new_token(TokenKind::SEMICOLON, self.ch),
            b'(' => tok = Self::new_token(TokenKind::LPAREN, self.ch),
            b')' => tok = Self::new_token(TokenKind::RPAREN, self.ch),
            b',' => tok = Self::new_token(TokenKind::COMMA, self.ch),
            b'+' => tok = Self::new_token(TokenKind::PLUS, self.ch),
            b'{' => tok = Self::new_token(TokenKind::LBRACE, self.ch),
            b'}' => tok = Self::new_token(TokenKind::RBRACE, self.ch),
            0 => {
                tok = Token {
                    token_kind: TokenKind::EOF,
                    literal: String::from(""),
                };
            }
            _ => {
                if Self::is_letter(&self.ch) {
                    let ident = self.read_identifier();
                    let tk_kind = lookup_ident(&ident);
                    tok = Token {
                        token_kind: tk_kind,
                        literal: ident,
                    };
                    return tok;
                } else if Self::is_digit(&self.ch) {
                    let ident = self.read_number();
                    tok = Token {
                        token_kind: TokenKind::INT,
                        literal: ident,
                    };
                    return tok;
                } else {
                    tok = Token {
                        token_kind: TokenKind::ILLEGAL,
                        literal: String::from(""),
                    };
                }
            }
        }
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenKind;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
            let ten = 10;
               let add = fn(x, y) {
                 x + y;
            };
            let result = add(five, ten);",
        );
        let mut lexer = Lexer::new(input);

        let tests = vec![
            (TokenKind::LET, String::from("let")),
            (TokenKind::IDENT, String::from("five")),
            (TokenKind::ASSIGN, String::from("=")),
            (TokenKind::INT, String::from("5")),
            (TokenKind::SEMICOLON, String::from(";")),
            (TokenKind::LET, String::from("let")),
            (TokenKind::IDENT, String::from("ten")),
            (TokenKind::ASSIGN, String::from("=")),
            (TokenKind::INT, String::from("10")),
            (TokenKind::SEMICOLON, String::from(";")),
            (TokenKind::LET, String::from("let")),
            (TokenKind::IDENT, String::from("add")),
            (TokenKind::ASSIGN, String::from("=")),
            (TokenKind::FUNCTION, String::from("fn")),
            (TokenKind::LPAREN, String::from("(")),
            (TokenKind::IDENT, String::from("x")),
            (TokenKind::COMMA, String::from(",")),
            (TokenKind::IDENT, String::from("y")),
            (TokenKind::RPAREN, String::from(")")),
            (TokenKind::LBRACE, String::from("{")),
            (TokenKind::IDENT, String::from("x")),
            (TokenKind::PLUS, String::from("+")),
            (TokenKind::IDENT, String::from("y")),
            (TokenKind::SEMICOLON, String::from(";")),
            (TokenKind::RBRACE, String::from("}")),
            (TokenKind::SEMICOLON, String::from(";")),
            (TokenKind::LET, String::from("let")),
            (TokenKind::IDENT, String::from("result")),
            (TokenKind::ASSIGN, String::from("=")),
            (TokenKind::IDENT, String::from("add")),
            (TokenKind::LPAREN, String::from("(")),
            (TokenKind::IDENT, String::from("five")),
            (TokenKind::COMMA, String::from(",")),
            (TokenKind::IDENT, String::from("ten")),
            (TokenKind::RPAREN, String::from(")")),
            (TokenKind::SEMICOLON, String::from(";")),
            (TokenKind::EOF, String::from("")),
        ];

        for test in tests.iter() {
            let _tok = lexer.next_token();
            print!("{:?} \n", _tok);
            assert_eq!(_tok.token_kind, test.0);
            assert_eq!(_tok.literal, test.1);
        }
    }
}
