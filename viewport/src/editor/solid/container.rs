use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use cgmath::{vec2, vec3, ElementWise, InnerSpace, Vector2, Vector3, Zero};
use mdl::{Mesh, Model, Vertex};

use crate::{
    editor::{
        camera::WorldCamera,
        config::{FACE_HIGHLIGHT_COLOR, MAX_FACES, MAX_POINTS, MAX_SOLIDS, POINT_SELECT_RADIUS},
    },
    math::{IntersectionPoint, Plane, Ray, SolidUtil, Triangle},
    render::{SolidBatch, SolidFactory, SolidVertex, TextureBank, TextureID},
    ring_vec::RingVec,
};

pub struct SolidContainer {
    points: RingVec<Point>,
    faces: RingVec<Face>,
    solids: RingVec<Solid>,
    selected: Option<Selection>,
    needs_rebuild: bool,
    mesh_cache: Vec<(TextureID, Rc<SolidBatch>)>,
}

impl Default for SolidContainer {
    fn default() -> Self {
        Self {
            points: RingVec::new(MAX_POINTS),
            faces: RingVec::new(MAX_FACES),
            solids: RingVec::new(MAX_SOLIDS),
            selected: None,
            needs_rebuild: false,
            mesh_cache: Default::default(),
        }
    }
}

impl SolidContainer {
    pub fn add(&mut self, position: Vector3<f32>, extent: Vector3<f32>) {
        let points = [
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            vec3(1.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 1.0, 0.0),
            vec3(1.0, 1.0, 1.0),
            vec3(0.0, 1.0, 1.0),
        ]
        .map(|p| {
            let p = position + p.mul_element_wise(extent);
            self.points.push(Point {
                position: p,
                previous: p,
            })
        });

        let faces = [
            [5, 6, 2, 1],
            [7, 4, 0, 3],
            [7, 6, 5, 4],
            [2, 3, 0, 1],
            [6, 7, 3, 2],
            [4, 5, 1, 0],
        ]
        .map(|q| {
            let q = q.map(|p| points[p]);
            self.faces.push(Face {
                quad: q,
                texture: 1,
            })
        });

        self.solids.push(Solid { faces });
        self.needs_rebuild = true;
    }

    pub fn delete_selected(&mut self) {
        if let Some(Selection::Solids(solids)) = self.selected.as_ref() {
            for solid in solids {
                self.solids[*solid].faces.iter().for_each(|f| {
                    self.faces[*f].quad.iter().for_each(|p| {
                        self.points.remove(*p);
                    });
                    self.faces.remove(*f);
                });
                self.solids.remove(*solid);
            }
            self.deselect()
        }
    }

    pub fn move_selected(&mut self, vec: Vector3<f32>) {
        if let Some(selection) = self.selected.as_ref() {
            match selection {
                Selection::Points(points) => {
                    for point in points {
                        let point = self.points.get_mut(*point).unwrap();
                        point.position = point.previous + vec;
                    }
                }
                Selection::Faces(faces) => {
                    for face in faces {
                        let face = self.faces.get(*face).unwrap();
                        for point in &face.quad {
                            let point = self.points.get_mut(*point).unwrap();
                            point.position = point.previous + vec;
                        }
                    }
                }
                Selection::Solids(solids) => {
                    for solid in solids {
                        let solid = self.solids.get(*solid).unwrap();
                        for face in &solid.faces {
                            let face = self.faces.get(*face).unwrap();
                            for point in &face.quad {
                                let point = self.points.get_mut(*point).unwrap();
                                point.position = point.previous + vec;
                            }
                        }
                    }
                }
            }
            self.needs_rebuild = true;
        }
    }

    pub fn confirm_move(&mut self) {
        if let Some(selection) = self.selected.as_ref() {
            match selection {
                Selection::Points(points) => {
                    for point in points {
                        let point = self.points.get_mut(*point).unwrap();
                        point.previous = point.position;
                    }
                }
                Selection::Faces(faces) => {
                    for face in faces {
                        let face = self.faces.get(*face).unwrap();
                        for point in &face.quad {
                            let point = self.points.get_mut(*point).unwrap();
                            point.previous = point.position;
                        }
                    }
                }
                Selection::Solids(solids) => {
                    for solid in solids {
                        let solid = self.solids.get(*solid).unwrap();
                        for face in &solid.faces {
                            let face = self.faces.get(*face).unwrap();
                            for point in &face.quad {
                                let point = self.points.get_mut(*point).unwrap();
                                point.previous = point.position;
                            }
                        }
                    }
                }
            }
            self.needs_rebuild = true;
        }
    }

