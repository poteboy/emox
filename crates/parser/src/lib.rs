extern crate lazy_static;
extern crate regex;
extern crate serde;

mod bnf;
mod helper;
pub mod hir;
pub mod lexer;
pub mod parser;
mod token;

use hir::HIRGenerator;
use lexer::Lexer;
use parser::Parser;
