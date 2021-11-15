use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use cgmath::{vec3, InnerSpace, Matrix4, Quaternion, Vector2, Vector3};

use crate::{
    info,
    math::{self, IntersectsTriangle, Ray},
    render::{
        data::{BrushVertex, Triangle},
        BrushCommand, BrushComponent, BrushMesh, GraphicsWorld, Texture, Transform,
    },
};

use super::config::POINT_SELECT_RADIUS;

pub type Point = Vector3<f32>;

pub struct Brush {
    points: Vec<Point>,
    textures: Vec<Rc<Texture>>,
    faces: Vec<Face>,
    transform: Rc<Transform>,
    mesh_cache: HashMap<usize, Rc<BrushMesh>>,
    selected_points: HashSet<u16>,
    selected_faces: HashSet<u16>,
}

struct Face {
    idx: [u16; 4],
    texture: usize,
}

impl Brush {
    pub fn new<G: GraphicsWorld>(
        gfx: &G,
        extent: Vector3<f32>,
        transform: Matrix4<f32>,
        texture: Rc<Texture>,
    ) -> Self {
        let points = vec![
            vec3(0.0, 0.0, 0.0),
            vec3(extent.x, 0.0, 0.0),
            vec3(extent.x, 0.0, extent.z),
            vec3(0.0, 0.0, extent.z),
            vec3(0.0, extent.y, 0.0),
            vec3(extent.x, extent.y, 0.0),
            vec3(extent.x, extent.y, extent.z),
            vec3(0.0, extent.y, extent.z),
        ];

        let faces = vec![
            [0, 1, 2, 3],
            [7, 6, 5, 4],
            [4, 5, 1, 0],
            [6, 7, 3, 2],
            [0, 3, 7, 4],
            [5, 6, 2, 1],
        ]
        .iter()
        .map(|idx| Face {
            idx: *idx,
            texture: 0,
        })
        .collect();

        let textures = vec![texture];

        let selected_points = HashSet::new();
        let selected_faces = HashSet::new();

        Self {
            points,
            textures,
            faces,
            transform: gfx.create_transform(transform),
            mesh_cache: Default::default(),
            selected_points,
            selected_faces,
        }
    }

    pub fn set_transform<G: GraphicsWorld>(&self, gfx: &G, transform: Matrix4<f32>) {
        gfx.update_transform(&self.transform, transform);
    }

    pub fn select_point<G: GraphicsWorld>(
        &mut self,
        gfx: &G,
        camera_pos: Vector3<f32>,
        pointer_pos: Vector2<f32>,
    ) {
        let mut sorted_points = self.points.iter().copied().enumerate().collect::<Vec<_>>();
        sorted_points.sort_by(|a, b| {
            let a_mag2 = (a.1 - camera_pos).magnitude2();
            let b_mag2 = (b.1 - camera_pos).magnitude2();
            a_mag2.partial_cmp(&b_mag2).unwrap_or(Ordering::Equal)
        });

        for (i, point) in sorted_points {
            if let Some(screen) = gfx.world_to_screen(point) {
                let mag2 = (pointer_pos - screen).magnitude2();
                let rad2 = POINT_SELECT_RADIUS * POINT_SELECT_RADIUS;

                if mag2 <= rad2 {
                    self.selected_points.insert(i as u16);
                    info!("Selected point {}", i);
                }
            }
        }
    }

    pub fn clear_point_selection(&mut self) {
        self.selected_points.clear();
    }

    pub fn select_face(&mut self, ray: Ray) {
        let mut sorted_faces = self
            .faces
            .iter()
            .enumerate()
            .map(|(i, f)| (i, f.idx))
            .collect::<Vec<_>>();

        sorted_faces.sort_by(|(_, f1), (_, f2)| {
            let center1 = (self.points[f1[0] as usize]
                + self.points[f1[1] as usize]
                + self.points[f1[2] as usize]
                + self.points[f1[3] as usize])
                * 0.25;

            let center2 = (self.points[f1[0] as usize]
                + self.points[f2[1] as usize]
                + self.points[f2[2] as usize]
                + self.points[f2[3] as usize])
                * 0.25;

            let mag1 = (center1 - ray.origin).magnitude2();
            let mag2 = (center2 - ray.origin).magnitude2();
            mag1.partial_cmp(&mag2).unwrap_or(Ordering::Equal)
        });

        for (i, face) in sorted_faces {
            let a = math::Triangle {
                a: self.points[face[0] as usize],
                b: self.points[face[1] as usize],
                c: self.points[face[2] as usize],
            };

            let b = math::Triangle {
                a: self.points[face[0] as usize],
                b: self.points[face[2] as usize],
                c: self.points[face[3] as usize],
            };

            if ray.intersects_triangle(&a) || ray.intersects_triangle(&b) {
                self.selected_faces.insert(i as u16);
                info!("Selected face {}", i);
                return;
            }
        }
    }

    pub fn clear_face_selection(&mut self) {
        self.selected_faces.clear();
    }

    pub fn set_texture(&mut self, texture: Rc<Texture>) {
        // TODO: This is really, really bad. But at the moment, the compiler bug
        // forces me to do it this way.

        for i in 0..self.textures.len() {
            let ptr_a = self.textures[i].as_ref() as *const Texture;
            let ptr_b = texture.as_ref() as *const Texture;
            if ptr_a == ptr_b {
                for face in &self.selected_faces {
                    self.faces[*face as usize].texture = i;
                }
                return;
            }
        }

        for face in &self.selected_faces {
            self.faces[*face as usize].texture = self.textures.len();
        }
        self.textures.push(texture);
    }

    pub fn regenerate<G: GraphicsWorld>(&mut self, gfx: &G) {
        let mut geometry: HashMap<usize, (Vec<BrushVertex>, Vec<Triangle>)> = HashMap::new();

        for face in &self.faces {
            let (vertices, triangles) = {
                if !geometry.contains_key(&face.texture) {
                    geometry.insert(face.texture, (Vec::new(), Vec::new()));
                }

                geometry.get_mut(&face.texture).unwrap()
            };

            let t0 = vertices.len() as u16;
            triangles.push([t0, t0 + 1, t0 + 2]);
            triangles.push([t0, t0 + 2, t0 + 3]);

            let idx = &face.idx;

            let p = [
                self.points[idx[0] as usize],
                self.points[idx[1] as usize],
                self.points[idx[2] as usize],
                self.points[idx[3] as usize],
            ];

            let edge0 = p[1] - p[0];
            let edge1 = p[2] - p[0];
            let normal = edge0.cross(edge1).normalize();

            let flatten = Quaternion::from_arc(-normal, Vector3::unit_y(), None);

            for i in 0..4 {
                let texcoord: [f32; 3] = (flatten * p[i]).into();
                vertices.push(BrushVertex {
                    position: p[i].into(),
                    normal: normal.into(),
                    texcoord: [texcoord[0], texcoord[2]],
                });
            }
        }

        self.mesh_cache = geometry
            .iter()
            .map(|(k, (v, t))| (*k, gfx.create_brush_mesh(&v, &t)))
            .collect();
    }

    pub fn draw<G: GraphicsWorld>(&self, gfx: &mut G) {
        gfx.draw_brush(BrushCommand {
            transform: self.transform.clone(),
            components: self
                .mesh_cache
                .iter()
                .map(|(texture, mesh)| BrushComponent {
                    mesh: mesh.clone(),
                    texture: self.textures[*texture].clone(),
                })
                .collect(),
        });
    }
}
