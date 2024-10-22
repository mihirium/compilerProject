#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

pub fn print_token(my_token: &Token) {
    println!("{:?}", my_token);
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL, PLUSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, PUB, FN, MUT, LET, MOD, USE,

    EOF,
    UNKNOWN, // For unrecognized characters
}

struct Scanner {
    source: Vec<char>,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, None),
            ')' => self.add_token(TokenType::RIGHTPAREN, None),
            '{' => self.add_token(TokenType::LEFTBRACE, None),
            '}' => self.add_token(TokenType::RIGHTBRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),

            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BANGEQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EQUALEQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LESSEQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GREATEREQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type, None);
            }

            '+' => {
                let token_type = if self.match_char('=') {
                    TokenType::PLUSEQUAL
                } else {
                    TokenType::PLUS
                };
                self.add_token(token_type, None);
            }

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }

            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),

            '0'..='9' => self.number(),

            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            _ => {
                self.add_token(TokenType::UNKNOWN, None);
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.source[self.current] != '"' {
            if self.source[self.current] == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.is_at_end() {
            println!("Unterminated string at line {}", self.line);
            self.add_token(TokenType::UNKNOWN, None);
            return;
        }

        self.current += 1;

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::STRING, Some(value));
    }

    fn number(&mut self) {
        while !self.is_at_end() && self.source[self.current].is_ascii_digit() {
            self.current += 1;
        }

        if !self.is_at_end() && self.source[self.current] == '.' && self.peek().is_ascii_digit() {
            self.current += 1;

            while !self.is_at_end() && self.source[self.current].is_ascii_digit() {
                self.current += 1;
            }
        }

        let number_str: String = self.source[self.start..self.current].iter().collect();
        self.add_token(TokenType::NUMBER, Some(number_str));
    }

    fn identifier(&mut self) {
        while !self.is_at_end()
            && (self.source[self.current].is_ascii_alphanumeric() || self.source[self.current] == '_')
        {
            self.current += 1;
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        
        // Check for println! specifically here
        let token_type = if text == "println" && self.match_char('!') {
            TokenType::PRINT // Match println!
        } else {
            match self.keyword_type(&text) {
                Some(kw) => kw,
                None => TokenType::IDENTIFIER,
            }
        };
        
        self.add_token(token_type, None);
    }

    fn keyword_type(&self, text: &str) -> Option<TokenType> {
        match text {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "fun" => Some(TokenType::FUN),
            "for" => Some(TokenType::FOR),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            "pub" => Some(TokenType::PUB),
            "fn" => Some(TokenType::FN),
            "mod" => Some(TokenType::MOD),
            "let" => Some(TokenType::LET),
            "mut" => Some(TokenType::MUT),
            _ => None,
        }
    }
}

pub fn scanner_main(source: &str) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    for token in scanner.tokens {
        print_token(&token);
    }
}

