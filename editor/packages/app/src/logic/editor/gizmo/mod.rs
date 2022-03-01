mod collider;
mod graphics;

use cgmath::{Vector3, Zero};
use winit::event::MouseButton;

use crate::{
    graphics::{Canvas, Graphics},
    logic::{camera::Camera, input::Input},
};

use self::collider::{ArrowCollider, HoverCheckInfo};

use super::common::Axis;

pub use graphics::*;

pub struct TranslationGizmo {
    position: Vector3<f32>,
    graphics: ArrowGraphics,
    collider: ArrowCollider,
    visible: bool,
}

impl TranslationGizmo {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            position: Vector3::zero(),
            graphics: ArrowGraphics::new_empty(graphics),
            collider: ArrowCollider::default(),
            visible: false,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn process(&mut self, graphics: &Graphics, camera: &Camera, input: &Input) -> Option<Axis> {
        if !self.visible {
            return None;
        }

        let hover_axis = self.collider.axis_above_cursor(HoverCheckInfo {
            camera,
            mouse_position: input.mouse_pos(),
            gizmo_position: self.position,
        });

        self.graphics
            .modify(graphics, self.position, hover_axis, false);
        hover_axis.and_then(|axis| input.is_button_down_once(MouseButton::Left).then(|| axis))
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if self.visible {
            self.graphics.render(canvas);
        }
    }
}
