mod utils;

mod load;
mod types;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SCREEN: [u8; 10] = [1,2,3,4,5,7,9,8,9,5];

#[wasm_bindgen]
pub fn screen() -> *const [u8; 10] {
    &SCREEN
}

#[wasm_bindgen]
pub fn tick() {
    
    self.enemies = self.load_level(self.level)?;

    let f = Closure::wrap(Box::new(|v| {
        alert(format!("{:?}", v).as_str());
    }) as Box<dyn Fn(Vec<u8>)>);

    load("/data/enemies/0.dat", &f);

    f.forget();
}
