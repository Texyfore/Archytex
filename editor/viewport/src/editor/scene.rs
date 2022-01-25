use std::{collections::HashMap, rc::Rc};

use asset_id::TextureID;
use cgmath::{vec2, vec3, ElementWise, InnerSpace, Matrix4, MetricSpace, Vector3, Zero};
use renderer::{
    data::{gizmo, line, solid},
    scene::{LineObject, SolidObject},
    Renderer,
};

use crate::math::{Intersects, Plane, Ray, Sphere, Triangle};

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
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name($ty);
    };
}

entity_id!(SolidID, u32);
entity_id!(FaceID, usize);
entity_id!(PointID, usize);
entity_id!(PropID, u32);

#[derive(Default)]
pub(super) struct Scene {
    solids: HashMap<SolidID, Solid>,
    next_solid_id: u32,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
    wip: WorkInProgress,
}

impl Scene {
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

    pub fn take_selected(&mut self) -> Vec<(SolidID, Solid)> {
        let mut vec = Vec::new();
        let ids = self
            .solids
            .iter()
            .filter(|(_, solid)| solid.selected)
            .map(|(solid_id, _)| *solid_id)
            .collect::<Vec<_>>();

        for id in ids {
            vec.push((id, self.solids.remove(&id).unwrap()));
        }

        vec
    }

    pub fn clone_selected(&mut self, ids: &[SolidID]) -> Vec<(SolidID, Solid)> {
        let mut vec = Vec::new();
        let ids = self
            .solids
            .iter()
            .filter(|(_, solid)| solid.selected)
            .map(|(solid_id, _)| *solid_id)
            .collect::<Vec<_>>();

        for id in ids {
            vec.push((id, self.solids.get(&id).unwrap().clone()));
        }

        vec
    }

    pub fn wip(&mut self) -> &mut WorkInProgress {
        &mut self.wip
    }

    pub fn confirm_wip(&mut self) {
        match self.wip.take() {
            WorkInProgress::None => {}
            WorkInProgress::NewSolid(solid) => {
                self.act(Action::AddSolid(solid));
            }
            WorkInProgress::MoveSolids(moving) => self.act(Action::AddSolids(
                moving
                    .into_iter()
                    .map(|moving| (moving.id, moving.solid))
                    .collect(),
            )),
        }
    }

    pub fn cancel_wip(&mut self) {
        if let WorkInProgress::MoveSolids(moving) = self.wip.take() {
            for mut moving in moving {
                for i in 0..8 {
                    moving.solid.points[i].position = moving.original_positions[i];
                }
                self.solids.insert(moving.id, moving.solid);
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

    pub fn gen_graphics(
        &self,
        renderer: &Renderer,
        graphics: &mut Option<Graphics>,
        mask: GraphicsMask,
    ) {
        let transform = Rc::new(renderer.create_transform());

        let old_texture_ids = graphics.as_ref().map(|graphics| {
            graphics
                .solid_objects
                .iter()
                .map(|solid_object| solid_object.texture_id)
                .collect::<Vec<_>>()
        });

        let mut batches = HashMap::<TextureID, (Vec<solid::Vertex>, Vec<[u16; 3]>)>::new();

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

        let mut gen_solid = |solid: &Solid| {
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

                    let has_tint = match mask {
                        GraphicsMask::Solids => solid.selected,
                        GraphicsMask::Faces => face.selected,
                        GraphicsMask::Points => false,
                    };

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
                        tint: if has_tint {
                            [0.04, 0.36, 0.85, 0.5]
                        } else {
                            [0.0; 4]
                        },
                    });
                }
            }

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
        };

        for solid in self.solids.values() {
            gen_solid(solid);
        }

        match &self.wip {
            WorkInProgress::None => {}
            WorkInProgress::NewSolid(solid) => {
                gen_solid(solid);
            }
            WorkInProgress::MoveSolids(solids) => {
                for moving_solid in solids {
                    gen_solid(&moving_solid.solid);
                }
            }
        }

