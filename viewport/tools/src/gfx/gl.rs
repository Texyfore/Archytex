use super::{Color, Image, Tri, Vert};
use crate::{math::Mat4, web_util};
use glow::*;
use std::rc::Rc;

pub struct WebGL {
    ctx: Rc<Context>,
}

impl Default for WebGL {
    fn default() -> Self {
        Self {
            ctx: Rc::new(Context::from_webgl2_context(web_util::get_webgl_context())),
        }
    }
}

impl WebGL {
    pub fn enable_depth_test(&self) {
        unsafe {
            self.ctx.enable(DEPTH_TEST);
            self.ctx.depth_func(LEQUAL);
        }
    }

    pub fn cull_back_faces(&self) {
        unsafe {
            self.ctx.enable(CULL_FACE);
            self.ctx.cull_face(BACK);
        }
    }

    pub fn set_clear_color(&self, color: Color) {
        unsafe { self.ctx.clear_color(color.r, color.g, color.b, color.a) };
    }

    pub fn clear(&self) {
        unsafe { self.ctx.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT) };
    }

    pub fn set_viewport_size(&self, width: i32, height: i32) {
        unsafe { self.ctx.viewport(0, 0, width, height) };
    }

    pub fn draw_triangles(&self, verts: &VertexBuffer, tris: &IndexBuffer, idx_count: i32) {
        unsafe {
            self.ctx.bind_buffer(ARRAY_BUFFER, Some(verts.inner));
            self.ctx.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(tris.inner));
            self.ctx
                .draw_elements(TRIANGLES, idx_count, UNSIGNED_SHORT, 0);
        }
    }
}

pub struct VertexBuffer {
    ctx: Rc<Context>,
    inner: WebBufferKey,
}

impl VertexBuffer {
    pub fn new(gl: &WebGL) -> Self {
        let ctx = gl.ctx.clone();
        let inner = unsafe { gl.ctx.create_buffer().unwrap() };
        Self { ctx, inner }
    }

    pub fn upload_verts(&self, verts: &[Vert]) {
        unsafe {
            self.ctx.bind_buffer(ARRAY_BUFFER, Some(self.inner));
            self.ctx
                .buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(verts), STATIC_DRAW);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_buffer(self.inner) }
    }
}

pub struct IndexBuffer {
    ctx: Rc<Context>,
    inner: WebBufferKey,
}

impl IndexBuffer {
    pub fn new(gl: &WebGL) -> Self {
        let ctx = gl.ctx.clone();
        let inner = unsafe { gl.ctx.create_buffer().unwrap() };
        Self { ctx, inner }
    }

    pub fn upload_tris(&self, tris: &[Tri]) {
        unsafe {
            self.ctx.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(self.inner));
            self.ctx.buffer_data_u8_slice(
                ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(tris),
                STATIC_DRAW,
            );
        }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_buffer(self.inner) }
    }
}

pub struct Shader {
    ctx: Rc<Context>,
    inner: WebShaderKey,
}

impl Shader {
    pub fn new(gl: &WebGL, kind: ShaderKind, src: &str) -> Self {
        let ctx = gl.ctx.clone();
        let inner = unsafe {
            let shader = ctx
                .create_shader(match kind {
                    ShaderKind::Vertex => VERTEX_SHADER,
                    ShaderKind::Fragment => FRAGMENT_SHADER,
                })
                .unwrap();

            ctx.shader_source(shader, src);
            ctx.compile_shader(shader);

            let info = ctx.get_shader_info_log(shader);
            if !info.is_empty() {
                panic!("Compile error in shader: {}", info);
            }

            shader
        };

        Self { ctx, inner }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_shader(self.inner) };
    }
}

pub enum ShaderKind {
    Vertex,
    Fragment,
}

pub struct Program {
    ctx: Rc<Context>,
    inner: WebProgramKey,
}

impl Program {
    pub fn new(gl: &WebGL, shaders: &[Shader]) -> Self {
        let ctx = gl.ctx.clone();
        let inner = unsafe {
            let program = ctx.create_program().unwrap();

            for shader in shaders {
                ctx.attach_shader(program, shader.inner);
            }

            ctx.link_program(program);

            let info = ctx.get_program_info_log(program);
            if !info.is_empty() {
                panic!("Program link error: {}", info);
            }

            for shader in shaders {
                ctx.detach_shader(program, shader.inner);
            }

            program
        };

        Self { ctx, inner }
    }

    pub fn bind(&self) {
        unsafe { self.ctx.use_program(Some(self.inner)) };
    }

    pub fn upload_mat4(&self, uniform: &str, value: Mat4) {
        unsafe {
            let location =
                self.ctx.get_uniform_location(self.inner, uniform).unwrap() as UniformLocation;

            self.ctx.use_program(Some(self.inner));
            self.ctx
                .uniform_matrix_4_f32_slice(Some(&location), false, value.as_ref());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_program(self.inner) }
    }
}

pub struct VertexLayout {
    ctx: Rc<Context>,
    stride: i32,
    attribs: Vec<VertexAttribute>,
}

impl VertexLayout {
    pub fn bind(&self) {
        unsafe {
            for attrib in &self.attribs {
                self.ctx.enable_vertex_attrib_array(attrib.location);
                self.ctx.vertex_attrib_pointer_f32(
                    attrib.location,
                    attrib.components,
                    FLOAT,
                    false,
                    self.stride,
                    attrib.offset,
                );
            }
        }
    }

    pub fn unbind(&self) {
        for attrib in &self.attribs {
            unsafe { self.ctx.disable_vertex_attrib_array(attrib.location) };
        }
    }
}

#[derive(Default)]
pub struct VertexLayoutBuilder {
    stride: i32,
    attribs: Vec<VertexAttribute>,
}

impl VertexLayoutBuilder {
    pub fn with_stride(mut self, stride: usize) -> Self {
        self.stride = stride as i32;
        self
    }

    pub fn with_attribute(mut self, location: u32, components: i32, offset: usize) -> Self {
        self.attribs.push(VertexAttribute {
            location,
            components,
            offset: offset as i32,
        });
        self
    }

    pub fn build(self, gl: &WebGL) -> VertexLayout {
        VertexLayout {
            ctx: gl.ctx.clone(),
            stride: self.stride,
            attribs: self.attribs,
        }
    }
}

struct VertexAttribute {
    location: u32,
    components: i32,
    offset: i32,
}

pub struct Texture {
    ctx: Rc<Context>,
    inner: WebTextureKey,
}

impl Texture {
    pub fn new(gl: &WebGL, image: &Image) -> Self {
        let ctx = gl.ctx.clone();
        let inner = unsafe {
            let texture = ctx.create_texture().unwrap();

            ctx.bind_texture(TEXTURE_2D, Some(texture));
            ctx.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
            ctx.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
            ctx.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
            ctx.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
            ctx.tex_image_2d(
                TEXTURE_2D,
                0,
                RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                RGBA,
                UNSIGNED_BYTE,
                Some(image.data()),
            );

            texture
        };

        Self { ctx, inner }
    }

    pub fn bind(&self) {
        unsafe {
            self.ctx.active_texture(TEXTURE0);
            self.ctx.bind_texture(TEXTURE_2D, Some(self.inner));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_texture(self.inner) };
    }
}
