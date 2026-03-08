use liblox::scanner::{Literal, Token, TokenType};
use liblox::{AstPrinter, Expr};
use std::io::Write;

fn main() {
    let expr = Expr::Binary(
        Box::new(Expr::Literal(Literal::Number(1.0))),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Box::new(Expr::Grouping(Box::new(Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), None, 2),
            Box::new(Expr::Literal(Literal::Number(1.0))),
        )))),
    );

    let ast = AstPrinter::new().print(&expr);

    println!("{}", ast);

    std::io::stdout().flush().expect("Failed to flush stdout");
}
