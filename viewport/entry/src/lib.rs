use logic::Viewport;
use std::sync::mpsc::{channel, Sender};
use tools::app::App;
use wasm_bindgen::prelude::*;

static mut MESSAGE_IN: Option<Sender<String>> = None;

#[wasm_bindgen]
pub fn main() {
    let (tx, rx) = channel();

    unsafe { MESSAGE_IN = Some(tx) };

    let app = App::new(rx);
    let viewport = Viewport::default();
    app.run(viewport);
}

#[wasm_bindgen(js_name = "sendMessage")]
pub fn message_received(msg: String) {
    unsafe {
        if let Some(message_in) = &MESSAGE_IN {
            message_in.send(msg).unwrap();
        }
    }
}
