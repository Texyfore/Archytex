use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, vec3, ElementWise, InnerSpace, Vector3};
use pin_vec::PinVec;
use renderer::{data::solid, scene::SolidObject, Renderer};

pub use formats::ascn::PointID;

macro_rules! points {
    [$($p:literal),* $(,)?] => {[
        $(PointID::new($p).unwrap()),*
    ]};
}

macro_rules! face {
    ($t:literal: $p0:literal $p1:literal $p2:literal $p3:literal) => {
        Face {
            texture_id: TextureID($t),
            points: points![$p0, $p1, $p2, $p3],
            selected: false,
        }
    };
}

macro_rules! point {
    ($o:ident $e:ident [$x:literal $y:literal $z:literal]) => {
        Point {
            position: $o + $e.mul_element_wise(vec3($x as f32, $y as f32, $z as f32)),
            selected: false,
        }
    };
}

#[derive(Default)]
pub struct Scene {
    solids: PinVec<Solid>,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl Scene {
    pub fn act(&mut self, action: Action) {
        let inverse = self.execute_action(action);
        self.undo_stack.push(inverse);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            let inverse = self.execute_action(action);
            self.redo_stack.push(inverse);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            let inverse = self.execute_action(action);
            self.undo_stack.push(inverse);
        }
    }

    pub fn gen_meshes(&self, renderer: &Renderer, solids: &mut Vec<SolidObject>) {
        let old_texture_ids = solids
            .iter()
            .map(|solid_object| solid_object.texture_id)
            .collect::<Vec<_>>();

        let mut batches = HashMap::<TextureID, (Vec<solid::Vertex>, Vec<[u16; 3]>)>::new();

        for solid in &self.solids {
            for face in &solid.faces {
                let (vertices, triangles) = batches.entry(face.texture_id).or_default();
                let t0 = vertices.len() as u16;

                triangles.push([t0, t0 + 1, t0 + 2]);
                triangles.push([t0, t0 + 2, t0 + 3]);

                let points = face
                    .points
                    .map(|point_id| &solid.points[Into::<usize>::into(point_id)]);

                let normal = {
                    let edge0 = points[1].position - points[0].position;
                    let edge1 = points[3].position - points[0].position;
                    edge0.cross(edge1).normalize()
                };

                for point in points {
                    vertices.push(solid::Vertex {
                        position: point.position,
                        normal,
                        texcoord: if normal.x.abs() > normal.y.abs() {
                            if normal.x.abs() > normal.z.abs() {
                                vec2(point.position.y, point.position.z)
                            } else {
                                vec2(point.position.x, point.position.y)
                            }
                        } else if normal.y.abs() > normal.z.abs() {
                            vec2(point.position.x, point.position.z)
                        } else {
                            vec2(point.position.x, point.position.y)
                        } / 4.0,
                    });
                }
            }
        }

        for old_texture_id in old_texture_ids {
            batches.entry(old_texture_id).or_default();
        }

        *solids = batches
            .into_iter()
            .map(|(texture_id, (vertices, triangles))| SolidObject {
                texture_id,
                transform: Rc::new(renderer.create_transform()),
                mesh: Rc::new(renderer.create_mesh(&vertices, &triangles)),
            })
            .collect()
    }

    fn execute_action(&mut self, action: Action) -> Action {
        match action {
            Action::AddSolid(solid) => Action::RemoveSolid(self.solids.push(solid).into()),
            Action::RemoveSolid(index) => Action::AddSolid(self.solids.remove(index.0).unwrap()),
            Action::MoveSolid { index, delta } => {
                if let Some(solid) = self.solids.get_mut(index.0) {
                    for point in &mut solid.points {
                        point.position += delta;
                    }
                }

                Action::MoveSolid {
                    index,
                    delta: -delta,
                }
            }
        }
    }
}

pub enum Action {
    AddSolid(Solid),
    RemoveSolid(SolidID),
    MoveSolid { index: SolidID, delta: Vector3<f32> },
}

pub struct Solid {
    faces: [Face; 6],
    points: [Point; 8],
    selected: bool,
}

impl Solid {
    pub fn new(origin: Vector3<f32>, extent: Vector3<f32>) -> Self {
        Self {
            faces: [
                face!(0: 1 2 6 5),
                face!(0: 0 4 7 3),
                face!(0: 2 3 7 6),
                face!(0: 0 1 5 4),
                face!(0: 4 5 6 7),
                face!(0: 0 3 2 1),
            ],
            points: [
                point!(origin extent [0 0 0]),
                point!(origin extent [1 0 0]),
                point!(origin extent [1 1 0]),
                point!(origin extent [0 1 0]),
                point!(origin extent [0 0 1]),
                point!(origin extent [1 0 1]),
                point!(origin extent [1 1 1]),
                point!(origin extent [0 1 1]),
            ],
            selected: false,
        }
    }
}

pub struct Face {
    texture_id: TextureID,
    points: [PointID; 4],
    selected: bool,
}

pub struct Point {
    position: Vector3<f32>,
    selected: bool,
}

#[derive(Clone, Copy)]
pub struct SolidID(pub usize);

impl From<usize> for SolidID {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy)]
pub struct FaceID(u8);

impl FaceID {
    fn new(value: u8) -> Option<Self> {
        if value < 8 {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl From<FaceID> for usize {
    fn from(value: FaceID) -> Self {
        value.0 as usize
    }
}
