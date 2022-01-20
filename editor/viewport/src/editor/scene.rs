use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, vec3, ElementWise, InnerSpace, Matrix4, Vector3};
use renderer::{
    data::{gizmo, line, solid},
    scene::{LineObject, SolidObject},
    Renderer,
};

use super::Graphics;

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
            position: $o + $e.mul_element_wise(vec3($x, $y, $z)),
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

    pub fn get_all_faces(&self) -> Vec<(SolidID, FaceID)> {
        self.solids
            .keys()
            .map(|solid_id| (0..6).map(|i| (*solid_id, FaceID(i))))
            .flatten()
            .collect()
    }

    pub fn get_all_points(&self) -> Vec<(SolidID, PointID)> {
        self.solids
            .keys()
            .map(|solid_id| (0..8).map(|i| (*solid_id, PointID(i))))
            .flatten()
            .collect()
    }

    pub fn raycast(&self) -> Option<RaycastHit> {
        if let Some(solid_id) = self.solids.keys().copied().next() {
            return Some(RaycastHit::Solid {
                solid_id,
                face_id: FaceID(0),
                point_id: Some(PointID(0)),
            });
        }
        None
    }

    pub(super) fn gen_meshes(&self, renderer: &Renderer, graphics: &mut Option<Graphics>) {
        let transform = Rc::new(renderer.create_transform());

        let old_texture_ids = graphics.as_ref().map(|graphics| {
            graphics
                .solid_objects
                .iter()
                .map(|solid_object| solid_object.texture_id)
                .collect::<Vec<_>>()
        });

        let mut batches = HashMap::<TextureID, (Vec<solid::Vertex>, Vec<[u16; 3]>)>::new();

        for solid in self.solids.values() {
            for face in &solid.faces {
                let (vertices, triangles) = batches.entry(face.texture_id).or_default();
                let t0 = vertices.len() as u16;

                triangles.push([t0, t0 + 1, t0 + 2]);
                triangles.push([t0, t0 + 2, t0 + 3]);

                let points = face.points.map(|point_id| &solid.points[point_id.0]);

                let normal = {
                    let edge0 = points[1].meters() - points[0].meters();
                    let edge1 = points[3].meters() - points[0].meters();
                    edge0.cross(edge1).normalize()
                };

                for point in points {
                    let position = point.meters();

                    vertices.push(solid::Vertex {
                        position: point.meters(),
                        normal,
                        texcoord: if normal.x.abs() > normal.y.abs() {
                            if normal.x.abs() > normal.z.abs() {
                                vec2(position.y, position.z)
                            } else {
                                vec2(position.x, position.y)
                            }
                        } else if normal.y.abs() > normal.z.abs() {
                            vec2(position.x, position.z)
                        } else {
                            vec2(position.x, position.y)
                        } / 4.0,
                        tint: if face.selected {
                            [1.0, 1.0, 0.5, 0.2]
                        } else {
                            [0.0; 4]
                        },
                    });
                }
            }
        }

        if let Some(old_texture_ids) = old_texture_ids {
            for old_texture_id in old_texture_ids {
                batches.entry(old_texture_id).or_default();
            }
        }

        let mut lines = Vec::new();

        let mut add_line = |solid: &Solid, a: usize, b: usize| {
            lines.push(line::Vertex {
                position: solid.points[a].meters(),
                color: [0.0; 3],
            });
            lines.push(line::Vertex {
                position: solid.points[b].meters(),
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

        *graphics = Some(Graphics {
            solid_objects: batches
                .into_iter()
                .map(|(texture_id, (vertices, triangles))| SolidObject {
                    texture_id,
                    transform: transform.clone(),
                    mesh: Rc::new(renderer.create_mesh(&vertices, &triangles)),
                })
                .collect(),
            line_object: LineObject {
                transform,
                lines: Rc::new(renderer.create_lines(&lines)),
            },
            point_gizmo_instances: Rc::new(
                renderer.create_gizmo_instances(
                    &self
                        .solids
                        .values()
                        .map(|solid| solid.points.iter())
                        .flatten()
                        .map(|point| gizmo::Instance {
                            matrix: Matrix4::from_translation(point.meters()).into(),
                            color: [0.0; 4],
                        })
                        .collect::<Vec<_>>(),
                ),
            ),
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

            Action::SelectFaces(ids) => {
                for (solid_id, face_id) in &ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    let face = &mut solid.faces[face_id.0];
                    face.selected = !face.selected;
                }

                Action::SelectFaces(ids)
            }

            Action::DeselectFaces => {
                let mut ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    for (face_id, face) in solid.faces.iter_mut().enumerate() {
                        if face.selected {
                            face.selected = false;
                            ids.push((*solid_id, FaceID(face_id)));
                        }
                    }
                }

                Action::SelectFaces(ids)
            }

            Action::SelectPoints(ids) => {
                for (solid_id, point_id) in &ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    let point = &mut solid.points[point_id.0];
                    point.selected = !point.selected;
                }

                Action::SelectPoints(ids)
            }

            Action::DeselectPoints => {
                let mut ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    for (point_id, point) in solid.points.iter_mut().enumerate() {
                        if point.selected {
                            point.selected = false;
                            ids.push((*solid_id, PointID(point_id)));
                        }
                    }
                }

                Action::SelectPoints(ids)
            }

            Action::MoveSolids(delta) => {
                for solid in self.solids.values_mut().filter(|solid| solid.selected) {
                    for point in &mut solid.points {
                        point.position += delta;
                    }
                }

                Action::MoveSolids(-delta)
            }

            Action::MoveFaces(delta) => {
                for solid in self.solids.values_mut() {
                    for face in &solid.faces {
                        if face.selected {
                            for point in face.points {
                                let point = &mut solid.points[point.0];
                                point.position += delta;
                            }
                        }
                    }
                }

                Action::MoveFaces(-delta)
            }

            Action::MovePoints(delta) => {
                for solid in self.solids.values_mut() {
                    for point in &mut solid.points {
                        if point.selected {
                            point.position += delta;
                        }
                    }
                }

                Action::MovePoints(-delta)
            }

            Action::AssignTexture(texture_id) => {
                let mut old_texture_ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    for (face_id, face) in solid.faces.iter_mut().enumerate() {
                        if face.selected {
                            old_texture_ids.push((*solid_id, FaceID(face_id), face.texture_id));
                            face.texture_id = texture_id;
                        }
                    }
                }

                Action::AssignTextures(old_texture_ids)
            }

            Action::AssignTextures(ids) => {
                let mut old_texture_ids = Vec::new();

                for (solid_id, face_id, texture_id) in ids {
                    let solid = self.solids.get_mut(&solid_id).unwrap();
                    let face = &mut solid.faces[face_id.0];
                    old_texture_ids.push((solid_id, face_id, face.texture_id));
                    face.texture_id = texture_id;
                }

                Action::AssignTextures(old_texture_ids)
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

    SelectFaces(Vec<(SolidID, FaceID)>),
    DeselectFaces,

    SelectPoints(Vec<(SolidID, PointID)>),
    DeselectPoints,

    MoveSolids(Vector3<i32>),
    MoveFaces(Vector3<i32>),
    MovePoints(Vector3<i32>),

    AssignTexture(TextureID),
    AssignTextures(Vec<(SolidID, FaceID, TextureID)>),
}

pub struct Solid {
    faces: [Face; 6],
    points: [Point; 8],
    selected: bool,
}

impl Solid {
    pub fn new(origin: Vector3<i32>, extent: Vector3<i32>) -> Self {
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
    position: Vector3<i32>,
    selected: bool,
}

impl Point {
    fn meters(&self) -> Vector3<f32> {
        self.position.cast().unwrap() * 0.01
    }
}

pub enum RaycastHit {
    Solid {
        solid_id: SolidID,
        face_id: FaceID,
        point_id: Option<PointID>,
    },
    Prop(PropID),
}
