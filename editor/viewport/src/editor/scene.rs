use std::collections::HashMap;

use asset_id::TextureID;
use cgmath::{vec3, ElementWise, MetricSpace, Vector3, Zero};

use crate::math::{Intersects, Plane, Ray, Sphere, Triangle};

use super::graphics::{DrawableFace, DrawablePoint, DrawableSolid};

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
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub $ty);
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
    pub fn iter_solids(&self) -> std::collections::hash_map::Values<SolidID, Solid> {
        self.solids.values()
    }

    pub fn act(&mut self, action: Action) {
        if let Some(inverse) = self.execute_action(action) {
            self.undo_stack.push(inverse);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            if let Some(inverse) = self.execute_action(action) {
                self.redo_stack.push(inverse);
            }
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            if let Some(inverse) = self.execute_action(action) {
                self.undo_stack.push(inverse);
            }
        }
    }

    pub fn raycast(&self, ray: &Ray) -> Option<RaycastHit> {
        struct HitFace {
            solid_id: SolidID,
            face_id: FaceID,
            point: Vector3<f32>,
            normal: Vector3<f32>,
        }

        struct HitPoint {
            solid_id: SolidID,
            point_id: PointID,
            point: Vector3<f32>,
        }

        let mut hit_faces = Vec::new();
        let mut hit_points = Vec::new();

        for (solid_id, solid) in &self.solids {
            for (i, face) in solid.faces.iter().enumerate() {
                let face_id = FaceID(i);

                let triangles = [
                    Triangle {
                        a: solid.points[face.points[0].0].meters(),
                        b: solid.points[face.points[1].0].meters(),
                        c: solid.points[face.points[2].0].meters(),
                    },
                    Triangle {
                        a: solid.points[face.points[0].0].meters(),
                        b: solid.points[face.points[2].0].meters(),
                        c: solid.points[face.points[3].0].meters(),
                    },
                ];

                for triangle in triangles {
                    if let Some(intersection) = ray.intersects(&triangle) {
                        hit_faces.push(HitFace {
                            solid_id: *solid_id,
                            face_id,
                            point: intersection.point,
                            normal: intersection.normal,
                        });
                        break;
                    }
                }
            }

            for (i, point) in solid.points.iter().enumerate() {
                let origin = point.meters();
                let dist = origin.distance(ray.start);

                let sphere = Sphere {
                    origin,
                    radius: dist * 0.1,
                };

                if let Some(intersection) = ray.intersects(&sphere) {
                    hit_points.push(HitPoint {
                        solid_id: *solid_id,
                        point_id: PointID(i),
                        point: intersection.point,
                    });
                }
            }
        }

        hit_faces.sort_unstable_by(|a, b| {
            let dist_a = a.point.distance2(ray.start);
            let dist_b = b.point.distance2(ray.start);
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        let endpoint = if let Some(hit_face) = hit_faces.first() {
            Some(RaycastEndpoint {
                point: hit_face.point,
                normal: hit_face.normal,
                kind: RaycastEndpointKind::Face {
                    solid_id: hit_face.solid_id,
                    face_id: hit_face.face_id,
                },
            })
        } else {
            let plane = Plane {
                origin: Vector3::zero(),
                normal: vec3(0.0, 1.0, 0.0),
            };

            ray.intersects(&plane).map(|intersection| RaycastEndpoint {
                point: intersection.point,
                normal: intersection.normal,
                kind: RaycastEndpointKind::Ground,
            })
        };

        endpoint.map(|endpoint| {
            let dist_endpoint = endpoint.point.distance2(ray.start);
            hit_points.retain(|hit_point| hit_point.point.distance2(ray.start) < dist_endpoint);

            RaycastHit {
                endpoint,
                points: hit_points
                    .into_iter()
                    .map(|hit_point| (hit_point.solid_id, hit_point.point_id))
                    .collect(),
            }
        })
    }

    fn execute_action(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::AddSolid(solid) => {
                let id = SolidID(self.next_solid_id);
                self.next_solid_id += 1;

                self.solids.insert(id, solid);
                Some(Action::RemoveSolids(vec![id]))
            }

            Action::AddSolids(solids) => {
                let mut solid_ids = Vec::new();

                for (solid_id, solid) in solids {
                    self.solids.insert(solid_id, solid);
                    solid_ids.push(solid_id);
                }

                (!solid_ids.is_empty()).then(|| Action::RemoveSolids(solid_ids))
            }

            Action::RemoveSolids(ids) => {
                let mut solids = Vec::new();
                for solid_id in ids {
                    let solid = self.solids.remove(&solid_id).unwrap();
                    solids.push((solid_id, solid));
                }

                (!solids.is_empty()).then(|| Action::AddSolids(solids))
            }

            Action::RemoveSelectedSolids => {
                let ids = self
                    .solids
                    .iter()
                    .filter(|(_, solid)| solid.selected)
                    .map(|(solid_id, _)| *solid_id)
                    .collect::<Vec<_>>();

                let mut solids = Vec::new();
                for id in ids {
                    solids.push((id, self.solids.remove(&id).unwrap()));
                }

                (!solids.is_empty()).then(|| Action::AddSolids(solids))
            }

            Action::SelectSolids(solid_ids) => {
                for solid_id in &solid_ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    solid.selected = !solid.selected;
                }

                (!solid_ids.is_empty()).then(|| Action::SelectSolids(solid_ids))
            }

            Action::DeselectSolids => {
                let mut solid_ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    if solid.selected {
                        solid.selected = false;
                        solid_ids.push(*solid_id);
                    }
                }

                (!solid_ids.is_empty()).then(|| Action::SelectSolids(solid_ids))
            }

            Action::SelectFaces(ids) => {
                for (solid_id, face_id) in &ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    let face = &mut solid.faces[face_id.0];
                    face.selected = !face.selected;
                }

                (!ids.is_empty()).then(|| Action::SelectFaces(ids))
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

                (!ids.is_empty()).then(|| Action::SelectFaces(ids))
            }

            Action::SelectPoints(ids) => {
                for (solid_id, point_id) in &ids {
                    let solid = self.solids.get_mut(solid_id).unwrap();
                    let point = &mut solid.points[point_id.0];
                    point.selected = !point.selected;
                }

                (!ids.is_empty()).then(|| Action::SelectPoints(ids))
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

                (!ids.is_empty()).then(|| Action::SelectPoints(ids))
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

                (!old_texture_ids.is_empty()).then(|| Action::AssignTextures(old_texture_ids))
            }

            Action::AssignTextures(ids) => {
                let mut old_texture_ids = Vec::new();

                for (solid_id, face_id, texture_id) in ids {
                    let solid = self.solids.get_mut(&solid_id).unwrap();
                    let face = &mut solid.faces[face_id.0];
                    old_texture_ids.push((solid_id, face_id, face.texture_id));
                    face.texture_id = texture_id;
                }

                (!old_texture_ids.is_empty()).then(|| Action::AssignTextures(old_texture_ids))
            }

            Action::MoveSelected { .. } => {
                todo!()
            }
        }
    }
}

