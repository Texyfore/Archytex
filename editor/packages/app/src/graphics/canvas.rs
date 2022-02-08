use super::{line, Camera};

#[derive(Default)]
pub struct Canvas {
    pub(super) camera: Camera,
    pub(super) lines: Vec<line::Object>,
}

impl Canvas {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn draw_lines(&mut self, object: line::Object) {
        self.lines.push(object);
    }
}