        if let Some(old_texture_ids) = old_texture_ids {
            for old_texture_id in old_texture_ids {
                batches.entry(old_texture_id).or_default();
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
            point_gizmo_instances: Rc::new(renderer.create_gizmo_instances(
                &if mask.show_points() {
                    self.solids
                        .values()
                        .map(|solid| solid.points.iter())
                        .flatten()
                        .map(|point| gizmo::Instance {
                            matrix: Matrix4::from_translation(point.meters()).into(),
                            color: if point.selected {
                                [0.04, 0.36, 0.85, 0.0]
                            } else {
                                [0.0; 4]
                            },
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                },
            )),
        });
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

            Action::MoveSolids(delta) => {
                for solid in self.solids.values_mut().filter(|solid| solid.selected) {
                    for point in &mut solid.points {
                        point.position += delta;
                    }
                }

                (delta.magnitude2() != 0).then(|| Action::MoveSolids(-delta))
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

                (delta.magnitude2() != 0).then(|| Action::MoveFaces(-delta))
            }

            Action::MovePoints(delta) => {
                for solid in self.solids.values_mut() {
                    for point in &mut solid.points {
                        if point.selected {
                            point.position += delta;
                        }
                    }
                }

                (delta.magnitude2() != 0).then(|| Action::MovePoints(-delta))
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

    MoveSolids(Vector3<i32>),
    MoveFaces(Vector3<i32>),
    MovePoints(Vector3<i32>),

    AssignTexture(TextureID),
    AssignTextures(Vec<(SolidID, FaceID, TextureID)>),
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

    fn set_min_max(&mut self, min: Vector3<i32>, max: Vector3<i32>) -> bool {
        let mut changed = false;

        let mut change = |index: usize, value: Vector3<i32>| {
            if self.points[index].position != value {
                self.points[index].position = value;
                changed = true;
            }
        };

        change(0, vec3(min.x, min.y, min.z));
        change(1, vec3(max.x, min.y, min.z));
        change(2, vec3(max.x, max.y, min.z));
        change(3, vec3(min.x, max.y, min.z));
        change(4, vec3(min.x, min.y, max.z));
        change(5, vec3(max.x, min.y, max.z));
        change(6, vec3(max.x, max.y, max.z));
        change(7, vec3(min.x, max.y, max.z));

        changed
    }
}

#[derive(Clone)]
pub struct Face {
    texture_id: TextureID,
    points: [PointID; 4],
    selected: bool,
}

#[derive(Clone)]
pub struct Point {
    position: Vector3<i32>,
    selected: bool,
}

impl Point {
    fn meters(&self) -> Vector3<f32> {
        self.position.cast().unwrap() * 0.01
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

pub enum GraphicsMask {
    Solids,
    Faces,
    Points,
}

impl GraphicsMask {
    fn show_points(&self) -> bool {
        matches!(self, Self::Points)
    }
}

pub enum WorkInProgress {
    None,
    NewSolid(Solid),
    MoveSolids(Vec<MovingSolid>),
}

impl Default for WorkInProgress {
    fn default() -> Self {
        Self::None
    }
}

impl WorkInProgress {
    pub fn set_min_max(&mut self, min: Vector3<i32>, max: Vector3<i32>) -> bool {
        if let Self::NewSolid(solid) = self {
            solid.set_min_max(min, max)
        } else {
            false
        }
    }

    pub fn displace(&mut self, delta: Vector3<i32>) {
        if let Self::MoveSolids(solids) = self {
            for moving in solids {
                for point in &mut moving.solid.points {
                    point.position += delta;
                }
            }
        }
    }

    pub fn take(&mut self) -> Self {
        match self {
            Self::None => Self::None,
            Self::NewSolid(solid) => {
                let ret = Self::NewSolid(solid.clone());
                *self = Self::None;
                ret
            }
            Self::MoveSolids(solids) => {
                let ret = Self::MoveSolids(solids.drain(..).collect());
                *self = Self::None;
                ret
            }
        }
    }
}

pub struct MovingSolid {
    id: SolidID,
    solid: Solid,
    original_positions: [Vector3<i32>; 8],
}

impl MovingSolid {
    pub fn new(id: SolidID, solid: Solid) -> Self {
        let original_positions = solid.points.clone().map(|point| point.position);
        Self {
            id,
            solid,
            original_positions,
        }
    }
}
