use crate::scanner::Literal;
use crate::scanner::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print(&self, expr: &Expr) -> String {
        let mut builder = String::new();

       let result = match expr {
            Expr::Binary(left, token, right) => {
                self.parenthesize(&token.lexeme, &[left, right])
            }
            Expr::Grouping(expr) => {
                self.parenthesize(&"grouping".to_string(), &[expr])
            }
            Expr::Literal(literal) => match literal {
                Literal::Number(number) => number.to_string(),
                Literal::String(string) => string.clone(),
            },
            Expr::Unary(token, expr) => {
                self.parenthesize(&token.lexeme, &[expr])
            }
        };

        builder.push_str(result.as_str());

        builder.to_string()
    }

    fn parenthesize(&self, name: &String, expr: &[&Box<Expr>]) -> String {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name.as_str());
        builder.push_str(" ");
        let expressions = expr
            .iter()
            .map(|expr| self.print(expr))
            .collect::<Vec<_>>()
            .join(" ");
        builder.push_str(expressions.as_str());
        builder.push_str(")");
        builder.to_string()
    }
}
