use std::{cmp::Ordering, rc::Rc};

use cgmath::{vec3, InnerSpace, Matrix4, Quaternion, Vector2, Vector3};

use crate::{
    math::{self, IntersectsTriangle, Ray},
    render::{
        data::BrushVertex, BrushCommand, BrushComponent, BrushMesh, GraphicsWorld, Texture,
        Transform,
    },
};

use super::config::POINT_SELECT_RADIUS;

macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point {
            position: vec3($x, $y, $z),
            selected: false,
        }
    };
}

macro_rules! face {
    ($g:ident, $t:ident, $a:literal, $b:literal, $c:literal, $d:literal) => {
        Face {
            idx: [$a, $b, $c, $d],
            texture: $t.clone(),
            selected: false,
            mesh: $g.create_brush_mesh(&[], &[]),
        }
    };
}

pub struct Brush {
    transform: Rc<Transform>,
    points: Vec<Point>,
    faces: Vec<Face>,
}

struct Point {
    position: Vector3<f32>,
    selected: bool,
}

struct Face {
    idx: [usize; 4],
    texture: Rc<Texture>,
    selected: bool,
    mesh: Rc<BrushMesh>,
}

impl Brush {
    pub fn new<G: GraphicsWorld>(
        gfx: &G,
        position: Vector3<f32>,
        extent: Vector3<f32>,
        texture: Rc<Texture>,
    ) -> Self {
        #[rustfmt::skip]
        let points = vec![
            point!( 0.0      , 0.0      , 0.0      ),
            point!( extent.x , 0.0      , 0.0      ),
            point!( extent.x , 0.0      , extent.z ),
            point!( 0.0      , 0.0      , extent.z ),
            point!( 0.0      , extent.y , 0.0      ),
            point!( extent.x , extent.y , 0.0      ),
            point!( extent.x , extent.y , extent.z ),
            point!( 0.0      , extent.y , extent.z ),
        ];

        let faces = vec![
            face!(gfx, texture, 0, 1, 2, 3),
            face!(gfx, texture, 7, 6, 5, 4),
            face!(gfx, texture, 4, 5, 1, 0),
            face!(gfx, texture, 6, 7, 3, 2),
            face!(gfx, texture, 0, 3, 7, 4),
            face!(gfx, texture, 5, 6, 2, 1),
        ];

        let transform = gfx.create_transform(Matrix4::from_translation(position));

        Self {
            transform,
            points,
            faces,
        }
    }

    pub fn set_position<G: GraphicsWorld>(&self, gfx: &G, position: Vector3<f32>) {
        gfx.update_transform(&self.transform, Matrix4::from_translation(position));
    }

    pub fn select_point<G: GraphicsWorld>(
        &mut self,
        gfx: &G,
        camera_pos: Vector3<f32>,
        pointer_pos: Vector2<f32>,
    ) {
        let sorted_points = {
            let mut points = self
                .points
                .iter()
                .enumerate()
                .map(|(i, p)| (i, p.position))
                .collect::<Vec<_>>();

            points.sort_by(|a, b| {
                let a_mag2 = (a.1 - camera_pos).magnitude2();
                let b_mag2 = (b.1 - camera_pos).magnitude2();
                a_mag2.partial_cmp(&b_mag2).unwrap_or(Ordering::Equal)
            });

            points
        };

        for (i, point) in sorted_points {
            if let Some(screen) = gfx.world_to_screen(point) {
                let mag2 = (pointer_pos - screen).magnitude2();
                let rad2 = POINT_SELECT_RADIUS * POINT_SELECT_RADIUS;

                if mag2 <= rad2 {
                    self.points[i].selected = true;
                    return;
                }
            }
        }
    }

    pub fn clear_selected_points(&mut self) {
        for point in &mut self.points {
            point.selected = false;
        }
    }

    pub fn select_face(&mut self, ray: Ray) {
        let sorted_faces = {
            let mut faces = self
                .faces
                .iter()
                .enumerate()
                .map(|(i, f)| (i, f.idx))
                .collect::<Vec<_>>();

            faces.sort_by(|(_, f1), (_, f2)| {
                let center1 = (self.points[f1[0]].position
                    + self.points[f1[1]].position
                    + self.points[f1[2]].position
                    + self.points[f1[3]].position)
                    * 0.25;

                let center2 = (self.points[f1[0]].position
                    + self.points[f2[1]].position
                    + self.points[f2[2]].position
                    + self.points[f2[3]].position)
                    * 0.25;

                let mag1 = (center1 - ray.origin).magnitude2();
                let mag2 = (center2 - ray.origin).magnitude2();
                mag1.partial_cmp(&mag2).unwrap_or(Ordering::Equal)
            });
            faces
        };

        for (i, face) in sorted_faces {
            let a = math::Triangle {
                a: self.points[face[0]].position,
                b: self.points[face[1]].position,
                c: self.points[face[2]].position,
            };

            let b = math::Triangle {
                a: self.points[face[0]].position,
                b: self.points[face[2]].position,
                c: self.points[face[3]].position,
            };

            if ray.intersects_triangle(&a) || ray.intersects_triangle(&b) {
                self.faces[i].selected = true;
                return;
            }
        }
    }

    pub fn clear_selected_faces(&mut self) {
        for face in &mut self.faces {
            face.selected = false;
        }
    }

    pub fn apply_texture(&mut self, texture: Rc<Texture>) {
        for face in &mut self.faces.iter_mut().filter(|f| f.selected) {
            face.texture = texture.clone();
        }
    }

    pub fn move_selected_points(&mut self, vector: Vector3<f32>) {
        for point in self.points.iter_mut().filter(|p| p.selected) {
            point.position += vector;
        }
    }

    pub fn rebuild<G: GraphicsWorld>(&mut self, gfx: &G) {
        for face in &mut self.faces {
            face.mesh = {
                let points = face.idx.map(|i| self.points[i].position);

                let edge0 = points[1] - points[0];
                let edge1 = points[2] - points[0];
                let normal = edge0.cross(edge1).normalize();
                let flatten = Quaternion::from_arc(-normal, Vector3::unit_y(), None);

                let normal = normal.into();
                let vertices = points
                    .iter()
                    .copied()
                    .map(|p| {
                        let texcoord = flatten * p;
                        let texcoord = [texcoord.x, texcoord.z];

                        BrushVertex {
                            position: p.into(),
                            normal,
                            texcoord,
                        }
                    })
                    .collect::<Vec<_>>();

                gfx.create_brush_mesh(&vertices, &[[0, 1, 2], [0, 2, 3]])
            }
        }
    }

    pub fn draw<G: GraphicsWorld>(&self, gfx: &mut G) {
        for face in &self.faces {
            gfx.draw_brush(BrushCommand {
                transform: self.transform.clone(),
                components: vec![BrushComponent {
                    mesh: face.mesh.clone(),
                    texture: face.texture.clone(),
                }],
            });
        }
    }
}
