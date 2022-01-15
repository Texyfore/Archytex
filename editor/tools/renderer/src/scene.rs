use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use gpu::data::buffer::Buffer;
use tk3d::{math::Vector3, Triangle, Vertex};

#[derive(Default)]
pub struct Scene {
    meshes: Vec<Rc<Mesh>>,
    lines: Vec<Rc<Lines>>,
    sprites: Vec<Sprite>,
}

impl Scene {
    pub fn push_mesh(&mut self, mesh: Rc<Mesh>) {
        self.meshes.push(mesh);
    }

    pub fn push_lines(&mut self, lines: Rc<Lines>) {
        self.lines.push(lines);
    }

    pub fn push_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }
}

pub struct Mesh {
    pub(crate) vertices: Buffer<Vertex>,
    pub(crate) triangles: Buffer<Triangle>,
}

pub struct Lines {
    pub(crate) vertices: Buffer<LineVertex>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LineVertex {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
}

unsafe impl Zeroable for LineVertex {}
unsafe impl Pod for LineVertex {}

pub struct Sprite;
