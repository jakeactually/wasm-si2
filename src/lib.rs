mod utils;

//mod load;
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
extern {
    fn load(s: &str, f: &Closure<dyn FnMut(Vec<u8>)>);
}

#[wasm_bindgen]
pub fn tick() {
    alert(format!("{:?}", fetch("/data/enemies/0.dat")).as_str());
}

pub fn fetch(s: &str) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let mut ready = false;

    let f = Closure::wrap(Box::new(move |v| {
        //out = v;
        ready = true;
    }) as Box<dyn FnMut(Vec<u8>)>);

    load(s, &f);
    f.forget();

    while !ready {

    }

    out.clone()
}
