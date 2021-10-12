mod gl;
mod primitives;

pub use primitives::*;

use crate::math::{perspective, Deg, Mat4, SquareMatrix};
use bytemuck::offset_of;
use gl::{Program, Shader, ShaderKind, VertexLayout, VertexLayoutBuilder, WebGL};

pub struct Graphics {
    gl: WebGL,
    mesh_program: Program,
    mesh_layout: VertexLayout,
    line_program: Program,
    line_layout: VertexLayout,
}

impl Default for Graphics {
    fn default() -> Self {
        let gl = WebGL::default();
        gl.enable_depth_test();
        gl.cull_back_faces();
        gl.set_clear_color(Color::new(0.25, 0.25, 0.25, 1.0));

        let mesh_program = Program::new(
            &gl,
            &[
                Shader::new(&gl, ShaderKind::Vertex, include_str!("shaders/mesh.vert")),
                Shader::new(&gl, ShaderKind::Fragment, include_str!("shaders/mesh.frag")),
            ],
        );

        let mesh_layout = VertexLayoutBuilder::default()
            .with_stride(std::mem::size_of::<Vert>())
            .with_attribute(0, 3, offset_of!(Vert, pos))
            .with_attribute(1, 3, offset_of!(Vert, normal))
            .with_attribute(2, 2, offset_of!(Vert, uv))
            .build(&gl);

        let line_program = Program::new(
            &gl,
            &[
                Shader::new(&gl, ShaderKind::Vertex, include_str!("shaders/line.vert")),
                Shader::new(&gl, ShaderKind::Fragment, include_str!("shaders/line.frag")),
            ],
        );

        let line_layout = VertexLayoutBuilder::default()
            .with_stride(std::mem::size_of::<LineVert>())
            .with_attribute(0, 3, offset_of!(LineVert, pos))
            .build(&gl);

        Self {
            gl,
            mesh_program,
            mesh_layout,
            line_program,
            line_layout,
        }
    }
}

impl Graphics {
    pub fn resize_viewport(&self, width: i32, height: i32) {
        let projection = perspective(Deg(60.0), width as f32 / height as f32, 0.1, 100.0);

        self.gl.set_viewport_size(width, height);
        self.mesh_program.upload_mat4("projection", projection);
        self.line_program.upload_mat4("projection", projection);
    }

    pub fn begin(&self) {
        self.gl.clear();
        self.mesh_program.upload_mat4("view", Mat4::identity());
        self.line_program.upload_mat4("view", Mat4::identity());
    }
}
