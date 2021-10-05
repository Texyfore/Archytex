use logic::Viewport;
use tools::app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    run();
}

fn run() {
    let app = App::default();
    let viewport = Viewport::default();
    app.run(viewport);
}
