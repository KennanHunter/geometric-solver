use std::convert::TryInto;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Position(i32);

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        return Position(value.try_into().unwrap());
    }
}
