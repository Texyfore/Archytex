use cgmath::{InnerSpace, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::scene::{Action, MovingSolid, RaycastHit, Scene, WorkInProgress},
    math::{Intersects, Plane, Ray, Snap},
};

use super::Context;

pub trait SelectProvider {
    fn select_action(hit: RaycastHit) -> Option<Action>;
    fn deselect_action() -> Action;
    fn delete_action() -> Action;
}

#[derive(Default)]
pub struct Select;

impl Select {
    pub fn process<P: SelectProvider>(&mut self, ctx: &mut Context) -> bool {
        let mut ret = false;

        if ctx.input().was_button_down_once(MouseButton::Left) {
            if !ctx.input().is_key_down(VirtualKeyCode::LShift) {
                ctx.scene().act(P::deselect_action());
                ctx.set_regen();
            }

            let mouse_pos = ctx.input().mouse_pos();
            let ray = ctx.camera().screen_ray(mouse_pos);
            if let Some(hit) = ctx.scene().raycast(&ray) {
                if let Some(action) = P::select_action(hit) {
                    ctx.scene().act(action);
                    ctx.set_regen();
                }
            }

            ret = true;
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::Delete) {
            ctx.scene().act(P::delete_action());
            ctx.set_regen();
        }

        ret
    }
}

pub struct Move {
    plane: Plane,
    start: Vector3<f32>,
}

impl Move {
    pub fn new(scene: &mut Scene, ray: &Ray) -> Self {
        let moving = scene
            .take_selected()
            .into_iter()
            .map(|(id, solid)| MovingSolid::new(id, solid))
            .collect::<Vec<_>>();

        *scene.wip() = WorkInProgress::MoveSolids {
            delta: Vector3::zero(),
            moving,
        };

        let dir = ray.direction();

        let plane = Plane {
            origin: scene.wip().center().unwrap(),
            normal: if dir.x.abs() > dir.y.abs() {
                if dir.x.abs() > dir.z.abs() {
                    Vector3::unit_x() * dir.x.signum()
                } else {
                    Vector3::unit_z() * dir.z.signum()
                }
            } else if dir.y.abs() > dir.z.abs() {
                Vector3::unit_y() * dir.y.signum()
            } else {
                Vector3::unit_z() * dir.z.signum()
            },
        };

        let start = ray.intersects(&plane).unwrap().point;

        Self { plane, start }
    }

    pub fn process(&mut self, ctx: &mut Context) {
        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(intersection) = ray.intersects(&self.plane) {
            let start = self.start.snap(100);
            let end = intersection.point.snap(100);
            let delta = end - start;

            if delta.magnitude2() > 0 {
                ctx.scene().wip().displace(delta);
                ctx.set_regen();
            }
        }
    }

    pub fn confirm(&mut self, ctx: &mut Context) {
        ctx.scene().confirm_wip();
        ctx.set_regen();
    }

    pub fn cancel(&mut self, ctx: &mut Context) {
        ctx.scene().cancel_wip();
        ctx.set_regen();
    }
}
