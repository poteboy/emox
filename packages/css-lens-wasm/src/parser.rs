use crate::ast::{Declaration, LiteralValue, Value};
use crate::token::{self, Token, TokenType};
use std::rc::Rc;

pub struct Parser {
    /**
     ```css
    .class {
        color: red;
    }
    ```
    `tokens` will be a vector of the following tokens:

    ```rust
    Token { token_type: ClassSelector, lexeme: ".class", line: 2 }
    Token { token_type: CurlyLeft, lexeme: "{", line: 2 }
    Token { token_type: Ident, lexeme: "color", line: 3 }
    Token { token_type: Colon, lexeme: ":", line: 3 }
    Token { token_type: Ident, lexeme: "red", line: 3 }
    Token { token_type: Semicolon, lexeme: ";", line: 3 }
    Token { token_type: CurlyRight, lexeme: "}", line: 4 }
    Token { token_type: Eof, lexeme: "", line: 5 }
    ```
    */
    tokens: Vec<Token>,
    current: usize,
}

/**
 *
 * ## Parse strategy
 *
 * The CSS grammar is a LL(1) grammar, which means that we can parse it using a recursive descent parser.
 * The reasoning behind this is that the CSS grammar is as below:
 *
 * Based on the [BNF grammar](./ast.rs), we can induce the following rules:
 * First Set:
 * - First(<stylesheet>) = First(<rule>) ∪ { EOF } = { identifier, '.', '#', '[', ':', '@media', EOF }
 * - First(<rule>) = First(<selector>) ∪ { '@media' } = { identifier, '.', '#', '[', ':', '@media' }
 * - First(<selectors>) = { identifier, '.', '#', '[', ':' }
 * - First(<combinator>) = { '+', '>', '~', ' ' }
 *
 * Follow Set:
 * - Follow(<stylesheet>) = { EOF }
 * - Follow(<rule>) = { EOF, '{' }
 * - Follow(<selectors>) = { '{', '}' }
 * - Follow(<combinator>) = { identifier, '.', '#', '[', ':' }
 *
 * Director Set:
 * - Director(selector, element) = { identifier }
 * - Director(selector, class) = { '.' }
 * - Director(selector, id) = { '#' }
 * - Director(selector, pseudo-class) = { ':' }
 * - Director(selector, pseudo-element) = { '::' }
 *
 * Director(selector, element) ∩ Director(selector, class) = ∅
 * Director(selector, element) ∩ Director(selector, id) = ∅
 * Director(selector, element) ∩ Director(selector, pseudo-class) = ∅
 * Director(selector, element) ∩ Director(selector, pseudo-element) = ∅
 * Director(selector, class) ∩ Director(selector, id) = ∅
 * Director(selector, class) ∩ Director(selector, pseudo-class) = ∅
 * Director(selector, class) ∩ Director(selector, pseudo-element) = ∅
 * Director(selector, id) ∩ Director(selector, pseudo-class) = ∅
 * Director(selector, id) ∩ Director(selector, pseudo-element) = ∅
 * Director(selector, pseudo-class) ∩ Director(selector, pseudo-element) = ∅
 *
 * Hence, CSS is a LL(1) grammar
 *
 */
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        while !self.is_end() {}
    }

    pub fn parse_rule(&mut self) {}

    pub fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        while !self.check_token_type(TokenType::CurlyRight) && !self.is_end() {
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    pub fn parse_declaration(&mut self) -> Declaration {
        let property = self
            .consume_next_token(TokenType::Ident)
            .expect("Expected property name")
            .lexeme
            .clone();
        self.consume_next_token(TokenType::Colon)
            .expect("Expected colon");
        let value = self.parse_value();
        self.consume_next_token(TokenType::Semicolon)
            .expect("Expected semicolon");

        let css_text = property.clone()
            + ": "
            + &match value.value.clone() {
                LiteralValue::Dimension(val)
                | LiteralValue::Percentage(val)
                | LiteralValue::Number(val)
                | LiteralValue::Ident(val) => val,
            }
            + ";";

        Declaration {
            property,
            value,
            css_text,
        }
    }

    pub fn parse_value(&mut self) -> Value {
        if let Some(token) = self.consume_next_token(TokenType::Ident) {
            Value {
                value: LiteralValue::Ident(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Number) {
            Value {
                value: LiteralValue::Number(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Percentage) {
            Value {
                value: LiteralValue::Percentage(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Dimension) {
            Value {
                value: LiteralValue::Dimension(token.lexeme.clone()),
            }
        } else {
            panic!("Expected value");
        }
    }

    /// Check if the current token matches any of the given token types
    pub fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check_token_type(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Advance the current token index and return the previous token
    pub fn advance(&mut self) -> Rc<Token> {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    // Consume the next token if it matches the given token type
    pub fn consume_next_token(&mut self, token_type: TokenType) -> Option<Rc<Token>> {
        if self.check_token_type(token_type) {
            return Some(self.advance());
        }
        None
    }

    /// Check if the current token is of the given type
    pub fn check_token_type(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        self.tokens[self.current].token_type == token_type
    }

    pub fn current_token(&self) -> Rc<Token> {
        Rc::new(self.tokens[self.current].clone())
    }

    pub fn previous_token(&self) -> Rc<Token> {
        Rc::new(self.tokens[self.current - 1].clone())
    }

    pub fn next_token(&self) -> Rc<Token> {
        Rc::new(self.tokens[self.current + 1].clone())
    }

    pub fn is_end(&self) -> bool {
        self.current_token().token_type == TokenType::Eof
    }
}