    pub fn abort_move(&mut self) {
        if let Some(selection) = self.selected.as_ref() {
            match selection {
                Selection::Points(points) => {
                    for point in points {
                        let point = self.points.get_mut(*point).unwrap();
                        point.position = point.previous;
                    }
                }
                Selection::Faces(faces) => {
                    for face in faces {
                        let face = self.faces.get(*face).unwrap();
                        for point in &face.quad {
                            let point = self.points.get_mut(*point).unwrap();
                            point.position = point.previous;
                        }
                    }
                }
                Selection::Solids(solids) => {
                    for solid in solids {
                        let solid = self.solids.get(*solid).unwrap();
                        for face in &solid.faces {
                            let face = self.faces.get(*face).unwrap();
                            for point in &face.quad {
                                let point = self.points.get_mut(*point).unwrap();
                                point.position = point.previous;
                            }
                        }
                    }
                }
            }
            self.needs_rebuild = true;
        }
    }

    pub fn move_plane(&self, ray: Ray) -> Option<Plane> {
        self.selected.as_ref().map(|selected| {
            let mut center = Vector3::zero();

            match selected {
                Selection::Points(points) => points
                    .iter()
                    .map(|p| self.points[*p].position)
                    .for_each(|p| center += p / points.len() as f32),
                Selection::Faces(faces) => {
                    let mut div = 0.0;
                    faces
                        .iter()
                        .map(|f| self.faces[*f].quad.iter())
                        .flatten()
                        .map(|p| self.points[*p].position)
                        .for_each(|p| {
                            center += p;
                            div += 1.0
                        });
                    center /= div;
                }
                Selection::Solids(solids) => {
                    let mut div = 0.0;
                    solids
                        .iter()
                        .map(|s| self.solids[*s].faces.iter())
                        .flatten()
                        .map(|f| self.faces[*f].quad.iter())
                        .flatten()
                        .map(|p| self.points[*p].position)
                        .for_each(|p| {
                            center += p;
                            div += 1.0
                        });
                    center /= div;
                }
            }

            Plane {
                origin: center,
                normal: -(ray.vec().normalize()).cardinal(),
            }
        })
    }

    pub fn select_point(&mut self, camera: &WorldCamera, position: Vector2<f32>) {
        if self.selected.is_none() {
            self.selected = Some(Selection::Points(Vec::new()));
        }

        if let Some(Selection::Points(_)) = self.selected.as_ref() {
            let mut candidates = Vec::new();
            for (i, point) in &self.points {
                if let Some(screen_pos) = camera.project(point.position, 0.0) {
                    let screen_pos = vec2(screen_pos.x, screen_pos.y);
                    let dist = (screen_pos - position).magnitude2();
                    if dist < POINT_SELECT_RADIUS * POINT_SELECT_RADIUS {
                        candidates.push((i, dist));
                    }
                }
            }

            candidates
                .sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            if let Some((i, _)) = candidates.get(0) {
                let point = &self.points[*i];

                let ray = Ray {
                    origin: camera.position(),
                    end: point.position,
                };

                let can_select = if let Some(Raycast {
                    point: hit_point, ..
                }) = self.raycast(ray)
                {
                    let a = (point.position - ray.origin).magnitude2();
                    let b = (hit_point - ray.origin).magnitude2();
                    if (a - b).abs() > 0.1 {
                        a < b
                    } else {
                        true
                    }
                } else {
                    true
                };

                if can_select {
                    if let Some(Selection::Points(points)) = self.selected.as_mut() {
                        if points.contains(i) {
                            points.retain(|p| *p != *i);
                        } else {
                            points.push(*i);
                        }
                    }
                }
            }
        }
    }

    pub fn select_face(&mut self, camera: &WorldCamera, position: Vector2<f32>) {
        if self.selected.is_none() {
            self.selected = Some(Selection::Faces(Vec::new()));
        }

        let raycast = self.raycast(camera.screen_ray(position));
        if let (
            Some(Selection::Faces(faces)),
            Some(Raycast {
                solid: Some(RaycastSolid { face, .. }),
                ..
            }),
        ) = (self.selected.as_mut(), raycast)
        {
            if faces.contains(&face) {
                faces.retain(|f| *f != face);
            } else {
                faces.push(face);
            }
            self.needs_rebuild = true;
        }
    }

