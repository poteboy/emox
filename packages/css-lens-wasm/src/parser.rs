use crate::token::Token;

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
 * <declaration> ::= <ident> ":" <ident> ";"
 *
 * ### Reference
 * - [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/)
 * - [Appendix G. Grammar of CSS 2.1](https://www.w3.org/TR/CSS21/grammar.html)
 * - [CSS.bnf](https://github.com/aptana/studio2/blob/master/tools/com.aptana.ide.parsing.tools/Parser%20Files/CSS.bnf)
 */
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
}
