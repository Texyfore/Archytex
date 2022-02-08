use super::{line, prop, solid, Camera};

#[derive(Default)]
pub struct Canvas {
    pub(super) camera: Camera,
    pub(super) lines: Vec<line::Object>,
    pub(super) solids: Vec<solid::Object>,
    pub(super) props: Vec<prop::Object>,
}

impl Canvas {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn draw_lines(&mut self, object: line::Object) {
        self.lines.push(object);
    }

    pub fn draw_solid(&mut self, object: solid::Object) {
        self.solids.push(object);
    }

    pub fn draw_prop(&mut self, object: prop::Object) {
        self.props.push(object);
    }
}
