mod constraints;
mod parametric_value;
mod point;
mod position;
mod range;
mod utils;
mod world;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, solver!");
}
