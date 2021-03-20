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
// pub fn window() -> Option<Window>
// pub fn document(&self) -> Option<Document>
// pub fn body(&self) -> Option<HtmlElement>
// pub fn append_child(&self, node: &Node) -> Result<Node, JsValue>
// 

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("global window unavailable");
    let document = window.document().expect("document unavailable");
    let body = document.body().expect("document body unavailable");

    // Header section
    let header = document.create_element("header")?;
    let h1 = document.create_element("h1")?;
    let title = document.create_text_node("Enterprise without JavaScript");
    h1.append_child(&title)?;
    header.append_child(&h1)?;
    body.append_child(&header)?;

    // Generate the form
    let form = document.create_element("form")?;
    body.append_child(&form)?;
    
    // Footer section
    let footer = document.create_element("footer")?;
    let footer_text = document.create_text_node("footer text goes here");
    footer.append_child(&footer_text)?;
    body.append_child(&footer)?;

    // If we made it this far all's well
    Ok(())
}