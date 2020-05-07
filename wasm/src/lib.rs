/// Foo.
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get(inp: &str) -> String {
  let mut inp = inp.to_owned();
  inp.make_ascii_uppercase();
  inp.push_str(" fella dude");
  inp
}
