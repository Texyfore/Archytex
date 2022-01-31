use std::rc::Rc;

use renderer::{data::line, scene::Scene, Renderer};

pub struct Grid {
    step: i32,
    graphics: Option<Rc<line::Mesh>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            step: 100,
            graphics: None,
        }
    }
}

impl Grid {
    pub fn increase(&mut self) {
        self.step = (self.step * 10).clamp(1, 1000);
    }

    pub fn decrease(&mut self) {
        self.step = (self.step / 10).clamp(1, 1000);
    }

    pub fn redraw(&mut self, renderer: &Renderer) {
        self.graphics = Some(Rc::new(renderer.create_lines(&[])));
    }

    pub fn step(&self) -> i32 {
        self.step
    }

    pub fn render(&self, scene: &mut Scene) {
        if let Some(graphics) = &self.graphics {}
    }
}
