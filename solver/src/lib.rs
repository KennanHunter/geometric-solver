mod constraints;
mod parametric_value;
mod point;
mod position;
mod range;
mod utils;
mod world;

use point::PointDescriptor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct JSPoint {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl From<PointDescriptor> for JSPoint {
    fn from(value: PointDescriptor) -> Self {
        todo!()
    }
}

#[wasm_bindgen]
pub fn get_sample_points() -> Vec<JSPoint> {
    return vec![JSPoint { id: 0, x: 0, y: 0 }];
}
