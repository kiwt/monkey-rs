use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement,
        Statement,
    },
    lexer::Lexer,
    parser::Precedence::Lowest,
    token::{Token, TokenKind},
};

pub enum Precedence {
    Lowest,
}

pub struct Parser {
    l: Lexer,

    pub errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,

            errors: vec![],

            cur_token: Token {
                token_kind: TokenKind::Default,
                literal: String::from(""),
            },
            peek_token: Token {
                token_kind: TokenKind::Default,
                literal: String::from(""),
            },
        };
        // Read two tokens, so curToken and peekToken are both set.
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token()
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };
        while self.cur_token.token_kind != TokenKind::Eof {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_kind {
            TokenKind::Let => Some(Box::new(self.parse_let_statement()?)),
            TokenKind::Return => Some(Box::new(self.parse_return_statement()?)),
            _ => None,
        }
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let tok = self.cur_token.clone();

        let exp = self.parse_expression(Lowest).unwrap();

        let stmt = ExpressionStatement {
            token: tok,
            expression: exp,
        };

        Some(stmt)
    }

    fn parse_expression(&mut self, p: Precedence) -> Option<Box<dyn Expression>> {
        None
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let tok = self.cur_token.clone();

        if !self.expect_peek(TokenKind::Ident) {
            return None;
        }

        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenKind::Assign) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.cur_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let val = self.parse_expression(Lowest).unwrap();

        let stmt = LetStatement {
            token: tok,
            name: ident,
            value: val,
        };

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let tok = self.cur_token.clone();

        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        self.next_token();

        // TODO: We're skipping the expressions until we // encounter a semicolon
        while !self.cur_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let stmt = ReturnStatement {
            token: tok,
            return_value: Box::new(ident),
        };

        Some(stmt)
    }

    fn cur_token_is(&self, t: TokenKind) -> bool {
        self.cur_token.token_kind == t
    }

    fn peek_token_is(&self, t: TokenKind) -> bool {
        self.peek_token.token_kind == t
    }

    fn expect_peek(&mut self, t: TokenKind) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t.clone());
            return false;
        }
    }

    fn peek_error(&mut self, t: TokenKind) {
        let msg = format!(
            "expected next token to be {:?} got {:?} instead",
            t, self.peek_token.token_kind
        );
        self.errors.push(msg);
    }
}
#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_let_statements() {
        let input = String::from(
            "
            return 5;
            return 10;
            return 993322;
            ",
        );

        let lexer = Lexer::new(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3)
    }
}