pub enum Action {
    AddSolid(Solid),
    AddSolids(Vec<(SolidID, Solid)>),
    RemoveSolids(Vec<SolidID>),
    RemoveSelectedSolids,

    SelectSolids(Vec<SolidID>),
    DeselectSolids,

    SelectFaces(Vec<(SolidID, FaceID)>),
    DeselectFaces,

    SelectPoints(Vec<(SolidID, PointID)>),
    DeselectPoints,

    AssignTexture(TextureID),
    AssignTextures(Vec<(SolidID, FaceID, TextureID)>),

    MoveSelected { kind: MoveKind, delta: Vector3<i32> },
}

pub enum MoveKind {
    Solids,
    Faces,
    Points,
    Props,
}

#[derive(Clone)]
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

impl DrawableSolid<Face, Point> for Solid {
    fn faces(&self) -> &[Face; 6] {
        &self.faces
    }

    fn points(&self) -> &[Point; 8] {
        &self.points
    }

    fn selected(&self) -> bool {
        self.selected
    }
}

#[derive(Clone)]
pub struct Face {
    texture_id: TextureID,
    points: [PointID; 4],
    selected: bool,
}

impl DrawableFace for Face {
    fn points(&self) -> &[PointID; 4] {
        &self.points
    }

    fn texture(&self) -> TextureID {
        self.texture_id
    }

    fn selected(&self) -> bool {
        self.selected
    }
}

#[derive(Clone)]
pub struct Point {
    position: Vector3<i32>,
    selected: bool,
}

impl DrawablePoint for Point {
    fn meters(&self) -> Vector3<f32> {
        self.position.cast().unwrap() * 0.01
    }

    fn selected(&self) -> bool {
        self.selected
    }
}

#[derive(Debug)]
pub struct RaycastHit {
    pub endpoint: RaycastEndpoint,
    pub points: Vec<(SolidID, PointID)>,
}

#[derive(Debug)]
pub struct RaycastEndpoint {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub kind: RaycastEndpointKind,
}

#[derive(Debug)]
pub enum RaycastEndpointKind {
    Face { solid_id: SolidID, face_id: FaceID },
    Prop(PropID),
    Ground,
}
