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

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input.as_bytes()[self.read_position]
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok;
        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let current_ch = String::from_utf8(vec![self.ch]).unwrap();
                    self.read_char();
                    let next_ch = String::from_utf8(vec![self.ch]).unwrap();
                    tok = Token {
                        token_kind: TokenKind::Eq,
                        literal: format!("{}{}", current_ch, next_ch),
                    };
                } else {
                    tok = Self::new_token(TokenKind::Assign, self.ch);
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    let current_ch = String::from_utf8(vec![self.ch]).unwrap();
                    self.read_char();
                    let next_ch = String::from_utf8(vec![self.ch]).unwrap();
                    tok = Token {
                        token_kind: TokenKind::NotEq,
                        literal: format!("{}{}", current_ch, next_ch),
                    };
                } else {
                    tok = Self::new_token(TokenKind::Bang, self.ch);
                }
            }
            b'+' => tok = Self::new_token(TokenKind::Plus, self.ch),
            b'-' => tok = Self::new_token(TokenKind::Minus, self.ch),
            b'/' => tok = Self::new_token(TokenKind::Slash, self.ch),
            b'*' => tok = Self::new_token(TokenKind::Asterisk, self.ch),
            b'<' => tok = Self::new_token(TokenKind::Lt, self.ch),
            b'>' => tok = Self::new_token(TokenKind::Gt, self.ch),
            b';' => tok = Self::new_token(TokenKind::Semicolon, self.ch),
            b'(' => tok = Self::new_token(TokenKind::LParen, self.ch),
            b')' => tok = Self::new_token(TokenKind::RParen, self.ch),
            b',' => tok = Self::new_token(TokenKind::Comma, self.ch),
            b'{' => tok = Self::new_token(TokenKind::LBrace, self.ch),
            b'}' => tok = Self::new_token(TokenKind::RBrace, self.ch),
            0 => {
                tok = Token {
                    token_kind: TokenKind::Eof,
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
                        token_kind: TokenKind::Int,
                        literal: ident,
                    };
                    return tok;
                } else {
                    tok = Token {
                        token_kind: TokenKind::Illegal,
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
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            ",
        );
        let mut lexer = Lexer::new(input);

        let tests = vec![
            (TokenKind::Let, String::from("let")),
            (TokenKind::Ident, String::from("five")),
            (TokenKind::Assign, String::from("=")),
            (TokenKind::Int, String::from("5")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Let, String::from("let")),
            (TokenKind::Ident, String::from("ten")),
            (TokenKind::Assign, String::from("=")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Let, String::from("let")),
            (TokenKind::Ident, String::from("add")),
            (TokenKind::Assign, String::from("=")),
            (TokenKind::Function, String::from("fn")),
            (TokenKind::LParen, String::from("(")),
            (TokenKind::Ident, String::from("x")),
            (TokenKind::Comma, String::from(",")),
            (TokenKind::Ident, String::from("y")),
            (TokenKind::RParen, String::from(")")),
            (TokenKind::LBrace, String::from("{")),
            (TokenKind::Ident, String::from("x")),
            (TokenKind::Plus, String::from("+")),
            (TokenKind::Ident, String::from("y")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::RBrace, String::from("}")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Let, String::from("let")),
            (TokenKind::Ident, String::from("result")),
            (TokenKind::Assign, String::from("=")),
            (TokenKind::Ident, String::from("add")),
            (TokenKind::LParen, String::from("(")),
            (TokenKind::Ident, String::from("five")),
            (TokenKind::Comma, String::from(",")),
            (TokenKind::Ident, String::from("ten")),
            (TokenKind::RParen, String::from(")")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Bang, String::from("!")),
            (TokenKind::Minus, String::from("-")),
            (TokenKind::Slash, String::from("/")),
            (TokenKind::Asterisk, String::from("*")),
            (TokenKind::Int, String::from("5")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Int, String::from("5")),
            (TokenKind::Lt, String::from("<")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::Gt, String::from(">")),
            (TokenKind::Int, String::from("5")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::If, String::from("if")),
            (TokenKind::LParen, String::from("(")),
            (TokenKind::Int, String::from("5")),
            (TokenKind::Lt, String::from("<")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::RParen, String::from(")")),
            (TokenKind::LBrace, String::from("{")),
            (TokenKind::Return, String::from("return")),
            (TokenKind::True, String::from("true")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::RBrace, String::from("}")),
            (TokenKind::Else, String::from("else")),
            (TokenKind::LBrace, String::from("{")),
            (TokenKind::Return, String::from("return")),
            (TokenKind::False, String::from("false")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::RBrace, String::from("}")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::Eq, String::from("==")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Int, String::from("10")),
            (TokenKind::NotEq, String::from("!=")),
            (TokenKind::Int, String::from("9")),
            (TokenKind::Semicolon, String::from(";")),
            (TokenKind::Eof, String::from("")),
        ];

        for test in tests.iter() {
            let _tok = lexer.next_token();
            print!("{:?} \n", _tok);
            assert_eq!(_tok.token_kind, test.0);
            assert_eq!(_tok.literal, test.1);
        }
    }
}
