mod gpu;

use std::{collections::HashMap, rc::Rc};

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector2};
use image::DynamicImage;
use wgpu::{BufferUsages, Sampler};
use winit::window::Window;

use self::gpu::{
    Context, DepthBuffer, LinePipeline, MsaaFramebuffer, SolidPipeline, TextureGroup,
    TextureLayout, TypedBuffer, UniformBufferLayout,
};

pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type TexCoord = [f32; 2];
pub type Color = [f32; 4];
pub type Triangle = [u16; 3];
pub type TextureID = u64;

const MSAA_SAMPLE_COUNT: i32 = 4;

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
    pub fn create_texture_bank(&self) -> Rc<TextureBank> {
        Rc::new(TextureBank {
            ctx: self.ctx.clone(),
            layout: self.texture_layout.clone(),
            sampler: self.sampler.clone(),
            textures: Default::default(),
        })
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
            sampler: self.sampler.clone(),
            depth_buffer: self.ctx.create_depth_buffer(self.width, self.height),
            msaa_buffer: self.ctx.create_msaa_framebuffer(self.width, self.height),
            solid_pipeline: self
                .ctx
                .create_solid_pipeline(&self.uniform_buffer_layout, &self.texture_layout),
            line_pipeline: self.ctx.create_line_pipeline(&self.uniform_buffer_layout),
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

pub struct Scene {
    pub texture_bank: Rc<TextureBank>,
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
    sampler: Rc<Sampler>,
    depth_buffer: DepthBuffer,
    msaa_buffer: MsaaFramebuffer,
    solid_pipeline: SolidPipeline,
    line_pipeline: LinePipeline,
}

impl SceneRenderer {
    pub fn render(&self, scene: Scene) {
        todo!()
    }
}