    pub fn select_solid(&mut self, camera: &WorldCamera, position: Vector2<f32>) {
        if self.selected.is_none() {
            self.selected = Some(Selection::Solids(Vec::new()));
        }

        let raycast = self.raycast(camera.screen_ray(position));
        if let (
            Some(Selection::Solids(solids)),
            Some(Raycast {
                solid: Some(RaycastSolid { solid, .. }),
                ..
            }),
        ) = (self.selected.as_mut(), raycast)
        {
            if solids.contains(&solid) {
                solids.retain(|s| *s != solid);
            } else {
                solids.push(solid);
            }
            self.needs_rebuild = true;
        }
    }

    pub fn deselect(&mut self) {
        if self.selected.is_some() {
            self.needs_rebuild = true;
            self.selected = None;
        }
    }

    pub fn copy_solids(&mut self) {
        if let Some(Selection::Solids(solids)) = self.selected.as_mut() {
            let mut new_selection = Vec::new();

            for solid in solids.iter() {
                let mut points = HashSet::new();
                self.solids[*solid]
                    .faces
                    .iter()
                    .map(|f| self.faces[*f].quad.iter())
                    .flatten()
                    .for_each(|p| {
                        points.insert(*p);
                    });

                let points = points
                    .iter()
                    .map(|p| (*p, self.points.push(self.points[*p].clone())))
                    .collect::<HashMap<_, _>>();

                let faces = self.solids[*solid].faces.map(|f| {
                    let face = self.faces[f].clone();
                    self.faces.push(Face {
                        quad: face.quad.map(|p| points[&p]),
                        texture: face.texture,
                    })
                });

                new_selection.push(self.solids.push(Solid { faces }));
            }

            *solids = new_selection;
            self.needs_rebuild = true;
        }
    }

