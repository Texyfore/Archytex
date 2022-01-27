use std::marker::PhantomData;

use cgmath::{Vector3, Zero};
use renderer::Renderer;
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
    elements: Vec<(P::ElementID, P::Element)>,
    graphics: Option<Graphics>,
    _p: PhantomData<P>,
}

impl<P: MoveProvider> Move<P> {
    pub fn new(ray: &Ray, elements: Vec<(P::ElementID, P::Element)>) -> Option<Self> {
        let mut center = Vector3::zero();

        for (_, element) in &elements {
            center += element.center();
        }

        let origin = center / elements.len() as f32;

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

        let plane = Plane { origin, normal };

        ray.intersects(&plane).map(|intersection| Self {
            plane,
            start: intersection.point,
            delta: Vector3::zero(),
            elements,
            graphics: None,
            _p: PhantomData,
        })
    }
}

impl<P: MoveProvider> Tool for Move<P> {
    fn process(&mut self, ctx: &mut Context) {
        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(intersection) = ray.intersects(&self.plane) {
            let start = self.start.snap(100);
            let end = intersection.point.snap(100);
            let delta = end - start;

            if delta != self.delta {
                let delta2 = delta - self.delta;
                for (_, element) in self.elements.iter_mut() {
                    element.displace(delta2);
                }

                P::regen(ctx.renderer(), &mut self.graphics);
                self.delta = delta;
            }
        }

        if ctx.input().is_button_down(MouseButton::Left) {
            ctx.switch_to(P::parent_tool())
        }

        if ctx.input().is_button_down(MouseButton::Right) {
            ctx.switch_to(P::parent_tool())
        }
    }

    fn element_mask(&self) -> ElementKind {
        P::element_mask()
    }
}

pub trait MoveProvider {
    type ElementID;
    type Element: Movable;

    fn regen(renderer: &Renderer, graphics: &mut Option<Graphics>);
    fn parent_tool() -> Box<dyn Tool>;
    fn element_mask() -> ElementKind;
}

pub trait Movable {
    fn center(&self) -> Vector3<f32>;
    fn displace(&mut self, delta: Vector3<i32>);
}
