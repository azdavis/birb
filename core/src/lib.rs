use wasm_bindgen::prelude::*;

mod error;
mod lex;
mod parse;
mod token;

#[wasm_bindgen]
pub fn mul(a: i32, b: i32) -> i32 {
  a * b
}
