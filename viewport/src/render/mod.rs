mod gpu;

pub mod data;

use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use cgmath::{perspective, Deg, Matrix4, SquareMatrix};
use winit::window::Window;

use self::{
    data::{BrushVertex, LineVertex, Triangle},
    gpu::{BrushPipeline, Context, LinePipeline, MsaaFramebuffer, TypedBuffer, UniformBufferGroup},
};

pub const MSAA_SAMPLE_COUNT: u32 = 4;

pub trait GraphicsWorld {
    fn update_camera_view(&mut self, view: Matrix4<f32>);
    fn update_grid(&mut self, cell_count: i32, cell_size: f32);
    fn create_brush_mesh(&self, vertices: &[BrushVertex], triangles: &[Triangle]) -> Rc<BrushMesh>;
    fn draw_brush_mesh(&mut self, brush_mesh: Rc<BrushMesh>);
}

pub struct Renderer {
    ctx: Context,
    msaa: MsaaFramebuffer,

    line_pipeline: LinePipeline,
    brush_pipeline: BrushPipeline,

    camera_group: UniformBufferGroup<CameraBlock>,
    camera_block: CameraBlock,

    grid: TypedBuffer<LineVertex>,
    brush_meshes: Vec<Rc<BrushMesh>>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let ctx = Context::new(window);

        let msaa = {
            let (width, height) = window.inner_size().into();
            ctx.create_msaa_framebuffer(width, height)
        };

        let uniform_buffer_layout = ctx.create_uniform_buffer_layout();
        let line_pipeline = ctx.create_line_pipeline(&uniform_buffer_layout);
        let brush_pipeline = ctx.create_brush_pipeline(&uniform_buffer_layout);

        let camera_group = ctx
            .create_uniform_buffer_group::<CameraBlock>(&uniform_buffer_layout, Default::default());
        let camera_block = Default::default();

        let grid = ctx.create_buffer(&[], wgpu::BufferUsages::VERTEX);
        let brush_meshes = Default::default();

        Self {
            ctx,
            msaa,
            line_pipeline,
            brush_pipeline,
            camera_group,
            camera_block,
            grid,
            brush_meshes,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.configure(width, height);
        self.msaa = self.ctx.create_msaa_framebuffer(width, height);

        let aspect = width as f32 / height as f32;
        self.camera_block.projection = perspective(Deg(80.0), aspect, 0.1, 100.0).into();
    }

    pub fn render(&mut self) {
        self.ctx
            .upload_uniform(&self.camera_group, self.camera_block);

        let mut frame = self.ctx.begin_frame();

        {
            let mut pass = frame.begin_pass([0.05, 0.05, 0.05, 1.0], &self.msaa);
            pass.set_camera_group(&self.camera_group);

            pass.begin_lines(&self.line_pipeline);
            pass.draw_lines(&self.grid);

            pass.begin_brush_meshes(&self.brush_pipeline);
            for brush_mesh in &self.brush_meshes {
                pass.draw_brush_mesh(&brush_mesh.vertices, &brush_mesh.triangles);
            }
        }

        self.ctx.end_frame(frame);
        self.brush_meshes.clear();
    }
}

impl GraphicsWorld for Renderer {
    fn update_camera_view(&mut self, view: Matrix4<f32>) {
        if let Some(view) = view.invert() {
            self.camera_block.view = view.into();
        }
    }

    fn update_grid(&mut self, cell_count: i32, cell_size: f32) {
        let half_line_len = cell_count as f32 * cell_size;
        let gray = [0.1, 0.1, 0.1, 1.0];
        let red = [0.4, 0.1, 0.1, 1.0];
        let blue = [0.1, 0.1, 0.4, 1.0];

        let mut vertices = Vec::with_capacity(cell_count as usize * 8 + 4);

        vertices.push(LineVertex {
            position: [-half_line_len, 0.0, 0.0],
            color: red,
        });

        vertices.push(LineVertex {
            position: [half_line_len, 0.0, 0.0],
            color: red,
        });

        vertices.push(LineVertex {
            position: [0.0, 0.0, -half_line_len],
            color: blue,
        });

        vertices.push(LineVertex {
            position: [0.0, 0.0, half_line_len],
            color: blue,
        });

        for sign in [-1.0, 1.0] {
            for i in 1..=cell_count {
                let pos = i as f32 * cell_size * sign;

                vertices.push(LineVertex {
                    position: [-half_line_len, 0.0, pos],
                    color: gray,
                });

                vertices.push(LineVertex {
                    position: [half_line_len, 0.0, pos],
                    color: gray,
                });

                vertices.push(LineVertex {
                    position: [pos, 0.0, -half_line_len],
                    color: gray,
                });

                vertices.push(LineVertex {
                    position: [pos, 0.0, half_line_len],
                    color: gray,
                });
            }
        }

        self.grid = self
            .ctx
            .create_buffer(&vertices, wgpu::BufferUsages::VERTEX);
    }

    fn create_brush_mesh(&self, vertices: &[BrushVertex], triangles: &[Triangle]) -> Rc<BrushMesh> {
        Rc::new(BrushMesh {
            vertices: self.ctx.create_buffer(vertices, wgpu::BufferUsages::VERTEX),
            triangles: self.ctx.create_buffer(triangles, wgpu::BufferUsages::INDEX),
        })
    }

    fn draw_brush_mesh(&mut self, brush_mesh: Rc<BrushMesh>) {
        self.brush_meshes.push(brush_mesh);
    }
}

pub struct BrushMesh {
    vertices: TypedBuffer<BrushVertex>,
    triangles: TypedBuffer<Triangle>,
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
