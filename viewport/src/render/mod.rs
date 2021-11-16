mod gpu;

pub mod data;

use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use cgmath::{perspective, vec2, vec3, Deg, Matrix4, SquareMatrix, Vector2, Vector3, Vector4};
use image::DynamicImage;
use winit::window::Window;

use crate::math::Ray;

use self::{
    data::{BrushVertex, LineVertex, Triangle},
    gpu::{
        BrushPipeline, Context, DepthBuffer, LinePipeline, MsaaFramebuffer, TextureGroup,
        TextureLayout, TypedBuffer, UniformBufferGroup, UniformBufferLayout,
    },
};

const MSAA_SAMPLE_COUNT: u32 = 4;

pub trait GraphicsWorld {
    fn create_brush_mesh(&self, vertices: &[BrushVertex], triangles: &[Triangle]) -> Rc<BrushMesh>;
    fn create_transform(&self, matrix: Matrix4<f32>) -> Rc<Transform>;
    fn create_texture(&self, image: &DynamicImage) -> Rc<Texture>;
    fn create_brush_detail(&self) -> Rc<BrushDetail>;

    fn update_camera_view(&mut self, view: Matrix4<f32>);
    fn update_grid(&mut self, cell_count: i32, cell_size: f32);
    fn update_transform(&self, transform: &Transform, matrix: Matrix4<f32>);
    fn update_brush_detail(&self, brush_detail: &BrushDetail, highlight: [f32; 4]);

    fn draw_brush(&mut self, command: BrushCommand);

    fn screen_ray(&self, coords: Vector2<f32>) -> Ray;
    fn world_to_screen(&self, point: Vector3<f32>) -> Option<Vector2<f32>>;
}

pub struct Renderer {
    ctx: Context,
    msaa: MsaaFramebuffer,
    depth: DepthBuffer,
    sampler: wgpu::Sampler,
    viewport_size: Vector2<f32>,

    uniform_buffer_layout: UniformBufferLayout,
    texture_layout: TextureLayout,

    line_pipeline: LinePipeline,
    brush_pipeline: BrushPipeline,

    camera_group: UniformBufferGroup<CameraBlock>,
    camera_block: CameraBlock,

    grid: TypedBuffer<LineVertex>,
    brush_commands: Vec<BrushCommand>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let ctx = Context::new(window);

        let (msaa, depth, viewport_size) = {
            let (width, height) = window.inner_size().into();
            (
                ctx.create_msaa_framebuffer(width, height),
                ctx.create_depth_buffer(width, height),
                vec2(width as f32, height as f32),
            )
        };

        let sampler = ctx.create_sampler();

        let uniform_buffer_layout = ctx.create_uniform_buffer_layout();
        let texture_layout = ctx.create_texture_layout();

        let line_pipeline = ctx.create_line_pipeline(&uniform_buffer_layout);
        let brush_pipeline = ctx.create_brush_pipeline(&uniform_buffer_layout, &texture_layout);

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
            depth,
            sampler,
            viewport_size,
            uniform_buffer_layout,
            texture_layout,
            line_pipeline,
            brush_pipeline,
            camera_group,
            camera_block,
            grid,
            brush_commands: brush_meshes,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.configure(width, height);
        self.msaa = self.ctx.create_msaa_framebuffer(width, height);
        self.depth = self.ctx.create_depth_buffer(width, height);
        self.viewport_size = vec2(width as f32, height as f32);

        let aspect = width as f32 / height as f32;
        self.camera_block.projection = perspective(Deg(80.0), aspect, 0.1, 100.0).into();
    }

    pub fn render(&mut self) {
        self.ctx
            .upload_uniform(&self.camera_group, self.camera_block);

        let mut frame = self.ctx.begin_frame();

        {
            let mut pass = frame.begin_pass([0.05, 0.05, 0.05, 1.0], &self.msaa, &self.depth);
            pass.set_ubg(0, &self.camera_group);

            pass.begin_lines(&self.line_pipeline);
            pass.draw_lines(&self.grid);

            pass.begin_brushes(&self.brush_pipeline);
            for command in &self.brush_commands {
                pass.set_ubg(1, &command.transform.group);
                for component in &command.components {
                    pass.set_ubg(2, &component.detail.group);
                    pass.set_texture(&component.texture.group);
                    pass.draw_mesh(&component.mesh.vertices, &component.mesh.triangles);
                }
            }
        }

        self.ctx.end_frame(frame);
        self.brush_commands.clear();
    }
}

