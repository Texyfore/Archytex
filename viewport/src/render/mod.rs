mod gpu;

use std::{collections::HashMap, rc::Rc};

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector2};
use image::DynamicImage;
use wgpu::{BufferUsages, Sampler};
use winit::window::Window;

use self::gpu::{
    Context, DepthBuffer, LinePipeline, MsaaFramebuffer, SolidPipeline, TextureGroup,
    TextureLayout, TypedBuffer, UniformBufferGroup, UniformBufferLayout,
};

pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type TexCoord = [f32; 2];
pub type Color = [f32; 4];
pub type Triangle = [u16; 3];
pub type TextureID = u64;

const MSAA_SAMPLE_COUNT: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LineVertex {
    pub position: Position,
    pub color: Color,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SolidVertex {
    pub position: Position,
    pub normal: Normal,
    pub texcoord: TexCoord,
    pub color: Color,
}

pub fn init(window: &Window) -> Init {
    let ctx = Context::new(window);
    let uniform_buffer_layout = ctx.create_uniform_buffer_layout();
    let texture_layout = ctx.create_texture_layout();
    let sampler = ctx.create_sampler();
    let (width, height) = window.inner_size().into();

    Init {
        ctx,
        uniform_buffer_layout,
        texture_layout,
        sampler,
        width,
        height,
    }
}

pub struct Init {
    ctx: Rc<Context>,
    uniform_buffer_layout: Rc<UniformBufferLayout>,
    texture_layout: Rc<TextureLayout>,
    sampler: Rc<Sampler>,
    width: u32,
    height: u32,
}

impl Init {
    pub fn create_texture_bank(&self) -> TextureBank {
        TextureBank {
            ctx: self.ctx.clone(),
            layout: self.texture_layout.clone(),
            sampler: self.sampler.clone(),
            textures: Default::default(),
        }
    }

    pub fn create_line_factory(&self) -> LineFactory {
        LineFactory {
            ctx: self.ctx.clone(),
        }
    }

    pub fn create_solid_factory(&self) -> SolidFactory {
        SolidFactory {
            ctx: self.ctx.clone(),
        }
    }

    pub fn create_scene_renderer(&self) -> SceneRenderer {
        SceneRenderer {
            ctx: self.ctx.clone(),
            depth_buffer: self.ctx.create_depth_buffer(self.width, self.height),
            msaa_buffer: self.ctx.create_msaa_framebuffer(self.width, self.height),
            solid_pipeline: self
                .ctx
                .create_solid_pipeline(&self.uniform_buffer_layout, &self.texture_layout),
            line_pipeline: self.ctx.create_line_pipeline(&self.uniform_buffer_layout),
            world_camera_group: self
                .ctx
                .create_uniform_buffer_group(&self.uniform_buffer_layout, [[0.0; 4]; 4]),
        }
    }
}

pub struct TextureBank {
    ctx: Rc<Context>,
    layout: Rc<TextureLayout>,
    sampler: Rc<Sampler>,
    textures: HashMap<TextureID, TextureGroup>,
}

impl TextureBank {
    pub fn insert(&mut self, id: TextureID, image: &DynamicImage) {
        self.textures.insert(
            id,
            self.ctx
                .create_texture_group(&self.layout, image, &self.sampler),
        );
    }
}

pub struct LineFactory {
    ctx: Rc<Context>,
}

impl LineFactory {
    pub fn create(&self, vertices: &[LineVertex]) -> Rc<LineBatch> {
        Rc::new(LineBatch {
            vertices: self.ctx.create_buffer(vertices, BufferUsages::VERTEX),
        })
    }
}

pub struct LineBatch {
    vertices: TypedBuffer<LineVertex>,
}

pub struct SolidFactory {
    ctx: Rc<Context>,
}

impl SolidFactory {
    pub fn create(&self, vertices: &[SolidVertex], triangles: &[Triangle]) -> Rc<SolidBatch> {
        Rc::new(SolidBatch {
            vertices: self.ctx.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.ctx.create_buffer(triangles, BufferUsages::INDEX),
        })
    }
}

pub struct SolidBatch {
    vertices: TypedBuffer<SolidVertex>,
    triangles: TypedBuffer<Triangle>,
}

pub struct Scene<'a> {
    pub texture_bank: &'a TextureBank,
    pub world_pass: WorldPass,
    pub sprite_pass: SpritePass,
}

pub struct WorldPass {
    pub camera_matrix: Matrix4<f32>,
    pub solid_batches: HashMap<TextureID, Vec<Rc<SolidBatch>>>,
    pub line_batches: Vec<Rc<LineBatch>>,
}

pub struct SpritePass {
    pub camera_matrix: Matrix4<f32>,
    pub sprites: HashMap<TextureID, Vec<Vector2<f32>>>,
}

pub struct SceneRenderer {
    ctx: Rc<Context>,
    depth_buffer: DepthBuffer,
    msaa_buffer: MsaaFramebuffer,
    solid_pipeline: SolidPipeline,
    line_pipeline: LinePipeline,
    world_camera_group: UniformBufferGroup<[[f32; 4]; 4]>,
}

impl SceneRenderer {
    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.ctx.configure(width, height);
        self.depth_buffer = self.ctx.create_depth_buffer(width, height);
        self.msaa_buffer = self.ctx.create_msaa_framebuffer(width, height);
    }

    pub fn render(&self, scene: Scene) {
        let world = scene.world_pass;

        let mut frame = self.ctx.begin_frame();

        {
            let mut pass = frame.begin_pass(
                [0.05, 0.05, 0.05, 1.0],
                &self.msaa_buffer,
                &self.depth_buffer,
            );

            {
                self.ctx
                    .upload_uniform(&self.world_camera_group, world.camera_matrix.into());
                pass.set_ubg(0, &self.world_camera_group);

                pass.begin_solids(&self.solid_pipeline);
                for (texture, batches) in &world.solid_batches {
                    if let Some(texture) = scene.texture_bank.textures.get(&texture) {
                        pass.set_texture(texture);
                        for batch in batches {
                            pass.draw_mesh(&batch.vertices, &batch.triangles);
                        }
                    }
                }

                pass.begin_lines(&self.line_pipeline);
                for batch in &world.line_batches {
                    pass.draw_lines(&batch.vertices);
                }
            }
        }

        self.ctx.end_frame(frame);
    }
}
