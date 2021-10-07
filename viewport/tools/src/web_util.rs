use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub fn get_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    document
        .get_element_by_id("viewport")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

pub fn get_webgl_context() -> WebGl2RenderingContext {
    get_canvas()
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap()
}
