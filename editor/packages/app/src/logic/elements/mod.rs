mod raycast;

use std::collections::HashMap;

use asset::{GizmoID, PropID, TextureID};
use cgmath::{vec2, vec3, ElementWise, InnerSpace, Matrix4, Vector3};

use crate::graphics::{
    structures::{GizmoInstance, LineVertex, SolidVertex},
    Canvas, GizmoGroup, GizmoInstances, Graphics, LineMesh, LineMeshDescriptor, PropData,
    PropInstance, Share, SolidMesh, SolidMeshDescriptor,
};

pub use raycast::*;

#[derive(Clone, Copy)]
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

    pub fn displace(&mut self, delta: Vector3<i32>, mask: ElementKind) {
        self.geometry.displace(delta, mask);
    }

    pub fn retexture(&mut self, texture: TextureID) {
        self.geometry.retexture(texture);
    }

    pub fn recalc(&mut self, graphics: &Graphics) {
        self.graphics = meshgen(graphics, &self.geometry, self.selected);
    }

    pub fn render(&self, canvas: &mut Canvas, verts: bool) {
        self.graphics.render(canvas, verts);
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

    fn displace(&mut self, delta: Vector3<i32>, mask: ElementKind) {
        match mask {
            ElementKind::Solid => {
                for point in &mut self.points {
                    point.position += delta;
                }
            }
            ElementKind::Face => {
                for face in self.faces.iter().filter(|face| face.selected) {
                    for index in face.indices {
                        let point = &mut self.points[index];
                        point.position += delta;
                    }
                }
            }
            ElementKind::Point => {
                for point in self.points.iter_mut().filter(|point| point.selected) {
                    point.position += delta;
                }
            }
            ElementKind::Prop => (),
        };
    }

    fn retexture(&mut self, texture: TextureID) {
        for face in self.faces.iter_mut().filter(|face| face.selected) {
            face.texture = texture;
        }
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
    data: PropData,
}

impl Prop {
    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_prop(PropInstance {
            prop: self.asset,
            data: self.data.share(),
        });
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