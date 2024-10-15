
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize
}

pub fn print_token(my_token: Token) {
    println!("{:?}", my_token);
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

pub fn scanner_main(source: String) {
    let mut my_tokens: Vec<Token> = Vec::new();

    for c in source.chars() {
        let new_token = Token {
            token_type: get_token_type(c),
            lexeme: c.to_string(),
            literal: "".to_string(),
            line: 0
        };

        my_tokens.push(new_token);
    }
    let end_token = Token {
        token_type: TokenType::EOF,
        lexeme: "".to_string(),
        literal: "".to_string(),
        line: source.chars().count()
    };
    my_tokens.push(end_token);

    for token in my_tokens {
        println!("{:?}", token);  // Assuming `Token` implements `Debug`
    }
}

pub fn get_token_type(c: char) -> TokenType {
    let token_type = match c {
        '(' => TokenType::LEFTPAREN,
        ')' => TokenType::RIGHTPAREN,
        '{' => TokenType::LEFTBRACE,
        '}' => TokenType::RIGHTBRACE,
        ',' => TokenType::COMMA,
        '.' => TokenType::DOT,
        '-' => TokenType::MINUS,
        '+' => TokenType::PLUS,
        ';' => TokenType::SEMICOLON,
        '*' => TokenType::STAR,
        '/' => TokenType::SLASH,
        _ => TokenType::EOF,
    };
    return token_type;
}



