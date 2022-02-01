use std::{marker::PhantomData, rc::Rc};

use asset_id::GizmoID;
use cgmath::{vec3, Vector3, Zero};
use renderer::{
    data::line,
    scene::{GizmoObject, LineObject, Scene},
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
            ctx.scene_mut().act(P::deselect_action());
            ctx.set_regen();
        }

        if let Some(action) =
            P::select_action(ctx.scene().raycast(ctx.input().mouse_pos(), ctx.camera()))
        {
            ctx.scene_mut().act(action);
            ctx.set_regen();
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
        ctx.scene_mut().act(P::action());
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
    ray: Ray,
    center: Vector3<f32>,
    plane: Plane,
    elements: Vec<P::Element>,

    start: Vector3<f32>,
    delta: Vector3<i32>,
    snap: MoveSnap,

    solid_gfx: Option<Graphics>,
    line_gfx: Option<LineObject>,

    regen: bool,
    _p: PhantomData<P>,
}

impl<P: MoveProvider> Move<P> {
    pub fn new(ray: Ray, elements: Vec<P::Element>) -> Option<Self> {
        let dir = ray.direction();
        let normal = -dir;

        let center = P::center(&elements);
        let plane = Plane {
            origin: center,
            normal,
        };

        ray.intersects(&plane).map(|intersection| Self {
            ray,
            center,
            plane,
            elements,
            start: intersection.point + intersection.normal * 0.0001,
            delta: Vector3::zero(),
            snap: MoveSnap::None,
            solid_gfx: None,
            line_gfx: None,
            regen: false,
            _p: PhantomData,
        })
    }

    fn snap_to_axis(&mut self, ctx: &Context, axis: Axis) {
        self.plane = Plane {
            origin: self.center,
            normal: match axis {
                Axis::Y => Vector3::unit_x(),
                _ => Vector3::unit_y(),
            },
        };

        self.snap = MoveSnap::Axis(axis);

        if let Some(intersection) = self.ray.intersects(&self.plane) {
            self.start = intersection.point + intersection.normal * 0.0001;
        }

        self.line_gfx = Some(LineObject {
            transform: Rc::new(ctx.renderer().create_transform()),
            lines: Rc::new(ctx.renderer().create_lines(&axis.vertices(self.center))),
        });
    }

    fn snap_exclude(&mut self, ctx: &Context, axis: Axis) {
        self.plane = Plane {
            origin: self.center,
            normal: axis.to_vec3(),
        };

        self.snap = MoveSnap::Plane(axis);

        if let Some(intersection) = self.ray.intersects(&self.plane) {
            self.start = intersection.point + intersection.normal * 0.0001;
        }

        let verts = axis
            .others()
            .into_iter()
            .map(|axis| axis.vertices(self.center))
            .flatten()
            .collect::<Vec<_>>();

        self.line_gfx = Some(LineObject {
            transform: Rc::new(ctx.renderer().create_transform()),
            lines: Rc::new(ctx.renderer().create_lines(&verts)),
        });
    }
}

impl<P: MoveProvider> Tool for Move<P> {
    fn process(&mut self, ctx: &mut Context) {
        if !self.regen {
            ctx.set_regen();
            P::regen(ctx.renderer(), &self.elements, &mut self.solid_gfx);
            self.regen = true;
        }

        for (axis, key) in [
            (Axis::X, VirtualKeyCode::X),
            (Axis::Y, VirtualKeyCode::Y),
            (Axis::Z, VirtualKeyCode::Z),
        ] {
            if ctx.input().is_key_down_once(key) {
                if ctx.input().is_key_down(VirtualKeyCode::LShift) {
                    self.snap_exclude(ctx, axis);
                } else {
                    self.snap_to_axis(ctx, axis);
                }
            }
        }

        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(intersection) = ray.intersects(&self.plane) {
            let start = self.start.snap(100);
            let end = (intersection.point + intersection.normal * 0.0001).snap(100);

            let delta = self.snap.snap_vec(end - start);
            if delta != self.delta {
                let delta2 = delta - self.delta;
                P::displace(&mut self.elements, delta2);
                P::regen(ctx.renderer(), &self.elements, &mut self.solid_gfx);
                self.delta = delta;
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.scene_mut().act(P::action(&self.elements, self.delta));
            ctx.scene_mut().unhide_all();
            ctx.set_regen();
            ctx.switch_to(P::parent_tool());
            return;
        }

        if ctx.input().was_button_down_once(MouseButton::Right)
            || ctx.input().is_key_down_once(VirtualKeyCode::G)
            || ctx.input().is_key_down_once(VirtualKeyCode::Escape)
        {
            ctx.scene_mut().unhide_all();
            ctx.set_regen();
            ctx.switch_to(P::parent_tool());
        }
    }

    fn render(&self, scene: &mut Scene) {
        if let Some(graphics) = &self.solid_gfx {
            for solid_object in &graphics.solid_objects {
                scene.push_solid_object(solid_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmos.clone(),
            })
        }

        if let Some(axis_line) = &self.line_gfx {
            scene.push_line_object(axis_line.clone());
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

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn others(&self) -> [Self; 2] {
        match self {
            Self::X => [Self::Y, Self::Z],
            Self::Y => [Self::X, Self::Z],
            Self::Z => [Self::X, Self::Y],
        }
    }

    fn to_vec3(self) -> Vector3<f32> {
        match self {
            Self::X => Vector3::unit_x(),
            Self::Y => Vector3::unit_y(),
            Self::Z => Vector3::unit_z(),
        }
    }

    fn vertices(&self, center: Vector3<f32>) -> [line::Vertex; 2] {
        let (min, max) = match self {
            Self::X => (vec3(-1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)),
            Self::Y => (vec3(0.0, -1.0, 0.0), vec3(0.0, 1.0, 0.0)),
            Self::Z => (vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, 1.0)),
        };

        let color = match self {
            Self::X => [1.0, 0.0, 0.0],
            Self::Y => [0.0, 1.0, 0.0],
            Self::Z => [0.0, 0.0, 1.0],
        };

        [
            line::Vertex {
                position: center + min * 1000.0,
                color,
            },
            line::Vertex {
                position: center + max * 1000.0,
                color,
            },
        ]
    }
}

enum MoveSnap {
    None,
    Axis(Axis),
    Plane(Axis),
}

impl MoveSnap {
    fn snap_vec(&self, vec: Vector3<i32>) -> Vector3<i32> {
        match self {
            MoveSnap::None => vec,
            MoveSnap::Axis(axis) => match axis {
                Axis::X => vec3(vec.x, 0, 0),
                Axis::Y => vec3(0, vec.y, 0),
                Axis::Z => vec3(0, 0, vec.z),
            },
            MoveSnap::Plane(axis) => match axis {
                Axis::X => vec3(0, vec.y, vec.z),
                Axis::Y => vec3(vec.x, 0, vec.z),
                Axis::Z => vec3(vec.x, vec.y, 0),
            },
        }
    }
}