impl GraphicsWorld for Renderer {
    fn create_brush_mesh(&self, vertices: &[BrushVertex], triangles: &[Triangle]) -> Rc<BrushMesh> {
        Rc::new(BrushMesh {
            vertices: self.ctx.create_buffer(vertices, wgpu::BufferUsages::VERTEX),
            triangles: self.ctx.create_buffer(triangles, wgpu::BufferUsages::INDEX),
        })
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

    fn create_texture(&self, image: &DynamicImage) -> Rc<Texture> {
        Rc::new(Texture {
            group: self
                .ctx
                .create_texture_group(&self.texture_layout, image, &self.sampler),
        })
    }

    fn create_brush_detail(&self) -> Rc<BrushDetail> {
        Rc::new(BrushDetail {
            group: self.ctx.create_uniform_buffer_group(
                &self.uniform_buffer_layout,
                BrushDetailBlock {
                    highlight: [1.0; 4],
                },
            ),
        })
    }

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

    fn update_transform(&self, transform: &Transform, matrix: Matrix4<f32>) {
        self.ctx.upload_uniform(
            &transform.group,
            TransformBlock {
                matrix: matrix.into(),
            },
        );
    }

    fn update_brush_detail(&self, brush_detail: &BrushDetail, highlight: [f32; 4]) {
        self.ctx
            .upload_uniform(&brush_detail.group, BrushDetailBlock { highlight })
    }

    fn draw_brush(&mut self, command: BrushCommand) {
        self.brush_commands.push(command);
    }

    fn screen_ray(&self, coords: Vector2<f32>) -> Ray {
        let coords = (vec2(
            coords.x / self.viewport_size.x,
            1.0 - coords.y / self.viewport_size.y,
        ) - vec2(0.5, 0.5))
            * 2.0;

        let unproject = {
            let view: Matrix4<f32> = self.camera_block.view.into();
            let projection: Matrix4<f32> = self.camera_block.projection.into();
            view.invert().unwrap() * projection.invert().unwrap()
        };

        let a = unproject * Vector4::new(coords.x, coords.y, 0.0, 1.0);
        let b = unproject * Vector4::new(coords.x, coords.y, 1.0, 1.0);

        let a = vec3(a.x / a.w, a.y / a.w, a.z / a.w);
        let b = vec3(b.x / b.w, b.y / b.w, b.z / b.w);

        Ray { origin: a, end: b }
    }

    fn world_to_screen(&self, point: Vector3<f32>) -> Option<Vector2<f32>> {
        let view: Matrix4<f32> = self.camera_block.view.into();
        let projection: Matrix4<f32> = self.camera_block.projection.into();

        let clip = projection * view * Vector4::new(point.x, point.y, point.z, 1.0);
        let clip = vec2(clip.x / clip.w, clip.y / clip.w);

        if (-1.0..=1.0).contains(&clip.x) && (-1.0..=1.0).contains(&clip.y) {
            let moved = vec2(clip.x + 1.0, 2.0 - (clip.y + 1.0)) * 0.5;
            Some(vec2(
                moved.x * self.viewport_size.x,
                moved.y * self.viewport_size.y,
            ))
        } else {
            None
        }
    }
}

pub struct BrushCommand {
    pub transform: Rc<Transform>,
    pub components: Vec<BrushComponent>,
}

pub struct BrushComponent {
    pub mesh: Rc<BrushMesh>,
    pub texture: Rc<Texture>,
    pub detail: Rc<BrushDetail>,
}

pub struct BrushMesh {
    vertices: TypedBuffer<BrushVertex>,
    triangles: TypedBuffer<Triangle>,
}

pub struct Transform {
    group: UniformBufferGroup<TransformBlock>,
}

pub struct Texture {
    group: TextureGroup,
}

pub struct BrushDetail {
    group: UniformBufferGroup<BrushDetailBlock>,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct BrushDetailBlock {
    highlight: [f32; 4],
}
