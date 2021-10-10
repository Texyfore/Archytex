use crate::math::Mat4;
use super::{
    gl::{self, IndexBuffer, VertexBuffer},
    Graphics,
};
use bytemuck::{Pod, Zeroable};
use image::{EncodableLayout, GenericImageView};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Vert {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Tri {
    pub idx: [u16; 3],
}

pub struct Mesh {
    verts: VertexBuffer,
    tris: IndexBuffer,
    idx_count: i32,
}

impl Mesh {
    pub fn new(gfx: &Graphics, verts: &[Vert], tris: &[Tri]) -> Self {
        let idx_count = tris.len() as i32 * 3;

        let verts = {
            let buf = VertexBuffer::new(&gfx.gl);
            buf.upload_verts(verts);
            buf
        };

        let tris = {
            let buf = IndexBuffer::new(&gfx.gl);
            buf.upload_tris(tris);
            buf
        };

        Self {
            verts,
            tris,
            idx_count,
        }
    }

    pub fn draw(&self, gfx: &Graphics, model: Mat4, texture: &Texture) {
        gfx.mesh_program.bind();
        gfx.mesh_program.upload_mat4("model", model);

        gfx.vertex_layout.bind();
        texture.inner.bind();

        gfx.gl
            .draw_triangles(&self.verts, &self.tris, self.idx_count);

        gfx.vertex_layout.unbind();
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

pub struct Image {
    data: Box<[u8]>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn load(buf: &[u8]) -> Self {
        let image = image::load_from_memory(buf).unwrap();
        let (width, height) = image.dimensions();
        let data = image
            .as_rgba8()
            .unwrap()
            .as_bytes()
            .to_vec()
            .into_boxed_slice();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct Texture {
    inner: gl::Texture,
}

impl Texture {
    pub fn new(gfx: &Graphics, image: &Image) -> Self {
        Self {
            inner: gl::Texture::new(&gfx.gl, image),
        }
    }
}
