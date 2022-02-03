use std::collections::{HashMap, HashSet};

use asset_id::{PropID, TextureID};
use cgmath::{InnerSpace, MetricSpace, Vector2, Vector3, Zero};
use formats::ascn;
use renderer::Renderer;

use crate::math::{Intersects, Plane, Ray, Sphere, Triangle};

use super::{
    camera::Camera,
    elements::{ElementKind, FaceID, Movable, PointID, Prop, Solid, SolidID},
    graphics::{self, Graphics, MeshGenInput},
};

#[derive(Default)]
pub struct Scene {
    solids: HashMap<SolidID, Solid>,
    props: HashMap<PropID, Prop>,
    next_entity_id: u32,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
    hidden_solids: HashSet<SolidID>,
    hidden_props: HashSet<PropID>,
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

    pub fn clone_solids(&mut self) -> Vec<Solid> {
        self.solids
            .iter_mut()
            .filter(|(_, solid)| solid.selected)
            .map(|(_, solid)| {
                let new = solid.clone();
                solid.selected = false;
                new
            })
            .collect()
    }

    pub fn unhide_all(&mut self) {
        self.hidden_solids.clear();
    }

    pub fn raycast(&self, screen_pos: Vector2<f32>, camera: &Camera) -> RaycastHit {
        struct HitPoint {
            solid_id: SolidID,
            point_id: PointID,
            world: Vector3<f32>,
            screen: Vector2<f32>,
        }

        let ray = camera.screen_ray(screen_pos);

        let mut hit_faces = self.raycast_faces(&ray);
        let mut hit_points = Vec::new();

        for (id, solid) in &self.solids {
            for (i, point) in solid.points.iter().enumerate() {
                let origin = point.meters();
                let dist = origin.distance(ray.start);

                let sphere = Sphere {
                    origin,
                    radius: dist * 0.1,
                };

                if ray.intersects(&sphere).is_some() {
                    if let Some(screen) = camera.project(origin) {
                        let screen = screen.truncate();
                        hit_points.push(HitPoint {
                            solid_id: *id,
                            point_id: PointID(i),
                            world: origin,
                            screen,
                        });
                    }
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
                normal: Vector3::unit_y(),
            };

            ray.intersects(&plane).map(|intersection| RaycastEndpoint {
                point: intersection.point,
                normal: intersection.normal,
                kind: RaycastEndpointKind::Ground,
            })
        };

        hit_points.sort_unstable_by(|a, b| {
            let dist_a = screen_pos.distance2(a.screen);
            let dist_b = screen_pos.distance2(b.screen);
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        if !hit_points.is_empty() {
            let first = hit_points[0].screen;
            hit_points.retain(|hit| {
                camera
                    .project(hit.world)
                    .unwrap()
                    .truncate()
                    .distance2(first)
                    < 25.0
            });
        }

        hit_points.retain(|hit| {
            let ray = camera.screen_ray(hit.screen);
            let dist = (hit.world - ray.start).magnitude2() * 0.99;
            self.raycast_faces(&ray)
                .iter()
                .all(|hit| hit.point.distance2(ray.start) > dist)
        });

        RaycastHit {
            endpoint,
            points: hit_points
                .into_iter()
                .map(|hit_point| (hit_point.solid_id, hit_point.point_id))
                .collect(),
        }
    }

    fn raycast_faces(&self, ray: &Ray) -> Vec<HitFace> {
        let mut hit_faces = Vec::new();

        for (solid_id, solid) in &self.solids {
            for (i, face) in solid.faces.iter().enumerate() {
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
                            face_id: FaceID(i),
                            point: intersection.point,
                            normal: intersection.normal,
                        });
                        break;
                    }
                }
            }
        }

