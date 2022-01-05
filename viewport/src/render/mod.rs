mod gpu;

use std::{collections::HashMap, rc::Rc};

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix, Vector2, Vector3};
use image::{DynamicImage, GenericImageView};
use wgpu::{BufferUsages, Sampler};
use winit::window::Window;

use crate::ring_vec::RingVec;

use self::gpu::{
    Context, DepthBuffer, LinePipeline, MsaaFramebuffer, PropPipeline, SolidPipeline,
    SpritePipeline, TextureGroup, TextureLayout, TypedBuffer, UniformBufferGroup,
    UniformBufferLayout,
};

pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type TexCoord = [f32; 2];
pub type Color = [f32; 4];
pub type Triangle = [u16; 3];
pub type TextureID = u32;
pub type PropID = u32;

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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpriteVertex {
    pub position: Position,
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
            textures: RingVec::new(64),
            partial: Some(Vec::new()),
        }
    }

    pub fn create_prop_bank(&self) -> PropBank {
        PropBank {
            ctx: self.ctx.clone(),
            props: RingVec::new(64),
            partial: Some(Vec::new()),
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
            layout: self.uniform_buffer_layout.clone(),
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
            prop_pipeline: self
                .ctx
                .create_prop_pipeline(&self.uniform_buffer_layout, &self.texture_layout),
            line_pipeline: self.ctx.create_line_pipeline(&self.uniform_buffer_layout),
            sprite_pipeline: self
                .ctx
                .create_sprite_pipeline(&self.uniform_buffer_layout, &self.texture_layout),
            world_camera_group: self
                .ctx
                .create_uniform_buffer_group(&self.uniform_buffer_layout, [[0.0; 4]; 4]),
            sprite_camera_group: self
                .ctx
                .create_uniform_buffer_group(&self.uniform_buffer_layout, [[0.0; 4]; 4]),
        }
    }
}

pub struct TextureBank {
    ctx: Rc<Context>,
    layout: Rc<TextureLayout>,
    sampler: Rc<Sampler>,
    textures: RingVec<TextureData>,
    partial: Option<Vec<(TextureID, Vec<u8>)>>,
}

struct TextureData {
    group: TextureGroup,
    size: Vector2<u32>,
}

impl TextureBank {
    pub fn insert_data(&mut self, id: TextureID, data: Vec<u8>) {
        self.partial.as_mut().unwrap().push((id, data));
    }

    pub fn finish(&mut self) {
        for (id, data) in self.partial.take().unwrap() {
            self.insert(id, &image::load_from_memory(&data).unwrap());
        }
    }

    pub fn exists(&self, id: TextureID) -> bool {
        self.textures.has_element_at(id as usize)
    }

    pub fn size_of(&self, id: TextureID) -> Option<Vector2<u32>> {
        self.textures.get(id as usize).map(|t| t.size)
    }

    fn insert(&mut self, id: TextureID, image: &DynamicImage) {
        let size = image.dimensions();
        self.textures.insert(
            id as usize,
            TextureData {
                group: self
                    .ctx
                    .create_texture_group(&self.layout, image, &self.sampler),
                size: size.into(),
            },
        );
    }
}

pub struct PropBank {
    ctx: Rc<Context>,
    props: RingVec<Prop>,
    partial: Option<Vec<(PropID, Vec<u8>)>>,
}

pub struct Prop {
    pub texture_id: TextureID,
    pub solid_batch: Rc<SolidBatch>,
}

impl PropBank {
    pub fn insert_data(&mut self, id: PropID, data: Vec<u8>) {
        self.partial.as_mut().unwrap().push((id, data));
    }

