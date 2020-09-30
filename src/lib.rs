mod enemy;
mod data;
mod font;
mod game;
mod game_util;
mod load;
mod objects;
mod render;
mod text;
mod types;
mod util;
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
