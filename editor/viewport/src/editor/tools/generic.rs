use std::marker::PhantomData;

use asset_id::GizmoID;
use cgmath::{Vector3, Zero};
use renderer::{
    scene::{GizmoObject, Scene},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::{
        elements::ElementKind,
        graphics::Graphics,
        scene::{Action, RaycastHit},
    },
    math::{Intersects, Plane, Ray, Snap},
};

use super::{Context, Tool};

#[derive(Default)]
pub struct Select<P: SelectProvider> {
    _p: PhantomData<P>,
}

impl<P: SelectProvider> Tool for Select<P> {
    fn process(&mut self, ctx: &mut Context) {
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

        ctx.switch_to(P::parent_tool());
    }

    fn element_mask(&self) -> ElementKind {
        P::element_mask()
    }
}

pub trait SelectProvider {
    fn deselect_action() -> Action;
    fn select_action(hit: RaycastHit) -> Option<Action>;
    fn parent_tool() -> Box<dyn Tool>;
    fn element_mask() -> ElementKind;
}

#[derive(Default)]
pub struct Delete<P: DeleteProvider> {
    _p: PhantomData<P>,
}

impl<P: DeleteProvider> Tool for Delete<P> {
    fn process(&mut self, ctx: &mut Context) {
        ctx.scene().act(P::action());
        ctx.set_regen();
        ctx.switch_to(P::parent_tool());
    }

    fn element_mask(&self) -> ElementKind {
        P::element_mask()
    }
}

pub trait DeleteProvider {
    fn action() -> Action;
    fn parent_tool() -> Box<dyn Tool>;
    fn element_mask() -> ElementKind;
}

pub struct Move<P: MoveProvider> {
    plane: Plane,
    start: Vector3<f32>,
    delta: Vector3<i32>,
    elements: Vec<P::Element>,
    graphics: Option<Graphics>,
    regen: bool,
    _p: PhantomData<P>,
}

impl<P: MoveProvider> Move<P> {
    pub fn new(ray: &Ray, elements: Vec<P::Element>) -> Option<Self> {
        let dir = ray.direction();
        let normal = if dir.x.abs() > dir.y.abs() {
            if dir.x.abs() > dir.z.abs() {
                Vector3::unit_x() * dir.x.signum()
            } else {
                Vector3::unit_z() * dir.z.signum()
            }
        } else if dir.y.abs() > dir.z.abs() {
            Vector3::unit_y() * dir.y.signum()
        } else {
            Vector3::unit_z() * dir.z.signum()
        };

        let plane = Plane {
            origin: P::center(&elements),
            normal,
        };

        ray.intersects(&plane).map(|intersection| Self {
            plane,
            start: intersection.point + intersection.normal * 0.0001,
            delta: Vector3::zero(),
            elements,
            graphics: None,
            regen: false,
            _p: PhantomData,
        })
    }
}

impl<P: MoveProvider> Tool for Move<P> {
    fn process(&mut self, ctx: &mut Context) {
        if !self.regen {
            ctx.set_regen();
            P::regen(ctx.renderer(), &self.elements, &mut self.graphics);
            self.regen = true;
        }

        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(intersection) = ray.intersects(&self.plane) {
            let start = self.start.snap(100);
            let end = (intersection.point + intersection.normal * 0.0001).snap(100);
            let delta = end - start;

            if delta != self.delta {
                let delta2 = delta - self.delta;
                P::displace(&mut self.elements, delta2);
                P::regen(ctx.renderer(), &self.elements, &mut self.graphics);
                self.delta = delta;
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.scene().act(P::action(&self.elements, self.delta));
            ctx.scene().unhide_all();
            ctx.set_regen();
            ctx.switch_to(P::parent_tool());
            return;
        }

        if ctx.input().was_button_down_once(MouseButton::Right)
            || ctx.input().is_key_down_once(VirtualKeyCode::G)
            || ctx.input().is_key_down_once(VirtualKeyCode::Escape)
        {
            ctx.scene().unhide_all();
            ctx.set_regen();
            ctx.switch_to(P::parent_tool());
        }
    }

    fn render(&self, scene: &mut Scene) {
        if let Some(graphics) = &self.graphics {
            for solid_object in &graphics.solid_objects {
                scene.push_solid_object(solid_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmos.clone(),
            })
        }
    }

    fn element_mask(&self) -> ElementKind {
        P::element_kind()
    }
}

pub trait MoveProvider {
    type Element;

    fn center(elements: &[Self::Element]) -> Vector3<f32>;
    fn displace(elements: &mut [Self::Element], delta: Vector3<i32>);
    fn action(elements: &[Self::Element], delta: Vector3<i32>) -> Action;
    fn parent_tool() -> Box<dyn Tool>;
    fn element_kind() -> ElementKind;

    fn regen(renderer: &Renderer, elements: &[Self::Element], graphics: &mut Option<Graphics>);
}
