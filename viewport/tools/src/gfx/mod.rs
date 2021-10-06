mod context;
mod mesh;

use context::Context;
use glow::*;
use std::rc::Rc;
use web_sys::HtmlCanvasElement;

pub struct Graphics {
    context: Rc<Context>,
}

impl Graphics {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let context = Context::new(canvas);

        unsafe {
            context.gl.enable(DEPTH_TEST);
            context.gl.clear_color(0.25, 0.25, 0.25, 1.0);
        }

        Self { context }
    }

    pub fn resize_viewport(&self, width: i32, height: i32) {
        unsafe {
            self.context.gl.viewport(0, 0, width, height);
        }
    }

    pub fn begin(&self) {
        unsafe {
            self.context.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }
}
