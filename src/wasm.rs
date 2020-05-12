use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(_name: &str) {
    let h = Hash::parse("00000000000000000000000000000000").unwrap();
    print!("encoding: {}\n", h);
}
