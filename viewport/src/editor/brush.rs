use std::rc::Rc;

use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3};

use crate::render::{
    data::BrushVertex, BrushCommand, BrushComponent, BrushMesh, GraphicsWorld, Texture, Transform,
};

pub struct Brush {
    points: Vec<Vector3<f32>>,
    faces: Vec<[u16; 4]>,
    transform: Rc<Transform>,
    mesh_cache: Vec<Rc<BrushMesh>>,
}

impl Brush {
    pub fn new<G: GraphicsWorld>(gfx: &G, points: Vec<Vector3<f32>>, faces: Vec<[u16; 4]>) -> Self {
        Self {
            points,
            faces,
            transform: gfx.create_transform(Matrix4::identity()),
            mesh_cache: Default::default(),
        }
    }

    pub fn regenerate<G: GraphicsWorld>(&mut self, gfx: &G) {
        let mut vertices = Vec::with_capacity(self.faces.len() * 4);
        let mut triangles = Vec::with_capacity(self.faces.len() * 2);

        for face in &self.faces {
            let t0 = vertices.len() as u16;
            triangles.push([t0, t0 + 1, t0 + 2]);
            triangles.push([t0, t0 + 2, t0 + 3]);

            let p = [
                self.points[face[0] as usize],
                self.points[face[1] as usize],
                self.points[face[2] as usize],
                self.points[face[3] as usize],
            ];

            let edge0 = p[1] - p[0];
            let edge1 = p[2] - p[0];
            let normal = edge0.cross(edge1).normalize().into();

            for i in 0..4 {
                vertices.push(BrushVertex {
                    position: p[i].into(),
                    normal,
                    texcoord: [0.0, 0.0],
                });
            }
        }

        self.mesh_cache.clear();
        self.mesh_cache
            .push(gfx.create_brush_mesh(&vertices, &triangles));
    }

    pub fn draw<G: GraphicsWorld>(&self, gfx: &mut G, texture: Rc<Texture>) {
        gfx.draw_brush(BrushCommand {
            transform: self.transform.clone(),
            components: self
                .mesh_cache
                .iter()
                .map(|mesh| BrushComponent {
                    mesh: mesh.clone(),
                    texture: texture.clone(),
                })
                .collect(),
        })
    }
}
