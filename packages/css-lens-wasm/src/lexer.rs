use crate::token::{Token, TokenType};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CLASS_SELECTOR: Regex = Regex::new(r"^\.[a-zA-Z_][a-zA-Z0-9_-]*").unwrap();
    static ref ID_SELECTOR: Regex = Regex::new(r"^#[a-zA-Z_][a-zA-Z0-9_-]*").unwrap();
    static ref IDENT: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_-]*").unwrap();
    static ref ATKEYWORD: Regex = Regex::new(r"^@[a-zA-Z_][a-zA-Z0-9_-]*").unwrap();
    static ref STRING: Regex = Regex::new(r#"^"[^"]*""#).unwrap();
    static ref NUMBER: Regex = Regex::new(r"^[0-9]+").unwrap();
    static ref PERCENTAGE: Regex = Regex::new(r"^[0-9]+%").unwrap();
    static ref DIMENSION: Regex = Regex::new(r"^[0-9]+[a-zA-Z_][a-zA-Z0-9_-]*").unwrap();
    static ref COLON: Regex = Regex::new(r"^:").unwrap();
    static ref SEMICOLON: Regex = Regex::new(r"^;").unwrap();
    static ref CURLY_LEFT: Regex = Regex::new(r"^\{").unwrap();
    static ref CURLY_RIGHT: Regex = Regex::new(r"^\}").unwrap();
    static ref PAREN_LEFT: Regex = Regex::new(r"^\(").unwrap();
    static ref PAREN_RIGHT: Regex = Regex::new(r"^\)").unwrap();
    static ref BRACKET_LEFT: Regex = Regex::new(r"^\[").unwrap();
    static ref BRACKET_RIGHT: Regex = Regex::new(r"^\]").unwrap();
    static ref S: Regex = Regex::new(r"^[ \t\r\n\f]+").unwrap();
    static ref COMMENT: Regex = Regex::new(r"^/\*[^*]*\*+([^/*][^*]*\*+)*/").unwrap();
    static ref COMMA: Regex = Regex::new(r"^,").unwrap();
    static ref PLUS: Regex = Regex::new(r"^\+").unwrap();
    static ref GREATER: Regex = Regex::new(r"^>").unwrap();
    static ref TILDE: Regex = Regex::new(r"^~").unwrap();
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
        }
    }

    pub fn build(&mut self) {
        // mappings of css token types to regex patterns
        let patterns: Vec<(TokenType, Regex)> = vec![
            (TokenType::ClassSelector, CLASS_SELECTOR.clone()),
            (TokenType::IdSelector, ID_SELECTOR.clone()),
            (TokenType::Ident, IDENT.clone()),
            (TokenType::Atkeyword, ATKEYWORD.clone()),
            (TokenType::String, STRING.clone()),
            // Percentage & Dimension must come before Number to avoid matching the number part of the dimension
            // Maybe we should use PEG parser to skip lexing, but for now, this is fine
            (TokenType::Percentage, PERCENTAGE.clone()),
            (TokenType::Dimension, DIMENSION.clone()),
            (TokenType::Number, NUMBER.clone()),
            (TokenType::Colon, COLON.clone()),
            (TokenType::Semicolon, SEMICOLON.clone()),
            (TokenType::CurlyLeft, CURLY_LEFT.clone()),
            (TokenType::CurlyRight, CURLY_RIGHT.clone()),
            (TokenType::ParenLeft, PAREN_LEFT.clone()),
            (TokenType::ParenRight, PAREN_RIGHT.clone()),
            (TokenType::BracketLeft, BRACKET_LEFT.clone()),
            (TokenType::BracketRight, BRACKET_RIGHT.clone()),
            (TokenType::S, S.clone()),
            (TokenType::Comment, COMMENT.clone()),
            (TokenType::Comma, COMMA.clone()),
            (TokenType::Plus, PLUS.clone()),
            (TokenType::Greater, GREATER.clone()),
            (TokenType::Tilde, TILDE.clone()),
        ];

        // iterate over the source string to store tokens when matched with a regex pattern
        while self.current < self.source.len() {
            let remaining = &self.source[self.current..];
            let mut matched = false;

            for (token_type, regex) in &patterns {
                if let Some(mat) = regex.find(remaining) {
                    let lexeme = &remaining[mat.start()..mat.end()];

                    // increment line count if token type is whitespace & contains newline
                    // we do not store whitespace tokens
                    if token_type == &TokenType::S {
                        self.line += lexeme.matches('\n').count();
                        break;
                    }

                    self.tokens.push(Token::new(
                        token_type.clone(),
                        lexeme.to_string(),
                        self.line,
                    ));

                    self.current += lexeme.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                // skip to the next character if no regex pattern is matched
                self.current += 1;
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        // Arrange
        let source = r#"
            .container {
                display: flex;
                justify-content: center;
                align-items: center;
                font-size: 16px;
            }
        "#;
        let mut lexer = Lexer::new(source.to_string());
        lexer.build();
        let tokens = lexer.tokens();

        // Act: get the last token
        let last_token = tokens.last().cloned().unwrap().token_type;
        // Assert: the last token should be Eof
        assert_eq!(last_token, TokenType::Eof);

        // Act: get the first class selector token
        let class_selector = tokens
            .iter()
            .find(|t| t.token_type == TokenType::ClassSelector)
            .unwrap();
        // Assert: the first class selector token should be .container
        assert_eq!(class_selector.lexeme, ".container");

        // Act: get all ident tokens
        let idents: Vec<Token> = tokens
            .iter()
            .filter_map(|t| {
                if t.token_type == TokenType::Ident {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect();
        // Assert: there should be 7 ident tokens
        // 16px is considered as Dimension token
        assert_eq!(idents.len(), 7);
    }

    #[test]
    fn complex_css() {
        let source = r#"
            .container {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    font-size: 16px;
            }

            @media screen and (max-width: 600px) {
                .container {
                    font-size: 14px;
                }
            }
        "#
        .to_string();
        let mut lexer = Lexer::new(source.to_string());
        lexer.build();
        let tokens = lexer.tokens();
        println!("{:#?}", tokens);
    }
}
