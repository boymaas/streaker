use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn is_mobile() -> bool;
}
