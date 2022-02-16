mod raycast;

use std::collections::HashMap;

use asset::{GizmoID, PropID, TextureID};
use cgmath::{vec2, vec3, Deg, ElementWise, InnerSpace, Matrix4, Transform, Vector3, Zero};

use crate::{
    data::{PropInfo, PropInfoContainer},
    graphics::{
        structures::{GizmoInstance, LineVertex, SolidVertex, TransformTint},
        Canvas, GizmoGroup, GizmoInstances, Graphics, LineMesh, LineMeshDescriptor, PropData,
        PropInstance, Share, SolidMesh, SolidMeshDescriptor,
    },
    math::{MinMax, Ray},
};

pub use raycast::*;

use super::scene::Scene;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Solid,
    Face,
    Point,
    Prop,
}

pub struct Solid {
    geometry: SolidGeometry,
    selected: bool,
    graphics: SolidGraphics,
}

impl Solid {
    pub fn new(graphics: &Graphics, origin: Vector3<i32>, extent: Vector3<i32>) -> Self {
        let geometry = SolidGeometry::new(origin, extent);
        let selected = false;
        let graphics = meshgen(graphics, &geometry, selected);

        Self {
            geometry,
            selected,
            graphics,
        }
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn face_selected(&self, index: usize) -> bool {
        self.geometry.faces[index].selected
    }

    pub fn set_face_selected(&mut self, index: usize, selected: bool) {
        self.geometry.faces[index].selected = selected;
    }

    pub fn point_selected(&self, index: usize) -> bool {
        self.geometry.points[index].selected
    }

    pub fn set_point_selected(&mut self, index: usize, selected: bool) {
        self.geometry.points[index].selected = selected;
    }

    pub fn any_face_selected(&self) -> bool {
        self.geometry.faces.iter().any(|face| face.selected)
    }

    pub fn any_point_selected(&self) -> bool {
        self.geometry.points.iter().any(|point| point.selected)
    }

    pub fn retexture(&mut self, face: usize, texture: TextureID) -> TextureID {
        self.geometry.retexture(face, texture)
    }
}

struct Point {
    position: Vector3<i32>,
    selected: bool,
}

impl From<Vector3<i32>> for Point {
    fn from(position: Vector3<i32>) -> Self {
        Self {
            position,
            selected: false,
        }
    }
}

impl Point {
    pub fn meters(&self) -> Vector3<f32> {
        self.position.map(|e| e as f32 * 0.01)
    }
}

#[derive(Clone, Copy)]
pub struct PointLocator {
    pub solid: usize,
    pub point: usize,
}

struct Face {
    texture: TextureID,
    indices: [usize; 4],
    selected: bool,
}

impl From<(TextureID, [usize; 4])> for Face {
    fn from(tuple: (TextureID, [usize; 4])) -> Self {
        Self {
            texture: tuple.0,
            indices: tuple.1,
            selected: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FaceLocator {
    pub solid: usize,
    pub face: usize,
}

struct SolidGeometry {
    points: [Point; 8],
    faces: [Face; 6],
}

impl SolidGeometry {
    fn new(origin: Vector3<i32>, extent: Vector3<i32>) -> Self {
        let points = [
            vec3(0, 0, 0),
            vec3(1, 0, 0),
            vec3(1, 0, 1),
            vec3(0, 0, 1),
            vec3(0, 1, 0),
            vec3(1, 1, 0),
            vec3(1, 1, 1),
            vec3(0, 1, 1),
        ]
        .map(|point| (origin + point.mul_element_wise(extent)).into());

        let faces = [
            [1, 5, 6, 2],
            [4, 0, 3, 7],
            [5, 4, 7, 6],
            [0, 1, 2, 3],
            [3, 2, 6, 7],
            [1, 0, 4, 5],
        ]
        .map(|indices| (TextureID(0), indices).into());

        Self { points, faces }
    }

    fn displace(&mut self, delta: Vector3<i32>, mask: ElementKind) -> bool {
        match mask {
            ElementKind::Solid => {
                let mut changed = false;
                for point in &mut self.points {
                    point.position += delta;
                    changed = true;
                }
                changed
            }
            ElementKind::Face => {
                let mut changed = false;
                for face in self.faces.iter().filter(|face| face.selected) {
                    for index in face.indices {
                        let point = &mut self.points[index];
                        point.position += delta;
                        changed = true;
                    }
                }
                changed
            }
            ElementKind::Point => {
                let mut changed = false;
                for point in self.points.iter_mut().filter(|point| point.selected) {
                    point.position += delta;
                    changed = true;
                }
                changed
            }
            ElementKind::Prop => false,
        }
    }

    fn retexture(&mut self, face: usize, texture: TextureID) -> TextureID {
        let old = self.faces[face].texture;
        self.faces[face].texture = texture;
        old
    }
}

struct SolidGraphics {
    meshes: Vec<SolidMesh>,
    lines: LineMesh,
    verts: GizmoInstances,
}

impl SolidGraphics {
    fn render(&self, canvas: &mut Canvas, verts: bool) {
        for mesh in &self.meshes {
            canvas.draw_solid(mesh.share());
        }

        canvas.draw_lines(self.lines.share());

        if verts {
            canvas.draw_gizmos(GizmoGroup {
                gizmo: GizmoID(0),
                instances: self.verts.share(),
            });
        }
    }
}

pub struct Prop {
    asset: PropID,
    position: Vector3<i32>,
    rotation: Vector3<i32>,
    selected: bool,
    data: PropData,
}

impl Prop {
    pub fn new(
        graphics: &Graphics,
        asset: PropID,
        position: Vector3<i32>,
        rotation: Vector3<i32>,
    ) -> Self {
        Self {
            asset,
            position,
            rotation,
            selected: false,
            data: graphics.create_prop_data(&TransformTint {
                transform: prop_transform(position, rotation),
                tint: [0.0; 4],
            }),
        }
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn recalc(&mut self, graphics: &Graphics) {
        self.data = graphics.create_prop_data(&TransformTint {
            transform: prop_transform(self.position, self.rotation),
            tint: if self.selected {
                [0.04, 0.36, 0.85, 0.5]
            } else {
                [0.0; 4]
            },
        })
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_prop(PropInstance {
            prop: self.asset,
            data: self.data.share(),
        });
    }

    pub fn intersects(&self, infos: &PropInfoContainer, ray: &Ray) -> bool {
        if let Some(PropInfo { bounds }) = infos.get(self.asset) {
            let untransform = prop_transform(self.position, self.rotation)
                .inverse_transform()
                .unwrap();

            let ray_origin = (untransform * ray.start.extend(1.0)).truncate();
            let ray_end = (untransform * ray.end.extend(1.0)).truncate();

            let ray_dir = ray_end - ray_origin;

            let t_min = (bounds.min - ray_origin).div_element_wise(ray_dir);
            let t_max = (bounds.max - ray_origin).div_element_wise(ray_dir);
            let t1 = t_min.min(t_max);
            let t2 = t_min.max(t_max);
            let near = t1.x.max(t1.y).max(t1.z);
            let far = t2.x.min(t2.y).min(t2.z);

            if near < far {
                return true;
            }
        }

        false
    }
}

fn meshgen(graphics: &Graphics, geometry: &SolidGeometry, selected: bool) -> SolidGraphics {
    let mut batches = HashMap::<TextureID, (Vec<SolidVertex>, Vec<[u16; 3]>)>::new();
    for face in &geometry.faces {
        let normal = {
            let edge0 = geometry.points[face.indices[1]].meters()
                - geometry.points[face.indices[0]].meters();

            let edge1 = geometry.points[face.indices[3]].meters()
                - geometry.points[face.indices[0]].meters();

            edge0.cross(edge1).normalize()
        };

        let (vertices, triangles) = batches.entry(face.texture).or_default();

        let t0 = vertices.len() as u16;
        triangles.push([t0, t0 + 1, t0 + 2]);
        triangles.push([t0, t0 + 2, t0 + 3]);

        for index in face.indices {
            let position = geometry.points[index].meters();
            let texcoord = if normal.x.abs() > normal.y.abs() {
                if normal.x.abs() > normal.z.abs() {
                    vec2(position.z, position.y)
                } else {
                    vec2(position.x, position.y)
                }
            } else if normal.y.abs() > normal.z.abs() {
                vec2(position.x, position.z)
            } else {
                vec2(position.x, position.y)
            } / 4.0;

            vertices.push(SolidVertex {
                position,
                normal,
                texcoord,
                tint: if selected || face.selected {
                    [0.04, 0.36, 0.85, 0.5]
                } else {
                    [0.0; 4]
                },
            })
        }
    }

    let lines = [
        0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
    ];

    SolidGraphics {
        meshes: batches
            .into_iter()
            .map(|(texture, batch)| {
                graphics.create_solid_mesh(SolidMeshDescriptor {
                    texture,
                    vertices: &batch.0,
                    triangles: &batch.1,
                })
            })
            .collect(),
        lines: graphics.create_line_mesh(LineMeshDescriptor {
            vertices: &lines.map(|index| LineVertex {
                position: geometry.points[index].meters(),
                color: [0.0; 3],
            }),
        }),
        verts: graphics.create_gizmo_instances(
            &geometry
                .points
                .iter()
                .map(|point| GizmoInstance {
                    matrix: Matrix4::from_translation(point.meters()),
                    color: if point.selected {
                        [0.04, 0.36, 0.85]
                    } else {
                        [0.0; 3]
                    },
                })
                .collect::<Vec<_>>(),
        ),
    }
}

pub trait Movable: Sized {
    fn center(&self, mask: ElementKind) -> Vector3<f32>;
    fn displace(&mut self, delta: Vector3<i32>, mask: ElementKind) -> bool;
    fn recalc(&mut self, graphics: &Graphics);
    fn render(&self, canvas: &mut Canvas, mask: ElementKind);
    fn insert_move(
        scene: &mut Scene,
        elements: Vec<(usize, Self)>,
        delta: Vector3<i32>,
        mask: ElementKind,
    );
    fn insert(scene: &mut Scene, elements: Vec<(usize, Self)>);
}

impl Movable for Solid {
    fn center(&self, mask: ElementKind) -> Vector3<f32> {
        let mut center = Vector3::zero();
        let mut div = 0.0;

        match mask {
            ElementKind::Solid => {
                for point in &self.geometry.points {
                    center += point.meters();
                    div += 1.0;
                }
            }
            ElementKind::Face => {
                for face in &self.geometry.faces {
                    if face.selected {
                        for pid in face.indices {
                            let point = &self.geometry.points[pid];
                            center += point.meters();
                            div += 1.0;
                        }
                    }
                }
            }
            ElementKind::Point => {
                for point in self.geometry.points.iter().filter(|point| point.selected) {
                    center += point.meters();
                    div += 1.0;
                }
            }
            ElementKind::Prop => (),
        };

        center / div
    }

    fn displace(&mut self, delta: Vector3<i32>, mask: ElementKind) -> bool {
        self.geometry.displace(delta, mask)
    }

    fn recalc(&mut self, graphics: &Graphics) {
        self.graphics = meshgen(graphics, &self.geometry, self.selected);
    }

    fn render(&self, canvas: &mut Canvas, mask: ElementKind) {
        self.graphics
            .render(canvas, matches!(mask, ElementKind::Point));
    }

    fn insert_move(
        scene: &mut Scene,
        elements: Vec<(usize, Self)>,
        delta: Vector3<i32>,
        mask: ElementKind,
    ) {
        scene.insert_solids_with_move(elements, delta, mask);
    }

    fn insert(scene: &mut Scene, elements: Vec<(usize, Self)>) {
        scene.insert_solids(elements);
    }
}

fn prop_transform(position: Vector3<i32>, rotation: Vector3<i32>) -> Matrix4<f32> {
    Matrix4::from_translation(position.map(|e| e as f32))
        * Matrix4::from_angle_x(Deg(rotation.x as f32))
        * Matrix4::from_angle_y(Deg(rotation.y as f32))
        * Matrix4::from_angle_z(Deg(rotation.z as f32))
}