    pub fn finish(&mut self, solid_factory: &SolidFactory) {
        for (id, data) in self.partial.take().unwrap() {
            let mesh = mdl::Mesh::decode(&data).unwrap();
            self.props.insert(
                id as usize,
                Prop {
                    texture_id: mesh.texture.0,
                    solid_batch: solid_factory.from_mesh(&mesh),
                },
            );
        }
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
    layout: Rc<UniformBufferLayout>,
}

impl SolidFactory {
    pub fn create(&self, vertices: &[SolidVertex], triangles: &[Triangle]) -> Rc<SolidBatch> {
        Rc::new(SolidBatch {
            vertices: self.ctx.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.ctx.create_buffer(triangles, BufferUsages::INDEX),
        })
    }

    pub fn create_prop(&self, texture_id: TextureID, mesh: &mdl::Mesh) -> Rc<Prop> {
        Rc::new(Prop {
            texture_id,
            solid_batch: self.from_mesh(mesh),
        })
    }

    pub fn from_mesh(&self, mdl: &mdl::Mesh) -> Rc<SolidBatch> {
        let vertices = mdl
            .vertices
            .iter()
            .map(|v| SolidVertex {
                position: [v.position.x, v.position.y, v.position.z],
                normal: [v.normal.x, v.normal.y, v.normal.z],
                texcoord: [v.texcoord.x, v.texcoord.y],
                color: [1.0; 4],
            })
            .collect::<Vec<_>>();

        let triangles = mdl.triangles.iter().map(|t| t.indices).collect::<Vec<_>>();

        Rc::new(SolidBatch {
            vertices: self.ctx.create_buffer(&vertices, BufferUsages::VERTEX),
            triangles: self.ctx.create_buffer(&triangles, BufferUsages::INDEX),
        })
    }

    pub fn create_transform(&self) -> Transform {
        Transform {
            matrix: Matrix4::identity(),
            group: Rc::new(
                self.ctx
                    .create_uniform_buffer_group(&self.layout, Matrix4::identity().into()),
            ),
        }
    }
}

pub struct SolidBatch {
    vertices: TypedBuffer<SolidVertex>,
    triangles: TypedBuffer<Triangle>,
}

#[derive(Clone)]
pub struct Transform {
    matrix: Matrix4<f32>,
    group: Rc<UniformBufferGroup<[[f32; 4]; 4]>>,
}

impl Transform {
    pub fn set(&mut self, matrix: Matrix4<f32>) {
        self.matrix = matrix;
    }
}

pub struct Scene<'a> {
    pub texture_bank: &'a TextureBank,
    pub prop_bank: &'a PropBank,
    pub world_pass: WorldPass,
    pub sprite_pass: SpritePass,
}

pub struct WorldPass {
    pub camera_matrix: Matrix4<f32>,
    pub solid_batches: Vec<(TextureID, Rc<SolidBatch>)>,
    pub props: Vec<(PropID, Vec<Transform>)>,
    pub line_batches: Vec<Rc<LineBatch>>,
}

pub struct SpritePass {
    pub camera_matrix: Matrix4<f32>,
    pub sprites: HashMap<TextureID, Vec<Sprite>>,
}

pub struct Sprite {
    pub origin: Vector3<f32>,
    pub extent: Vector2<f32>,
    pub color: Color,
}

pub struct SceneRenderer {
    ctx: Rc<Context>,
    depth_buffer: DepthBuffer,
    msaa_buffer: MsaaFramebuffer,
    solid_pipeline: SolidPipeline,
    prop_pipeline: PropPipeline,
    line_pipeline: LinePipeline,
    sprite_pipeline: SpritePipeline,
    world_camera_group: UniformBufferGroup<[[f32; 4]; 4]>,
    sprite_camera_group: UniformBufferGroup<[[f32; 4]; 4]>,
}

