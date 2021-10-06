use web_sys::HtmlCanvasElement;

pub struct Graphics;

impl Graphics {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Self
    }

    pub fn resize_viewport(&self, width: i32, height: i32) {}

    pub fn begin(&self) {}
}
