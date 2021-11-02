use logic::Viewport;
use tools::{app::App, console};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let app = App::default();
    let viewport = Viewport::default();
    app.run(viewport);
}

#[wasm_bindgen(js_name = "sendMessage")]
pub fn message_received(msg: &str) {
    console!("received message: `{}`", msg)
}
