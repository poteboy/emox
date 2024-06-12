use crate::lexer::Lexer;
use crate::parser::Parser;

struct ASTGenerator {
    source_text: String,
}

impl ASTGenerator {
    fn new(source_text: String) -> Self {
        Self { source_text }
    }

    fn generate(&self) -> String {
        let mut lexer = Lexer::new(self.source_text.clone());
        lexer.build();
        let tokens = lexer.tokens();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        format!("{:#?}", ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let generator = ASTGenerator::new(source_text.to_string());
        let ast = generator.generate();
        println!("{}", ast);
    }
}
