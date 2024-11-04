use std::fs;
mod scanner;
use scanner::*;
mod parser;
use parser::*;
mod expr;
use expr::*;

fn main() {
   let file_path: &str = "/Users/mihirs/Desktop/Rust/compilerProject/src/test_output.jlox";
   println!("In file {file_path}");

   let contents = fs::read_to_string(file_path)
       .expect("Should have been able to read the file");
    // scanner_main(&contents);
    // let tokens = scanner_main(&contents);
    // parser_main(&tokens);

    let minus_token = Token { token_type: TokenType::MINUS, lexeme: "-".to_owned(), literal: None, line: usize::MAX };
    let ontwothree = Expr::Literal { value: LiteralValue::Number(123.0)};
    let group = Expr::Grouping { expression: Box::new(Expr::Literal { value: LiteralValue::Number(45.67) }) };
    let multi = Token { token_type: TokenType::STAR, lexeme: "*".to_owned(), literal: None, line: usize::MAX };
    let ast = Expr::Binary { left: Box::new(Expr::Unary { operator: minus_token, right: Box::new(ontwothree) }), operator: multi, right: Box::new(group) };
    ast.print();
}


