use crate::scanner::Literal;
use crate::scanner::Token;
use std::fmt::{Display, Formatter, Result};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Binary(left, token, right) => {
                parenthesize(f, &token.lexeme, &[left, right])
            }
            Expr::Grouping(expr) => {
                parenthesize(f, "grouping", &[expr])
            }
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Unary(token, expr) => {
                parenthesize(f, &token.lexeme, &[expr])
            }
        }
    }
}

fn parenthesize(f: &mut Formatter<'_>, name: &str, exprs: &[&Box<Expr>]) -> Result {
    write!(f, "({}", name)?;
    for expr in exprs {
        write!(f, " {}", expr)?;
    }
    write!(f, ")")
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&self, expr: &Expr) -> String {
        expr.to_string()
    }
}
