use asset::Prop;

use super::{Context, Tool};

pub struct RotateTool {
    props: Vec<(usize, Prop)>,
}

impl RotateTool {
    pub fn new(props: Vec<(usize, Prop)>) -> Self {
        Self { props }
    }
}

impl Tool for RotateTool {
    fn process(&mut self, _ctx: Context) -> Option<Box<dyn Tool>> {
        None
    }

    fn render(&self, _canvas: &mut crate::graphics::Canvas) {}
}
