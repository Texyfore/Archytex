use cgmath::{Quaternion, Vector2, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Graphics},
    logic::{
        camera::Camera,
        common::{calc_angle, Axis, Snap},
        editor::gizmo::ArcGraphics,
        elements::{Movable, Prop},
        input::Input,
        ElementKind,
    },
};

use super::{CameraTool, Context, Tool};

pub struct GizmoRotate {
    props: Vec<(usize, Prop)>,
    axis: Axis,
    origin: Vector2<f32>,
    originals: Vec<Quaternion<f32>>,
    angle: i32,
    init_angle: i32,
    graphics: ArcGraphics,
}

impl GizmoRotate {
    pub fn new(
        graphics: &Graphics,
        camera: &Camera,
        input: &Input,
        props: Vec<(usize, Prop)>,
        center: Vector3<f32>,
        axis: Axis,
    ) -> Self {
        let origin = camera
            .project(center)
            .unwrap_or_else(Vector3::zero)
            .truncate();
        let originals = props.iter().map(|(_, prop)| prop.rotation()).collect();
        let init_angle = calc_angle(origin, input.mouse_pos());

        let arcs = ArcGraphics::new(graphics);
        arcs.modify(graphics, center, Some(axis), true);

        Self {
            props,
            axis,
            origin,
            originals,
            angle: 0,
            init_angle,
            graphics: arcs,
        }
    }
}

impl Tool for GizmoRotate {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let snap = if ctx.input.is_key_down(VirtualKeyCode::LControl) {
            Snap::Deg15
        } else {
            Snap::None
        };

        let delta = calc_angle(self.origin, ctx.input.mouse_pos()) - self.init_angle;
        if delta != self.angle {
            for ((_, prop), original) in self.props.iter_mut().zip(self.originals.iter()) {
                let snapped = snap.snap(delta) as f32;
                prop.set_rotation(self.axis.angle(snapped, ctx.camera.forward()) * original);
                prop.recalc(ctx.graphics);
            }
            self.angle = delta;
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            let props = self.props.drain(..).collect();

            let delta = snap.snap(delta) as f32;
            ctx.scene
                .insert_props_with_rotate(props, self.axis.angle(delta, ctx.camera.forward()));

            return Some(Box::new(CameraTool::new(ctx.graphics, false)));
        }

        None
    }

    fn render(&self, canvas: &mut Canvas) {
        self.graphics.render(canvas);
        for (_, prop) in &self.props {
            prop.render(canvas, ElementKind::Prop);
        }
    }

    fn keep_old(&self) -> bool {
        true
    }
}
