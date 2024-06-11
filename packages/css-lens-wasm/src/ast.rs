use crate::token::{Token, TokenType};

/**
 * ## BNF Grammar for CSS
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
 *
 */
#[derive(Debug, Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    StyleRule(StyleRule),
    MediaQuery,
}

#[derive(Debug, Clone)]
pub struct StyleRule {}
