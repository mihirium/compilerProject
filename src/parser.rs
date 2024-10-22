use crate::print_token;
use crate::Token;
use crate::TokenType;

pub fn parser_main(tokens: &Vec<Token>) {
    for token in tokens{
        // print!("{}", token.lexeme);
        print_token(&token);
    }
}

// pub enum NODE {
//     Program(Program),
//     Expression(Expression),
//     Statement(Statement)
// }

// pub struct Program {
    
// }

