pub mod data;
pub mod scene;

use std::{collections::HashMap, rc::Rc};

use asset_id::{GizmoID, PropID, TextureID};
use bytemuck::{Pod, Zeroable};
use formats::agzm;
use gpu::{
    data::{
        DepthBuffer, TextureLayout, {Uniform, UniformLayout},
    },
    handle::GpuHandle,
    pipelines::{GizmoPipeline, GridPipeline, LinePipeline, PropPipeline, SolidPipeline},
    BufferUsages, Sampler,
};
use image::{EncodableLayout, ImageError};
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;

use self::{
    data::{gizmo, prop},
    scene::{Scene, PropObject},
};

pub struct Renderer {
    gpu: GpuHandle,
    depth_buffer: DepthBuffer,

    uniform_layout: UniformLayout,
    texture_layout: TextureLayout,

    solid_pipeline: SolidPipeline,
    prop_pipeline: PropPipeline,
    line_pipeline: LinePipeline,
    grid_pipeline: GridPipeline,
    gizmo_pipeline: GizmoPipeline,

    camera_uniform: Uniform<CameraBlock>,
    sampler: Sampler,

    textures: HashMap<TextureID, Texture>,
    props: HashMap<PropID, Prop>,
    gizmos: HashMap<GizmoID, Rc<gizmo::Mesh>>,
}

impl Renderer {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, NewError> {
        let gpu = GpuHandle::new(window_handle)?;
        let depth_buffer = gpu.create_depth_buffer(1024, 768);

        let uniform_layout = gpu.create_uniform_layout();
        let texture_layout = gpu.create_texture_layout();

        let solid_pipeline = gpu.create_solid_pipeline(&uniform_layout, &texture_layout);
        let prop_pipeline = gpu.create_prop_pipeline(&uniform_layout, &texture_layout);
        let line_pipeline = gpu.create_line_pipeline(&uniform_layout);
        let grid_pipeline = gpu.create_grid_pipeline(&uniform_layout);
        let gizmo_pipeline = gpu.create_gizmo_pipeline(&uniform_layout);

        let camera_uniform = gpu.create_uniform(&uniform_layout);
        let sampler = gpu.create_sampler();

        let textures = HashMap::new();
        let props = HashMap::new();
        let gizmos = HashMap::new();

        Ok(Self {
            gpu,
            depth_buffer,
            uniform_layout,
            texture_layout,
            solid_pipeline,
            prop_pipeline,
            line_pipeline,
            grid_pipeline,
            gizmo_pipeline,
            camera_uniform,
            sampler,
            textures,
            props,
            gizmos,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.configure(width, height);
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
    }

    pub fn render(&self, scene: &Scene) -> Result<(), RenderError> {
        let mut frame = self.gpu.next_frame()?;

        {
            self.gpu.set_uniform(
                &self.camera_uniform,
                &CameraBlock {
                    world: scene.camera_world,
                    clip: scene.camera_clip,
                },
            );

            let mut pass = frame.begin_pass(&self.depth_buffer, [0.1; 3]);
            pass.set_uniform(0, &self.camera_uniform);

            pass.set_solid_pipeline(&self.solid_pipeline);
            for mesh_object in &scene.solid_objects {
                if let Some(texture) = self.textures.get(&mesh_object.texture) {
                    pass.set_uniform(1, &mesh_object.transform.uniform);
                    pass.set_texture(&texture.inner);
                    pass.draw_indexed(&mesh_object.mesh.vertices, &mesh_object.mesh.triangles);
                }
            }

            pass.set_prop_pipeline(&self.prop_pipeline);
            for prop_object in &scene.prop_objects {
                if let Some(prop) = self.props.get(&prop_object.prop) {
                    pass.set_uniform(1 , &prop_object.transform.uniform);
                    for mesh in &prop.meshes {
                        if let Some(texture) = self.textures.get(&mesh.texture) {
                        }
                    }
                }
            }

            pass.set_line_pipeline(&self.line_pipeline);
            for line_object in &scene.line_objects {
                pass.set_uniform(1, &line_object.transform.uniform);
                pass.draw(&line_object.lines.vertices);
            }

            pass.set_grid_pipeline(&self.grid_pipeline);
            for grid_object in &scene.grid_objects {
                pass.set_uniform(1, &grid_object.info.uniform);
                pass.draw(&grid_object.lines.vertices);
            }

            pass.set_gizmo_pipeline(&self.gizmo_pipeline);
            for gizmo_object in &scene.gizmo_objects {
                if let Some(mesh) = self.gizmos.get(&gizmo_object.id) {
                    pass.draw_gizmos(
                        &mesh.vertices,
                        &mesh.triangles,
                        &gizmo_object.instances.buffer,
                    )
                }
            }
        }

        frame.draw(&self.gpu);

        Ok(())
    }

    pub fn load_texture(&mut self, id: TextureID, buf: &[u8]) -> Result<(), LoadTextureError> {
        let data = image::load_from_memory(buf)?.into_rgba8();
        let (width, height) = data.dimensions();
        self.textures.insert(
            id,
            Texture {
                inner: self.gpu.create_texture(
                    &self.texture_layout,
                    &self.sampler,
                    width,
                    height,
                    data.as_bytes(),
                ),
                _width: width,
                _height: height,
            },
        );
        Ok(())
    }

    pub fn load_gizmo(&mut self, id: GizmoID, buf: &[u8]) -> Result<(), LoadGizmoError> {
        let gizmo = agzm::Mesh::decode(buf)?;

        let vertices = gizmo
            .vertices
            .iter()
            .map(|vertex| gizmo::Vertex {
                position: vertex.position,
            })
            .collect::<Vec<_>>();

        self.gizmos.insert(
            id,
            Rc::new(gizmo::Mesh {
                vertices: self.gpu.create_buffer(&vertices, BufferUsages::VERTEX),
                triangles: self
                    .gpu
                    .create_buffer(&gizmo.triangles, BufferUsages::INDEX),
            }),
        );

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Couldn't create renderer: {0}")]
    GpuError(#[from] gpu::handle::NewError),
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Couldn't render next frame: {0}")]
    NoNextFrame(#[from] gpu::frame::NextFrameError),
}

#[derive(Error, Debug)]
pub enum LoadTextureError {
    #[error("Couldn't load texture: {0}")]
    BadBuffer(#[from] ImageError),
}

#[derive(Error, Debug)]
pub enum LoadGizmoError {
    #[error("Couldn't load gizmo: {0}")]
    BadBuffer(#[from] agzm::DecodeError),
}

struct Texture {
    inner: gpu::data::Texture,
    _width: u32,
    _height: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CameraBlock {
    world: [[f32; 4]; 4],
    clip: [[f32; 4]; 4],
}

struct Prop {
    meshes: Vec<PropMesh>,
}

struct PropMesh {
    texture: TextureID,
    mesh: prop::Mesh,
}
