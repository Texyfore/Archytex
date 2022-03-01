mod graphics;

use cgmath::{Vector3, Zero};

use crate::graphics::{Canvas, Graphics};

use self::graphics::ArrowGraphics;

pub struct TranslationGizmo {
    position: Vector3<f32>,
    graphics: ArrowGraphics,
    visible: bool,
}

impl TranslationGizmo {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            position: Vector3::zero(),
            graphics: ArrowGraphics::new_empty(graphics),
            visible: false,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn process(&mut self, graphics: &Graphics) {
        if !self.visible {
            return;
        }
        
        self.graphics.modify(graphics, self.position, None);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if self.visible {
            self.graphics.render(canvas);
        }
    }
}
