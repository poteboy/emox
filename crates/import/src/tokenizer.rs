#[derive(Debug, PartialEq)]
pub enum TokenType {
    Import,
    Ident,
    StringLiteral,
    Comma,
    From,
    SemiColon,
    Whitespace,
    LeftCurly,
    RightCurly,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn walk(&mut self) {
        while !self.is_at_end() {
            let c = self.source.chars().nth(self.current).unwrap();
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.add_token(TokenType::Whitespace, c.to_string());
                    self.current += 1;
                }
                ',' => {
                    self.add_token(TokenType::Comma, c.to_string());
                    self.current += 1;
                }
                ';' => {
                    self.add_token(TokenType::SemiColon, c.to_string());
                    self.current += 1;
                }
                '{' => {
                    self.add_token(TokenType::LeftCurly, c.to_string());
                    self.current += 1;
                }
                '}' => {
                    self.add_token(TokenType::RightCurly, c.to_string());
                    self.current += 1;
                }
                '"' | '\'' => {
                    self.string_literal(c);
                }
                _ => {
                    if c.is_alphabetic() {
                        self.identifier(c);
                    } else {
                        continue;
                    }
                }
            }
        }
        self.add_token(TokenType::Eof, "".to_string());
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token { token_type, lexeme });
    }

    fn string_literal(&mut self, quote: char) {
        let mut value = String::new();
        self.current += 1;
        while let Some(c) = self.source.chars().nth(self.current) {
            // e.g. "hello"
            if c == quote {
                self.current += 1;
                break;
            }
            value.push(c);
            self.current += 1;
        }
        self.add_token(TokenType::StringLiteral, value);
    }

    fn identifier(&mut self, first: char) {
        let mut value = first.to_string();
        self.current += 1;
        while let Some(c) = self.source.chars().nth(self.current) {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            value.push(c);
            self.current += 1;
        }

        let token_type = match value.as_str() {
            "import" => TokenType::Import,
            "from" => TokenType::From,
            _ => TokenType::Ident,
        };
        self.add_token(token_type, value);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "import { foo, bar } from 'baz';".to_string();
        let mut lexer = Lexer::new(source);
        lexer.walk();
        let expected = vec![
            Token {
                token_type: TokenType::Import,
                lexeme: "import".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::LeftCurly,
                lexeme: "{".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "foo".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "bar".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::RightCurly,
                lexeme: "}".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::From,
                lexeme: "from".to_string(),
            },
            Token {
                token_type: TokenType::Whitespace,
                lexeme: " ".to_string(),
            },
            Token {
                token_type: TokenType::StringLiteral,
                lexeme: "'baz'".to_string(),
            },
            Token {
                token_type: TokenType::SemiColon,
                lexeme: ";".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
            },
        ];
        assert_eq!(lexer.tokens.len(), expected.len());
        for (actual, expected) in lexer.tokens.iter().zip(expected.iter()) {
            assert_eq!(actual.token_type, expected.token_type);
        }
    }
}
