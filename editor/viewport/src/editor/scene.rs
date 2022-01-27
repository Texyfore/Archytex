use std::collections::{HashMap, HashSet};

use asset_id::TextureID;
use cgmath::{vec3, MetricSpace, Vector3, Zero};
use renderer::Renderer;

use crate::math::{Intersects, Plane, Ray, Sphere, Triangle};

use super::{
    elements::{ElementKind, FaceID, Movable, PointID, PropID, Solid, SolidID},
    graphics::{self, Graphics, MeshGenInput},
};

#[derive(Default)]
pub struct Scene {
    solids: HashMap<SolidID, Solid>,
    next_solid_id: u32,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
    hidden_solids: HashSet<SolidID>,
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

    pub fn clone_and_hide_solids(&mut self, mask: ElementKind) -> Vec<(SolidID, Solid)> {
        self.solids
            .iter()
            .filter(|(_, solid)| match mask {
                ElementKind::Solid => solid.selected,
                ElementKind::Face => solid.faces.iter().any(|face| face.selected),
                ElementKind::Point => solid.points.iter().any(|point| point.selected),
                ElementKind::Prop => false,
            })
            .map(|(id, solid)| {
                self.hidden_solids.insert(*id);
                (*id, solid.clone())
            })
            .collect()
    }

    pub fn unhide_all(&mut self) {
        self.hidden_solids.clear();
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

    pub fn regen(&self, renderer: &Renderer, graphics: &mut Option<Graphics>, mask: ElementKind) {
        graphics::generate(
            MeshGenInput {
                renderer,
                mask,
                solids: self
                    .solids
                    .iter()
                    .filter(|(id, _)| !self.hidden_solids.contains(id))
                    .map(|(_, solid)| solid),
            },
            graphics,
        );
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

            Action::AssignTexture(texture) => {
                let mut old_texture_ids = Vec::new();

                for (solid_id, solid) in &mut self.solids {
                    for (face_id, face) in solid.faces.iter_mut().enumerate() {
                        if face.selected {
                            old_texture_ids.push((*solid_id, FaceID(face_id), face.texture));
                            face.texture = texture;
                        }
                    }
                }

                (!old_texture_ids.is_empty()).then(|| Action::AssignTextures(old_texture_ids))
            }

            Action::AssignTextures(ids) => {
                let mut old_texture_ids = Vec::new();

                for (solid_id, face_id, texture) in ids {
                    let solid = self.solids.get_mut(&solid_id).unwrap();
                    let face = &mut solid.faces[face_id.0];
                    old_texture_ids.push((solid_id, face_id, face.texture));
                    face.texture = texture;
                }

                (!old_texture_ids.is_empty()).then(|| Action::AssignTextures(old_texture_ids))
            }

            Action::Move { kind, delta } => {
                let mut modified = false;

                for solid in self.solids.values_mut().filter(|solid| match kind {
                    ElementKind::Solid => solid.selected,
                    ElementKind::Face => solid.faces.iter().any(|face| face.selected),
                    ElementKind::Point => solid.points.iter().any(|point| point.selected),
                    ElementKind::Prop => false,
                }) {
                    if solid.displace(kind, delta) {
                        modified = true;
                    }
                }

                modified.then(|| Action::Move {
                    kind,
                    delta: -delta,
                })
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

    Move {
        kind: ElementKind,
        delta: Vector3<i32>,
    },
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
