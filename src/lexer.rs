#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Use,
    Return,
    
    // Identifiers and literals
    Identifier,
    Type,
    Number,
    String,
    
    // Symbols
    Slash,         // /
    Equal,         // =
    Semicolon,     // ;
    LeftBracket,   // [
    RightBracket,  // ]
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    Comma,         // ,
    Colon,         // :
    Asterisk,
    Dot,
    
    EOF,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            let token = self.next_token();
            if token.token_type != TokenType::Unknown {
                tokens.push(token);
            }
        }
        
        tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: self.line,
        });
        
        tokens
    }
    
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                line: self.line,
            };
        }
        
        let c = self.advance();
        
        match c {
            '/' => Token {
                token_type: TokenType::Slash,
                lexeme: String::from("/"),
                line: self.line,
            },
            '=' => Token {
                token_type: TokenType::Equal,
                lexeme: String::from("="),
                line: self.line,
            },
            ';' => Token {
                token_type: TokenType::Semicolon,
                lexeme: String::from(";"),
                line: self.line,
            },
            '[' => Token {
                token_type: TokenType::LeftBracket,
                lexeme: String::from("["),
                line: self.line,
            },
            ']' => Token {
                token_type: TokenType::RightBracket,
                lexeme: String::from("]"),
                line: self.line,
            },
            '(' => Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                line: self.line,
            },
            ')' => Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                line: self.line,
            },
            '{' => Token {
                token_type: TokenType::LeftBrace,
                lexeme: String::from("{"),
                line: self.line,
            },
            '}' => Token {
                token_type: TokenType::RightBrace,
                lexeme: String::from("}"),
                line: self.line,
            },
            ',' => Token {
                token_type: TokenType::Comma,
                lexeme: String::from(","),
                line: self.line,
            },
            ':' => Token {
                token_type: TokenType::Colon,
                lexeme: String::from(":"),
                line: self.line,
            },
            '*' => Token {
                token_type: TokenType::Asterisk,
                lexeme: String::from("*"),
                line: self.line,
            },
            '.' => Token {
                token_type: TokenType::Dot,
                lexeme: String::from("."),
                line: self.line,
            },
            _ => {
                if self.is_alpha(c) {
                    return self.identifier_or_keyword(c);
                } else if self.is_digit(c) {
                    return self.number(c);
                } else {
                    Token {
                        token_type: TokenType::Unknown,
                        lexeme: c.to_string(),
                        line: self.line,
                    }
                }
            }
        }
    }
    
    fn identifier_or_keyword(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);
        
        while !self.is_at_end() && (self.is_alpha(self.peek()) || self.is_digit(self.peek())) {
            identifier.push(self.advance());
        }
        
        let token_type = match identifier.as_str() {
            "use" => TokenType::Use,
            "return" => TokenType::Return,
            _ => {
                if identifier.chars().next().unwrap().is_uppercase() {
                    TokenType::Type
                } else {
                    TokenType::Identifier
                }
            }
        };
        
        Token {
            token_type,
            lexeme: identifier,
            line: self.line,
        }
    }
    
    fn number(&mut self, first_digit: char) -> Token {
        let mut number = String::new();
        number.push(first_digit);
        
        while !self.is_at_end() && self.is_digit(self.peek()) {
            number.push(self.advance());
        }
        
        if !self.is_at_end() && self.peek() == '.' && self.is_digit(self.peek_next()) {
            number.push(self.advance()); // Add the '.'
            
            while !self.is_at_end() && self.is_digit(self.peek()) {
                number.push(self.advance());
            }
        }
        
        Token {
            token_type: TokenType::Number,
            lexeme: number,
            line: self.line,
        }
    }
    
    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
    
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    
    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                // Comments
                '/' if self.peek_next() == '/' => {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }
}
