mod collider;
mod graphics;

use cgmath::{Vector3, Zero};
use winit::event::MouseButton;

use crate::{
    graphics::{structures::LineVertex, Canvas, Graphics},
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

    pub fn process(
        &mut self,
        graphics: &Graphics,
        camera: &Camera,
        input: &Input,
    ) -> Option<Selection> {
        if !self.visible {
            return None;
        }

        let selection = self.collider.hover_check(HoverCheckInfo {
            camera,
            mouse_position: input.mouse_pos(),
            gizmo_position: self.position,
        });

        self.graphics
            .modify(graphics, self.position, selection, false);

        selection.and_then(|axis| input.is_button_down_once(MouseButton::Left).then(|| axis))
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if self.visible {
            self.graphics.render(canvas);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Selection {
    Axis(Axis),
    Plane(Axis),
}

impl Selection {
    pub fn line_vertices(&self, center: Vector3<f32>) -> Vec<LineVertex> {
        match self {
            Self::Axis(axis) => axis.line_vertices(center).into(),
            Self::Plane(axis) => axis
                .others()
                .into_iter()
                .map(|axis| axis.line_vertices(center).into_iter())
                .flatten()
                .collect(),
        }
    }

    pub fn axis(&self) -> &Axis {
        match self {
            Selection::Axis(axis) => axis,
            Selection::Plane(axis) => axis,
        }
    }

    pub fn is_axis(&self) -> bool {
        matches!(self, Self::Axis(_))
    }
}
