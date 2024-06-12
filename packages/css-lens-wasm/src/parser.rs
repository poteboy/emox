use crate::bnf::{
    ClassSelector, Declaration, IdSelector, LiteralValue, PseudoClassSelector,
    PseudoElementSelector, Rule, Selector, SimpleSelector, StyleRule, StyleSheet, TypeSelector,
    Value,
};
use crate::helper::ToString;
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

    pub fn parse(&mut self) -> StyleSheet {
        let mut rules = Vec::new();
        while !self.is_end() {
            rules.push(self.parse_rule());
        }
        StyleSheet { rules }
    }

    // <rule> ::= <style-rule> | <media-rule>
    pub fn parse_rule(&mut self) -> Rule {
        if self.match_token(&[TokenType::Atkeyword]) {
            // self.parse_media_rule();
            panic!("Media rule not implemented yet")
        } else {
            Rule::StyleRule(self.parse_style_rule())
        }
    }

    // <style-rule> ::= <selectors> "{" <declarations> "}"
    pub fn parse_style_rule(&mut self) -> StyleRule {
        let selectors = self.parse_selectors();
        self.consume_next_token(TokenType::CurlyLeft)
            .expect("Expected '{' after selectors");
        let declarations = self.parse_declarations();
        self.consume_next_token(TokenType::CurlyRight)
            .expect("Expected '}' after declarations");

        let selector_text = selectors
            .iter()
            .map(|selector| selector.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        StyleRule {
            selector_text,
            selectors,
            declarations,
        }
    }

    // <selectors> ::= <selector> | <selector> <combinator> <selectors>
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        selectors.push(self.parse_selector());
        while self.match_token(&[TokenType::Comma]) {
            selectors.push(self.parse_selector());
        }
        selectors
    }

    // <selector> ::= <simple-selector> | <selector> <simple-selector>
    pub fn parse_selector(&mut self) -> Selector {
        let mut simple_selectors = Vec::new();
        simple_selectors.push(self.parse_simple_selector());
        while self.match_token(&[TokenType::Greater, TokenType::Plus, TokenType::Tilde]) {
            simple_selectors.push(self.parse_simple_selector());
        }
        Selector { simple_selectors }
    }

    // <simple-selector> ::= <type-selector> | <id-selector> | <class-selector> | <pseudo-class-selector> | <pseudo-element-selector>
    pub fn parse_simple_selector(&mut self) -> SimpleSelector {
        if self.match_token(&[TokenType::Ident]) {
            // e.g. div { ... }
            SimpleSelector::Type(TypeSelector {
                element: self.previous_token().lexeme.clone(),
            })
        } else if self.match_token(&[TokenType::ClassSelector]) {
            // e.g. .class { ... }
            SimpleSelector::Class(ClassSelector {
                class_name: self.previous_token().lexeme.clone(),
            })
        } else if self.match_token(&[TokenType::IdSelector]) {
            // e.g. #id { ... }
            SimpleSelector::Id(IdSelector {
                id: self.previous_token().lexeme.clone(),
            })
        } else if self.match_token(&[TokenType::PseudoClassSelector]) {
            // e.g. :hover { ... }
            SimpleSelector::PseudoClass(PseudoClassSelector {
                ident: self.previous_token().lexeme.clone(),
            })
        } else if self.match_token(&[TokenType::PseudoElementSelector]) {
            // e.g. ::before { ... }
            SimpleSelector::PseudoElement(PseudoElementSelector {
                ident: self.previous_token().lexeme.clone(),
            })
        } else {
            panic!("Expected simple selector");
        }
    }

    // <declarations> ::= <declaration> | <declaration> <declarations>
    pub fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        while !self.check_token_type(TokenType::CurlyRight) && !self.is_end() {
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    // <declaration> ::= <property> ":" <value> ";"
    // <property> ::= <ident>
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

    // <value> ::= <ident> | <number> | <percentage> | <dimension>, more stricly: <ident> | <number> | <percentage> | <length> | <color> | <string> | <function> | <url>
    pub fn parse_value(&mut self) -> Value {
        if let Some(token) = self.consume_next_token(TokenType::Ident) {
            // e.g. { color: red; }
            Value {
                value: LiteralValue::Ident(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Number) {
            // e.g. { flex: 1; }
            Value {
                value: LiteralValue::Number(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Percentage) {
            // e.g. { width: 100%; }
            Value {
                value: LiteralValue::Percentage(token.lexeme.clone()),
            }
        } else if let Some(token) = self.consume_next_token(TokenType::Dimension) {
            // e.g. { width: 100px; }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_basic_case() {
        // .hoge { color: red; }
        let tokens = vec![
            Token {
                token_type: TokenType::ClassSelector,
                lexeme: ".hoge".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::CurlyLeft,
                lexeme: "{".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "color".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Colon,
                lexeme: ":".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Ident,
                lexeme: "red".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::CurlyRight,
                lexeme: "}".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 4,
            },
        ];
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse();
        assert_eq!(stylesheet.rules.len(), 1);
    }
}
