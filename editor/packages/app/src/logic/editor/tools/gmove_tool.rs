use cgmath::{Vector3, Zero};

use crate::{
    graphics::{Canvas, Graphics},
    logic::{
        editor::gizmo::TranslationGizmo,
        elements::{ElementKind, Movable},
    },
};

use super::{Context, Tool};

pub struct GizmoMoveTool<E> {
    mask: ElementKind,
    elements: Vec<(usize, E)>,
    _clone: bool,
    gizmo: TranslationGizmo,
}

impl<E> GizmoMoveTool<E>
where
    E: Movable,
{
    pub fn new(
        graphics: &Graphics,
        mask: ElementKind,
        elements: Vec<(usize, E)>,
        clone: bool,
    ) -> Self {
        let center = {
            let mut center = Vector3::zero();
            let mut n = 0.0;

            for (_, element) in &elements {
                center += element.center(mask);
                n += 1.0;
            }

            center / n
        };

        let gizmo = TranslationGizmo::new(graphics);
        gizmo.set_position(graphics, center);

        Self {
            mask,
            elements,
            _clone: clone,
            gizmo,
        }
    }
}

impl<E> Tool for GizmoMoveTool<E>
where
    E: Movable,
{
    fn process(&mut self, _ctx: Context) -> Option<Box<dyn Tool>> {
        None
    }

    fn render(&self, canvas: &mut Canvas) {
        self.gizmo.render(canvas);
        for (_, element) in &self.elements {
            element.render(canvas, self.mask);
        }
    }
}
