use crate::{
    bnf::{Rule, StyleRule, StyleSheet},
    helper::ToString,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HIR {
    pub rules: Vec<HIRRule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HIRRule {
    pub selector: String,
    pub declarations: Vec<HIRDeclaration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HIRDeclaration {
    pub property: String,
    pub value: String,
}

pub struct HIRGenerator {
    ast: StyleSheet,
}

impl HIRGenerator {
    pub fn new(ast: StyleSheet) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> HIR {
        let mut rules = vec![];
        // O(n^2), FIXME: optimize
        for rule in &self.ast.rules {
            // modify this when we have more types of rules e.g. AtRule
            let Rule::StyleRule(style_rule) = rule;
            let selector = style_rule.selector_text.clone();
            let mut declarations = vec![];
            for declaration in &style_rule.declarations {
                let property = declaration.property.clone();
                let value = declaration.value.clone().to_string();
                declarations.push(HIRDeclaration { property, value });
            }
            rules.push(HIRRule {
                selector: selector.clone(),
                declarations: declarations.clone(),
            });
        }
        HIR { rules }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_generate() {
        let source_text = r#"
            .container, div > span {
                display: flex;
                justify-content: center;
                align-items: center;
                font-size: 16px;
                width: 100%;
            }
        "#;
        let mut lexer = Lexer::new(source_text.to_string());
        lexer.build();
        let tokens = lexer.tokens();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let generator = HIRGenerator::new(ast);
        let hir = generator.generate();
        assert_eq!(hir.rules.len(), 1);

        let rule = &hir.rules[0];
        let declarations = &rule.declarations;
        assert_eq!(declarations.len(), 5);
    }
}
