mod utils;

use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator when this feature is enabled
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use Console.log throughout the Rust code to aid debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Keep this in for now just to confirm the web page is still loading WASM properly
#[wasm_bindgen]
pub fn verify() {
    utils::set_panic_hook();
    log("WASM loaded");
}
