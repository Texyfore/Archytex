use std::{collections::HashMap, rc::Rc};

use cgmath::{vec3, InnerSpace, Matrix4, Quaternion, Vector3};

use crate::render::{
    data::{BrushVertex, Triangle},
    BrushCommand, BrushComponent, BrushMesh, GraphicsWorld, Texture, Transform,
};

pub type Point = Vector3<f32>;

pub struct Brush {
    points: Vec<Point>,
    textures: Vec<Rc<Texture>>,
    faces: Vec<Face>,
    transform: Rc<Transform>,
    mesh_cache: HashMap<usize, Rc<BrushMesh>>,
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

        Self {
            points,
            textures,
            faces,
            transform: gfx.create_transform(transform),
            mesh_cache: Default::default(),
        }
    }

    pub fn set_point(&mut self, idx: u16, point: Vector3<f32>) {
        self.points[idx as usize] = point;
    }

    pub fn set_texture(&mut self, face: u16, texture: Rc<Texture>) {
        // TODO: This is really, really bad. But at the moment, the compiler bug
        // forces me to do it this way.

        for i in 0..self.textures.len() {
            let ptr_a = self.textures[i].as_ref() as *const Texture;
            let ptr_b = texture.as_ref() as *const Texture;
            if ptr_a == ptr_b {
                self.faces[face as usize].texture = i;
                return;
            }
        }

        self.faces[face as usize].texture = self.textures.len();
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
