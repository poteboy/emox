use crate::tokenizer::Lexer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportDeclaration {
    pub source: String,
    pub specifiers: Vec<ImportSpecifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ImportSpecifier {
    Default(DefaultImportSpecifier),
    Named(String),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultImportSpecifier {
    pub local: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedImportSpecifier {
    pub local: String,    // b in import { a as b }
    pub imported: String, // a in import { a as b }
}