    pub fn raycast(&self, ray: Ray) -> Option<Raycast> {
        let mut hits = Vec::new();

        for (i, solid) in self.solids.iter() {
            for j in solid.faces {
                let face = &self.faces[j];
                let tri0 = Triangle {
                    a: self.points[face.quad[0]].position,
                    b: self.points[face.quad[1]].position,
                    c: self.points[face.quad[2]].position,
                };
                let tri1 = Triangle {
                    a: self.points[face.quad[0]].position,
                    b: self.points[face.quad[2]].position,
                    c: self.points[face.quad[3]].position,
                };

                if let Some(point) = ray.intersection_point(&tri0) {
                    hits.push((i, j, point, tri0.normal()));
                } else if let Some(point) = ray.intersection_point(&tri1) {
                    hits.push((i, j, point, tri1.normal()));
                }
            }
        }

        hits.sort_unstable_by(|(_, _, c1, _), (_, _, c2, _)| {
            let a = (c1 - ray.origin).magnitude2();
            let b = (c2 - ray.origin).magnitude2();
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        if let Some((solid, face, point, normal)) = hits.get(0).copied() {
            Some(Raycast {
                point,
                normal,
                solid: Some(RaycastSolid { solid, face }),
            })
        } else {
            let plane = Plane {
                origin: Vector3::zero(),
                normal: Vector3::unit_y(),
            };

            ray.intersection_point(&plane).map(|point| Raycast {
                point,
                normal: plane.normal,
                solid: None,
            })
        }
    }

    pub fn rebuild(&mut self, factory: &SolidFactory, textures: &TextureBank) {
        if self.needs_rebuild {
            let mut batches = HashMap::new();
            for (i, solid) in &self.solids {
                for j in &solid.faces {
                    let face = self.faces.get(*j).unwrap();
                    let selected = if let Some(selected) = self.selected.as_ref() {
                        match selected {
                            Selection::Points(_) => false,
                            Selection::Faces(faces) => faces.contains(j),
                            Selection::Solids(solids) => solids.contains(&i),
                        }
                    } else {
                        false
                    };

                    let texture = if textures.exists(face.texture) {
                        face.texture
                    } else {
                        1
                    };

                    batches
                        .entry(texture)
                        .or_insert_with(|| (Vec::new(), Vec::new()));

                    let (vertices, triangles) = batches.get_mut(&texture).unwrap();

                    let t0 = vertices.len() as u16;
                    triangles.push([t0, t0 + 1, t0 + 2]);
                    triangles.push([t0, t0 + 2, t0 + 3]);

                    let points = face.quad.map(|i| self.points[i].position);
                    let edge0 = points[1] - points[0];
                    let edge1 = points[3] - points[0];
                    let normal = (edge0.cross(edge1)).normalize();

                    let color = if selected {
                        FACE_HIGHLIGHT_COLOR
                    } else {
                        [1.0; 4]
                    };

                    if let Some(texture_size) = textures.size_of(texture) {
                        let texture_size = texture_size.map(|x| x as f32);
                        let scale_factor = texture_size.div_element_wise(vec2(256.0, 256.0));
                        for point in points {
                            let texcoord = point.texcoord(normal).div_element_wise(scale_factor);
                            vertices.push(SolidVertex {
                                position: point.into(),
                                normal: normal.into(),
                                texcoord: texcoord.into(),
                                color,
                            });
                        }
                    }
                }
            }

            self.mesh_cache = batches
                .iter()
                .map(|(t, (v, i))| (*t, factory.create(v, i)))
                .collect();

            self.needs_rebuild = false;
        }
    }

    pub fn mesh(&self) -> Vec<(TextureID, Rc<SolidBatch>)> {
        self.mesh_cache.clone()
    }

    pub fn point_graphics(&self) -> Vec<PointGraphics> {
        if let Some(Selection::Points(selected)) = self.selected.as_ref() {
            self.points
                .iter()
                .map(|(i, p)| PointGraphics {
                    position: p.position,
                    selected: selected.contains(&i),
                })
                .collect()
        } else {
            self.points
                .iter()
                .map(|(_, p)| PointGraphics {
                    position: p.position,
                    selected: false,
                })
                .collect()
        }
    }

    pub fn export(&self, textures: &TextureBank) -> Model {
        let mut meshes = HashMap::new();

        for (_, solid) in self.solids.iter().filter(|(i, _)| *i != 0) {
            for face in solid.faces.map(|face| &self.faces[face]) {
                let (vertices, triangles) = meshes
                    .entry(&face.texture)
                    .or_insert_with(|| (Vec::new(), Vec::new()));

                let t0 = vertices.len() as u16;
                triangles.push(mdl::Triangle {
                    a: t0,
                    b: t0 + 1,
                    c: t0 + 2,
                });
                triangles.push(mdl::Triangle {
                    a: t0,
                    b: t0 + 2,
                    c: t0 + 3,
                });

                let points = face.quad.map(|i| self.points[i].position);
                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                let normal = (edge0.cross(edge1)).normalize();

                let texture = if textures.exists(face.texture) {
                    face.texture
                } else {
                    1
                };

                if let Some(texture_size) = textures.size_of(texture) {
                    let texture_size = texture_size.map(|x| x as f32);
                    let scale_factor = texture_size.div_element_wise(vec2(256.0, 256.0));
                    for point in points {
                        let texcoord = point.texcoord(normal).div_element_wise(scale_factor);
                        vertices.push(Vertex {
                            position: mdl::Vector3 {
                                x: point.x,
                                y: point.y,
                                z: point.z,
                            },
                            normal: mdl::Vector3 {
                                x: normal.x,
                                y: normal.y,
                                z: normal.z,
                            },
                            texcoord: mdl::Vector2 {
                                x: texcoord.x,
                                y: texcoord.y,
                            },
                        });
                    }
                }
            }
        }

        Model {
            meshes: meshes
                .into_iter()
                .map(|(texture, (vertices, triangles))| Mesh {
                    vertices,
                    triangles,
                    texture_id: mdl::TextureID(*texture),
                })
                .collect(),
        }
    }
}

pub struct Raycast {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub solid: Option<RaycastSolid>,
}

pub struct RaycastSolid {
    pub solid: usize,
    pub face: usize,
}

pub struct PointGraphics {
    pub position: Vector3<f32>,
    pub selected: bool,
}

#[derive(Clone)]
struct Point {
    position: Vector3<f32>,
    previous: Vector3<f32>,
}

#[derive(Clone)]
struct Face {
    quad: [usize; 4],
    texture: TextureID,
}

struct Solid {
    faces: [usize; 6],
}

enum Selection {
    Points(Vec<usize>),
    Faces(Vec<usize>),
    Solids(Vec<usize>),
}
