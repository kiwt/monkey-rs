use std::{any::Any, fmt};

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
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

    fn string(&mut self) -> String {
        let mut out = String::new();
        for stmt in self.statements.iter() {
            out.push_str(&stmt.string());
        }

        out
    }
}

pub struct LetStatement {
    pub token: Token, // the 'let' token.
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");

        out.push_str(&self.name.value);
        out.push_str(" = ");

        match self.value.as_any().downcast_ref::<Identifier>() {
            Some(val) => out.push_str(&val.string()),
            _ => (),
        };

        out.push_str(";");

        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token: Token, // the 'return' token;
    pub return_value: Box<dyn Expression>,
}

impl ReturnStatement {}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());

        match self.return_value.as_any().downcast_ref::<Identifier>() {
            Some(idt) => out.push_str(&idt.string()),
            _ => (),
        };

        out.push_str(" ");
        out.push_str(";");

        out
    }
}

pub struct ExpressionStatement {
    pub token: Token, // the first token of the expression
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }

    fn string(&self) -> String {
        let out = String::new();

        match self
            .expression
            .as_any()
            .downcast_ref::<Box<dyn Expression>>()
        {
            Some(e) => return e.string(),
            _ => (),
        };

        out
    }
}

pub struct Identifier {
    pub token: Token, // the 'ident' token.
    pub value: String,
}

impl Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        let l = self.token.literal.to_string();
        l
    }

    fn string(&self) -> String {
        let l = self.value.to_string();
        l
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenKind};

    use crate::ast::{Expression, Identifier, LetStatement, Program};

    #[test]
    fn test_string() {
        let mut program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_kind: TokenKind::Let,
                    literal: String::from("let"),
                },
                name: Identifier {
                    token: Token {
                        token_kind: TokenKind::Ident,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVar"),
                },
                value: Box::new(Identifier {
                    token: Token {
                        token_kind: TokenKind::Ident,
                        literal: String::from("anotherVar"),
                    },
                    value: String::from("anotherVar"),
                }) as Box<dyn Expression>,
            })],
        };

        assert_eq!(program.string(), "let myVar = anotherVar;")
    }
}