        hit_faces
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
                props: self
                    .props
                    .iter()
                    .filter(|(id, _)| !self.hidden_props.contains(id))
                    .map(|(_, prop)| prop),
            },
            graphics,
        );
    }

    pub fn as_ascn_model(&self) -> ascn::Model {
        ascn::Model {
            solids: self
                .solids
                .values()
                .map(|solid| ascn::Solid {
                    faces: solid.faces.clone().map(|face| ascn::Face {
                        texture_id: face.texture,
                        points: face.points.map(|point| ascn::PointID(point.0)),
                    }),
                    points: solid.points.clone().map(|point| ascn::Point {
                        position: point.position,
                    }),
                })
                .collect(),
        }
    }

    fn execute_action(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NewSolids(solids) => {
                let mut ids = Vec::new();

                for solid in solids {
                    let id = SolidID(self.next_entity_id);
                    self.next_entity_id += 1;

                    self.solids.insert(id, solid);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            Action::AddSolids(solids) => {
                let mut ids = Vec::new();

                for (id, solid) in solids {
                    self.solids.insert(id, solid);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            Action::NewProps(props) => {
                let mut ids = Vec::new();

                for prop in props {
                    let id = PropID(self.next_entity_id);
                    self.next_entity_id += 1;

                    self.props.insert(id, prop);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveProps(ids))
            }

            Action::AddProps(props) => {
                let mut ids = Vec::new();

                for (id, prop) in props {
                    self.props.insert(id, prop);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveProps(ids))
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

            Action::RemoveProps(ids) => {
                let mut props = Vec::new();
                for id in ids {
                    let prop = self.props.remove(&id).unwrap();
                    props.push((id, prop));
                }

                (!props.is_empty()).then(|| Action::AddProps(props))
            }

            Action::RemoveSelectedProps => {
                let ids = self
                    .props
                    .iter()
                    .filter(|(_, prop)| prop.selected)
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();

                let mut props = Vec::new();
                for id in ids {
                    props.push((id, self.props.remove(&id).unwrap()));
                }

                (!props.is_empty()).then(|| Action::AddProps(props))
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

            Action::SelectProps(ids) => {
                for id in &ids {
                    let prop = self.props.get_mut(id).unwrap();
                    prop.selected = !prop.selected;
                }

                (!ids.is_empty()).then(|| Action::SelectProps(ids))
            }

            Action::DeselectProps => {
                let mut ids = Vec::new();

                for (id, prop) in &mut self.props {
                    if prop.selected {
                        prop.selected = false;
                        ids.push(*id);
                    }
                }

                (!ids.is_empty()).then(|| Action::SelectProps(ids))
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

struct HitFace {
    solid_id: SolidID,
    face_id: FaceID,
    point: Vector3<f32>,
    normal: Vector3<f32>,
}

pub enum Action {
    NewSolids(Vec<Solid>),
    AddSolids(Vec<(SolidID, Solid)>),

    NewProps(Vec<Prop>),
    AddProps(Vec<(PropID, Prop)>),

    RemoveSolids(Vec<SolidID>),
    RemoveSelectedSolids,

    RemoveProps(Vec<PropID>),
    RemoveSelectedProps,

    SelectSolids(Vec<SolidID>),
    DeselectSolids,

    SelectFaces(Vec<(SolidID, FaceID)>),
    DeselectFaces,

    SelectPoints(Vec<(SolidID, PointID)>),
    DeselectPoints,

    SelectProps(Vec<PropID>),
    DeselectProps,

    AssignTexture(TextureID),
    AssignTextures(Vec<(SolidID, FaceID, TextureID)>),

    Move {
        kind: ElementKind,
        delta: Vector3<i32>,
    },
}

pub struct RaycastHit {
    pub endpoint: Option<RaycastEndpoint>,
    pub points: Vec<(SolidID, PointID)>,
}

pub struct RaycastEndpoint {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub kind: RaycastEndpointKind,
}

pub enum RaycastEndpointKind {
    Face { solid_id: SolidID, face_id: FaceID },
    Prop(PropID),
    Ground,
}
