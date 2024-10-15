use std::fs;
mod scanner;
use scanner::*;

fn main() {
   let file_path: &str = "/Users/mihirs/Desktop/Rust/compilerProject/src/test_output.rs";
   println!("In file {file_path}");

   let contents = fs::read_to_string(file_path)
       .expect("Should have been able to read the file");

   let std_output: String = contents.replace("\n","");
   println!("{std_output}");

// let my_token = Token {
//     token_type: TokenType::AND,
//     lexeme:"hello".to_string(),
//     literal: "maybe".to_string(),
//     line: 40
//    };
//    print_token(my_token);
    scanner_main(std_output);

}


