mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

const SCREEN: [u8; 10] = [1,2,3,4,5,7,9,8,9,5];

#[wasm_bindgen]
pub fn screen() -> *const [u8; 10] {
    &SCREEN
}