impl SceneRenderer {
    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.ctx.configure(width, height);
        self.depth_buffer = self.ctx.create_depth_buffer(width, height);
        self.msaa_buffer = self.ctx.create_msaa_framebuffer(width, height);
    }

    pub fn render(&self, scene: Scene) {
        let world_pass = scene.world_pass;
        let sprite_pass = scene.sprite_pass;
        let baked_sprites = {
            let mut map = HashMap::new();
            for (texture, sprites) in &sprite_pass.sprites {
                if scene
                    .texture_bank
                    .textures
                    .has_element_at(*texture as usize)
                {
                    map.insert(*texture, build_sprite_batch(&self.ctx, sprites));
                }
            }
            map
        };

        let mut frame = self.ctx.begin_frame();

        {
            let mut pass = frame.begin_pass(
                [0.05, 0.05, 0.05, 1.0],
                &self.msaa_buffer,
                &self.depth_buffer,
            );

            {
                self.ctx
                    .upload_uniform(&self.world_camera_group, world_pass.camera_matrix.into());
                pass.set_ubg(0, &self.world_camera_group);

                pass.begin_solids(&self.solid_pipeline);
                for (texture, batch) in &world_pass.solid_batches {
                    if let Some(texture) = scene.texture_bank.textures.get(*texture as usize) {
                        pass.set_texture(&texture.group);
                        pass.draw_mesh(&batch.vertices, &batch.triangles);
                    }
                }

                pass.begin_props(&self.prop_pipeline);
                for (prop, transforms) in &world_pass.props {
                    if let Some(prop) = scene.prop_bank.props.get(*prop as usize) {
                        if let Some(texture) =
                            scene.texture_bank.textures.get(prop.texture_id as usize)
                        {
                            pass.set_texture(&texture.group);
                            for transform in transforms {
                                self.ctx
                                    .upload_uniform(&transform.group, transform.matrix.into());
                                pass.set_ubg(2, &transform.group);
                                pass.draw_mesh(
                                    &prop.solid_batch.vertices,
                                    &prop.solid_batch.triangles,
                                );
                            }
                        }
                    }
                }

                pass.begin_lines(&self.line_pipeline);
                for batch in &world_pass.line_batches {
                    pass.draw_lines(&batch.vertices);
                }
            }

            {
                self.ctx
                    .upload_uniform(&self.sprite_camera_group, sprite_pass.camera_matrix.into());
                pass.set_ubg(0, &self.sprite_camera_group);

                pass.begin_sprites(&self.sprite_pipeline);
                for (texture, (vertices, triangles)) in &baked_sprites {
                    if let Some(texture) = scene.texture_bank.textures.get(*texture as usize) {
                        pass.set_texture(&texture.group);
                        pass.draw_mesh(vertices, triangles);
                    }
                }
            }
        }

        self.ctx.end_frame(frame);
    }
}

fn build_sprite_batch(
    ctx: &Context,
    sprites: &[Sprite],
) -> (TypedBuffer<SpriteVertex>, TypedBuffer<Triangle>) {
    let mut vertices = Vec::with_capacity(sprites.len() * 4);
    let mut triangles = Vec::with_capacity(sprites.len() * 2);

    for sprite in sprites {
        let t0 = vertices.len() as u16;
        triangles.push([t0, t0 + 1, t0 + 2]);
        triangles.push([t0, t0 + 2, t0 + 3]);

        vertices.push(SpriteVertex {
            position: [
                sprite.origin.x,
                sprite.origin.y + sprite.extent.y,
                sprite.origin.z,
            ],
            texcoord: [0.0, 1.0],
            color: sprite.color,
        });

        vertices.push(SpriteVertex {
            position: [
                sprite.origin.x + sprite.extent.x,
                sprite.origin.y + sprite.extent.y,
                sprite.origin.z,
            ],
            texcoord: [1.0, 1.0],
            color: sprite.color,
        });

        vertices.push(SpriteVertex {
            position: [
                sprite.origin.x + sprite.extent.x,
                sprite.origin.y,
                sprite.origin.z,
            ],
            texcoord: [1.0, 0.0],
            color: sprite.color,
        });

        vertices.push(SpriteVertex {
            position: sprite.origin.into(),
            texcoord: [0.0, 0.0],
            color: sprite.color,
        });
    }

    (
        ctx.create_buffer(&vertices, BufferUsages::VERTEX),
        ctx.create_buffer(&triangles, BufferUsages::INDEX),
    )
}
