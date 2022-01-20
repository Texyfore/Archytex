use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, vec3, ElementWise, InnerSpace, Vector3};
use renderer::{
    data::{line, solid},
    scene::{LineObject, SolidObject},
    Renderer,
};

macro_rules! points {
    [$($p:literal),* $(,)?] => {[
        $(PointID($p)),*
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

macro_rules! entity_id {
    ($name:ident, $ty:ty) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name($ty);
    };
}

entity_id!(SolidID, u32);
entity_id!(FaceID, usize);
entity_id!(PointID, usize);
entity_id!(PropID, u32);

#[derive(Default)]
pub struct Scene {
    solids: HashMap<SolidID, Solid>,
    next_solid_id: u32,
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

    pub fn get_all_solids(&self) -> Vec<SolidID> {
        self.solids.keys().copied().collect()
    }

    pub fn raycast(&self) -> Option<RaycastHit> {
        if let Some(solid_id) = self.solids.keys().copied().next() {
            return Some(RaycastHit::Solid {
                solid_id,
                face_id: FaceID(0),
                point_id: None,
            });
        }
        None
    }

    pub fn gen_meshes(
        &self,
        renderer: &Renderer,
        solids: &mut Vec<SolidObject>,
        lines: &mut Option<LineObject>,
    ) {
        let transform = Rc::new(renderer.create_transform());

        let old_texture_ids = solids
            .iter()
            .map(|solid_object| solid_object.texture_id)
            .collect::<Vec<_>>();

        let mut batches = HashMap::<TextureID, (Vec<solid::Vertex>, Vec<[u16; 3]>)>::new();

        for solid in self.solids.values() {
            for face in &solid.faces {
                let (vertices, triangles) = batches.entry(face.texture_id).or_default();
                let t0 = vertices.len() as u16;

                triangles.push([t0, t0 + 1, t0 + 2]);
                triangles.push([t0, t0 + 2, t0 + 3]);

                let points = face.points.map(|point_id| &solid.points[point_id.0]);

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
                transform: transform.clone(),
                mesh: Rc::new(renderer.create_mesh(&vertices, &triangles)),
            })
            .collect();

        let mut vertices = Vec::new();

        let mut add_line = |solid: &Solid, a: usize, b: usize| {
            vertices.push(line::Vertex {
                position: solid.points[a].position,
                color: [0.0; 3],
            });
            vertices.push(line::Vertex {
                position: solid.points[b].position,
                color: [0.0; 3],
            });
        };

        for solid in self.solids.values() {
            for face in [0, 1] {
                let disp = face * 4;
                add_line(solid, disp, disp + 1);
                add_line(solid, disp + 1, disp + 2);
                add_line(solid, disp + 2, disp + 3);
                add_line(solid, disp + 3, disp);
            }

            for segment in 0..4 {
                add_line(solid, segment, segment + 4);
            }
        }

        *lines = Some(LineObject {
            transform,
            lines: Rc::new(renderer.create_lines(&vertices)),
        });
    }

    fn execute_action(&mut self, action: Action) -> Action {
        match action {
            Action::AddSolid(id, solid) => {
                let id = id.unwrap_or_else(|| {
                    let id = SolidID(self.next_solid_id);
                    self.next_solid_id += 1;
                    id
                });

                self.solids.insert(id, solid);
                Action::RemoveSolids(vec![id])
            }

            Action::AddSolids(solids) => {
                let mut solid_ids = Vec::new();

                for (solid_id, solid) in solids {
                    self.solids.insert(solid_id, solid);
                    solid_ids.push(solid_id);
                }

                Action::RemoveSolids(solid_ids)
            }

            Action::RemoveSolids(ids) => {
                let mut solids = Vec::new();
                for solid_id in ids {
                    let solid = self.solids.remove(&solid_id).unwrap();
                    solids.push((solid_id, solid));
                }

                Action::AddSolids(solids)
            }

            Action::SelectSolids(solid_ids) => {
                for solid_id in &solid_ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    solid.selected = !solid.selected;
                }

                Action::SelectSolids(solid_ids)
            }

            Action::DeselectSolids => {
                let mut solid_ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    if solid.selected {
                        solid.selected = false;
                        solid_ids.push(*solid_id);
                    }
                }

                Action::SelectSolids(solid_ids)
            }

            Action::MoveSolids(delta) => {
                for solid in self.solids.values_mut().filter(|solid| solid.selected) {
                    for point in &mut solid.points {
                        point.position += delta;
                    }
                }

                Action::MoveSolids(-delta)
            }
        }
    }
}

pub enum Action {
    AddSolid(Option<SolidID>, Solid),
    AddSolids(Vec<(SolidID, Solid)>),
    RemoveSolids(Vec<SolidID>),

    SelectSolids(Vec<SolidID>),
    DeselectSolids,

    MoveSolids(Vector3<f32>),
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

pub enum RaycastHit {
    Solid {
        solid_id: SolidID,
        face_id: FaceID,
        point_id: Option<PointID>,
    },
    Prop(PropID),
}
