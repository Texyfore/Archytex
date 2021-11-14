mod gpu;

pub mod data;

use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use cgmath::{perspective, Deg, Matrix4, SquareMatrix};
use winit::window::Window;

use self::{
    data::{BrushVertex, LineVertex, Triangle},
    gpu::{
        BrushPipeline, Context, LinePipeline, MsaaFramebuffer, TypedBuffer, UniformBufferGroup,
        UniformBufferLayout,
    },
};

pub const MSAA_SAMPLE_COUNT: u32 = 4;

pub trait GraphicsWorld {
    fn update_camera_view(&mut self, view: Matrix4<f32>);
    fn update_grid(&mut self, cell_count: i32, cell_size: f32);
    fn create_brush_mesh(&self, vertices: &[BrushVertex], triangles: &[Triangle]) -> Rc<BrushMesh>;
    fn draw_brush_mesh(&mut self, brush_mesh: Rc<BrushMesh>, transform: Rc<Transform>);
    fn create_transform(&self, matrix: Matrix4<f32>) -> Rc<Transform>;
    fn update_transform(&self, transform: &Transform, matrix: Matrix4<f32>);
}

pub struct Renderer {
    ctx: Context,
    msaa: MsaaFramebuffer,
    uniform_buffer_layout: UniformBufferLayout,

    line_pipeline: LinePipeline,
    brush_pipeline: BrushPipeline,

    camera_group: UniformBufferGroup<CameraBlock>,
    camera_block: CameraBlock,

    grid: TypedBuffer<LineVertex>,
    brush_meshes: Vec<(Rc<BrushMesh>, Rc<Transform>)>,
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

        let camera_block = CameraBlock {
            view: Matrix4::identity().into(),
            projection: Matrix4::identity().into(),
        };

        let camera_group =
            ctx.create_uniform_buffer_group::<CameraBlock>(&uniform_buffer_layout, camera_block);

        let grid = ctx.create_buffer(&[], wgpu::BufferUsages::VERTEX);
        let brush_meshes = Default::default();

        Self {
            ctx,
            msaa,
            uniform_buffer_layout,
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
            for (brush_mesh, transform) in &self.brush_meshes {
                pass.draw_brush_mesh(
                    &brush_mesh.vertices,
                    &brush_mesh.triangles,
                    &transform.group,
                );
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

    fn draw_brush_mesh(&mut self, brush_mesh: Rc<BrushMesh>, transform: Rc<Transform>) {
        self.brush_meshes.push((brush_mesh, transform));
    }

    fn create_transform(&self, matrix: Matrix4<f32>) -> Rc<Transform> {
        Rc::new(Transform {
            group: self.ctx.create_uniform_buffer_group(
                &self.uniform_buffer_layout,
                TransformBlock {
                    matrix: matrix.into(),
                },
            ),
        })
    }

    fn update_transform(&self, transform: &Transform, matrix: Matrix4<f32>) {
        self.ctx.upload_uniform(
            &transform.group,
            TransformBlock {
                matrix: matrix.into(),
            },
        );
    }
}

pub struct BrushMesh {
    vertices: TypedBuffer<BrushVertex>,
    triangles: TypedBuffer<Triangle>,
}

pub struct Transform {
    group: UniformBufferGroup<TransformBlock>,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct TransformBlock {
    matrix: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CameraBlock {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}
