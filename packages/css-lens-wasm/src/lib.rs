mod ast;
mod lexer;
mod parser;
mod token;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str = r#"
export type CssClassInfo = {
    code: string;
    imports: Record<string, string>;
}

export function transformSync(code: string, extension: string): Result;
"#;

#[wasm_bindgen]
pub fn greet() {}
