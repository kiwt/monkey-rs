use std::io;

use crate::{lexer, token};

pub fn start() {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line.");

    let mut l = lexer::Lexer::new(line);

    loop {
        let tok = l.next_token();
        if tok.token_kind != token::TokenKind::Eof {
            print!("{:?} \n", tok);
            continue;
        } else {
            break;
        }
    }
}
