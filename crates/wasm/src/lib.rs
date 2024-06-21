extern crate emox_parser;
extern crate tsify;
extern crate wasm_bindgen;

use emox_parser::hir::HIRGenerator;
use emox_parser::lexer::Lexer;
use emox_parser::parser::Parser;
use tsify::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str = r#"
export type CssInfo = {
  rules: Rule[];
};

export type Rule = {
  selector: string;
  declarations: Declaration[];
};

export type Declaration = {
  property: string;
  value: string;
};

export function getCssInfo(code: string): CssInfo;
"#;

#[wasm_bindgen(js_name = getCssInfo, skip_typescript)]
pub fn get_css_info(code: &str) -> Result<JsValue, JsValue> {
    let mut lexer = Lexer::new(code.to_string());
    lexer.build();
    let mut tokens = lexer.tokens();
    let mut parser = Parser::new(tokens);
    let mut style_sheet = parser.parse();
    let hir = HIRGenerator::new(style_sheet).generate();

    JsValue::from_serde(&hir).map_err(|e| JsValue::from_str(&e.to_string()))
}
