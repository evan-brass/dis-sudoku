use wasm_bindgen::prelude::*;
use shared;

#[wasm_bindgen]
extern "C" {
	fn eval(js_code: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
	shared::init();

	// eval("alert(\"Does this message come through?\");");
}