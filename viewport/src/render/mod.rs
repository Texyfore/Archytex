mod gpu;

pub mod data;

use bytemuck::{Pod, Zeroable};
use cgmath::{perspective, Deg, Matrix4, SquareMatrix};
use winit::window::Window;

use self::{
    data::LineVertex,
    gpu::{Context, LinePipeline, TypedBuffer, UniformBufferGroup},
};

pub struct Renderer {
    ctx: Context,

    line_pipeline: LinePipeline,

    camera_group: UniformBufferGroup<CameraBlock>,
    camera_block: CameraBlock,

    wireframe: TypedBuffer<LineVertex>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let ctx = Context::new(window);
        let uniform_buffer_layout = ctx.create_uniform_buffer_layout();
        let line_pipeline = ctx.create_line_pipeline(&uniform_buffer_layout);
        let camera_group = ctx
            .create_uniform_buffer_group::<CameraBlock>(&uniform_buffer_layout, Default::default());
        let camera_block = Default::default();

        let wireframe = ctx.create_buffer(&[], wgpu::BufferUsages::VERTEX);

        Self {
            ctx,
            line_pipeline,
            camera_group,
            camera_block,
            wireframe,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.configure(width, height);

        let aspect = width as f32 / height as f32;
        self.camera_block.projection = perspective(Deg(80.0), aspect, 0.1, 100.0).into();
    }

    pub fn render(&self) {
        self.ctx
            .upload_uniform(&self.camera_group, self.camera_block);

        let mut frame = self.ctx.begin_frame();

        {
            let mut pass = frame.begin_pass();
            pass.set_camera_group(&self.camera_group);
            pass.draw_wireframe(&self.line_pipeline, &self.wireframe);
        }

        self.ctx.end_frame(frame);
    }

    pub fn update_camera_view(&mut self, view: Matrix4<f32>) {
        if let Some(view) = view.invert() {
            self.camera_block.view = view.into();
        }
    }

    pub fn update_wireframe(&mut self, vertices: &[LineVertex]) {
        self.wireframe = self.ctx.create_buffer(vertices, wgpu::BufferUsages::VERTEX);
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CameraBlock {
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

impl Default for CameraBlock {
    fn default() -> Self {
        let identity = Matrix4::identity().into();
        Self {
            view: identity,
            projection: identity,
        }
    }
}
