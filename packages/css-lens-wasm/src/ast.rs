/**
 * ## BNF Grammar for CSS
 * <stylesheet> ::= <rule>*
 * <rule> ::= <style-rule> | <media-rule>
 * <style-rule> ::= <selectors> "{" <declarations> "}"
 * <media-rule> ::= <media-query> "{" <rule>* "}"
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
    // MediaQuery, TODO
}

#[derive(Debug, Clone)]
pub struct StyleRule {
    pub selector_text: String,
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

// #[derive(Debug, Clone)]
// pub struct MediaRule {
//     pub condition: MediaCondition,
//     pub rules: Vec<Rule>,
// }

// pub struct MediaCondition {
//     pub ident: String,
//     pub feature: Option<MediaFeature>,
// }

#[derive(Debug, Clone)]
pub struct Selector {
    pub simple_selectors: Vec<SimpleSelector>,
}

#[derive(Debug, Clone)]
pub enum SimpleSelector {
    Type(TypeSelector),
    Id(IdSelector),
    Class(ClassSelector),
    PseudoClass(PseudoClassSelector),
    PseudoElement(PseudoElementSelector),
}

#[derive(Debug, Clone)]
pub struct TypeSelector {
    pub element: String,
}

#[derive(Debug, Clone)]
pub struct IdSelector {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ClassSelector {
    pub class_name: String,
}

#[derive(Debug, Clone)]
pub struct PseudoClassSelector {
    pub ident: String,
}

#[derive(Debug, Clone)]
pub struct PseudoElementSelector {
    pub ident: String,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub css_text: String,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Ident(String),
    Number(String),     // TODO: Use f64
    Dimension(String),  // TODO: Use f64 + unit
    Percentage(String), // TODO: Use f64 + unit
}
