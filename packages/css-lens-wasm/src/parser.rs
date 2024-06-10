use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/**
 *
 * BNF Grammar for CSS
 * <stylesheet> ::= <rule>*
 * <rule> ::= <selectors> "{" <declarations> "}"
 *
 * ### Reference
 * - [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/)
 * - [Appendix G. Grammar of CSS 2.1](https://www.w3.org/TR/CSS21/grammar.html)
 * - [CSS.bnf](https://github.com/aptana/studio2/blob/master/tools/com.aptana.ide.parsing.tools/Parser%20Files/CSS.bnf)
 */
impl Parser {}
