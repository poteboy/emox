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
    Token { token_type: ClassSelector, lexeme: ".class", line: 1 }
    Token { token_type: S, lexeme: " ", line: 1 }
    Token { token_type: CurlyLeft, lexeme: "{", line: 1 }
    Token { token_type: S, lexeme: "\n            ", line: 1 }
    Token { token_type: Ident, lexeme: "color", line: 2 }
    Token { token_type: Colon, lexeme: ":", line: 2 }
    Token { token_type: S, lexeme: " ", line: 2 }
    Token { token_type: Ident, lexeme: "red", line: 2 }
    Token { token_type: Semicolon, lexeme: ";", line: 2 }
    Token { token_type: S, lexeme: "\n        ", line: 2 }
    Token { token_type: CurlyRight, lexeme: "}", line: 3 }
    Token { token_type: Eof, lexeme: "", line: 3 }
    ```
    */
    tokens: Vec<Token>,
    current: usize,
}

/**
 *
 * BNF Grammar for CSS
 * <stylesheet> ::= <rule>*
 * <rule> ::= <selectors> "{" <declarations> "}" | <media-query> "{" <rule>* "}"
 * <selectors> ::= <selector> | <selector> <combinator> <selectors>
 * <combinator> ::= "+" | ">" | "~" | " "
 * <selector> ::= <simple-selector> | <selector> <simple-selector>
 * <simple-selector> ::= <type-selector> | <id-selector> | <class-selector> | <pseudo-class-selector> | <pseudo-element-selector>
 * <type-selector> ::= <ident>
 * <id-selector> ::= "#" <ident>
 * <class-selector> ::= "." <ident>
 * <pseudo-class-selector> ::= ":" <ident>
 * <pseudo-element-selector> ::= "::" <ident>
 *
 * <declarations> ::= <declaration> | <declaration> <declarations>
 * <declaration> ::= <property> ":" <value> ";"
 * <property> ::= <ident>
 * <value> ::= <ident> | <number> | <percentage> | <length> | <color> | <string> | <function> | <url>
 * <function> ::= <ident> "(" <value>* ")"
 * <url> ::= "url(" <string> ")"
 *
 * <media-query> ::= "@media" <media-condition>
 * <media-condition> ::= ::= <ident> | <ident> "(" <media-feature> ")"
 * <media-feature> ::= <ident> ":" <value>
 *
 * ### Reference
 * - [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/)
 * - [Appendix G. Grammar of CSS 2.1](https://www.w3.org/TR/CSS21/grammar.html)
 * - [CSS.bnf](https://github.com/aptana/studio2/blob/master/tools/com.aptana.ide.parsing.tools/Parser%20Files/CSS.bnf)
 */
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        while !self.is_end() {}
    }

    pub fn parse_rule(&mut self) {}

    /// Advance the current token index and return the previous token
    pub fn advance(&mut self) -> Rc<Token> {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous_token()
    }

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

    pub fn is_end(&self) -> bool {
        self.current_token().token_type == TokenType::Eof
    }
}
