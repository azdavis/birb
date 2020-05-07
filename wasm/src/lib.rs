#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get(inp: &str) -> String {
  match birb_core::get(inp.as_bytes()) {
    Ok(v) => format!("{}", v),
    Err(e) => format!("error: {}", e),
  }
}
