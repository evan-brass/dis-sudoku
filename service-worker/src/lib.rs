use wasm_bindgen::prelude::*;
use shared;

#[wasm_bindgen(start)]
pub fn run() {
	shared::init();
}