mod utils;

mod load;
mod objects;
mod types;
mod util;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SCREEN: [u8; 10] = [1,2,3,4,5,7,9,8,9,5];

#[wasm_bindgen]
pub fn screen() -> *const [u8; 10] {
    &SCREEN
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn tick() {
    fetch("/data/enemies/0.dat", &|v| {
        alert(format!("{:?}", v).as_str());
    });
}
