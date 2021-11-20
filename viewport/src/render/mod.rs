mod gpu;

use std::{collections::HashMap, rc::Rc};

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector2};
use image::DynamicImage;

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

pub struct Init;

impl Init {
    pub fn create_texture_bank(&self) -> Rc<TextureBank> {
        todo!()
    }

    pub fn create_line_factory(&self) -> LineFactory {
        todo!()
    }

    pub fn create_solid_factory(&self) -> SolidFactory {
        todo!()
    }

    pub fn create_scene_renderer(&self) -> SceneRenderer {
        todo!()
    }
}

pub struct TextureBank;

impl TextureBank {
    pub fn insert(&mut self, id: TextureID, image: &DynamicImage) {
        todo!()
    }
}

pub struct LineFactory;

impl LineFactory {
    pub fn create(&self, vertices: &[LineVertex]) -> Rc<LineBatch> {
        todo!()
    }
}

pub struct LineBatch;

pub struct SolidFactory;

impl SolidFactory {
    pub fn create(&self, vertices: &[SolidVertex], triangles: &[Triangle]) -> Rc<SolidBatch> {
        todo!()
    }
}

pub struct SolidBatch;

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

pub struct SceneRenderer;

impl SceneRenderer {
    pub fn render(&self, scene: Scene) {
        todo!()
    }
}
