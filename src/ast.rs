use std::fmt;

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Statement").finish()
    }
}

impl PartialEq<dyn Statement> for dyn Statement {
    fn eq(&self, other: &dyn Statement) -> bool {
        self.statement_node() == other.statement_node()
    }
}
#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&mut self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::from("");
        }
    }
}

pub struct LetStatement {
    pub token: Token, // the 'let' token.
    pub name: Identifier,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token: Token, // the 'return' token;
    pub return_value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }
}

pub struct Identifier {
    pub token: Token, // the 'ident' token.
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
