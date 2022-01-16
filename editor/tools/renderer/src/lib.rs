pub mod data;
pub mod scene;

use std::collections::HashMap;

use gpu::{
    data::{
        DepthBuffer, TextureLayout, {Uniform, UniformLayout},
    },
    handle::GpuHandle,
    pipelines::{GizmoPipeline, LinePipeline, MeshPipeline},
    Sampler,
};
use image::{EncodableLayout, ImageError};
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;
use tk3d::{
    math::{perspective, Deg},
    TextureID,
};

use self::scene::Scene;

pub struct Renderer {
    gpu: GpuHandle,
    depth_buffer: DepthBuffer,

    uniform_layout: UniformLayout,
    texture_layout: TextureLayout,

    mesh_pipeline: MeshPipeline,
    line_pipeline: LinePipeline,
    gizmo_pipeline: GizmoPipeline,

    camera_uniform: Uniform<[[f32; 4]; 4]>,
    textures: HashMap<u32, Texture>,
    sampler: Sampler,
}

impl Renderer {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, NewError> {
        let gpu = GpuHandle::new(window_handle)?;
        let depth_buffer = gpu.create_depth_buffer(1024, 768);
        gpu.configure(1024, 768);

        let uniform_layout = gpu.create_uniform_layout();
        let texture_layout = gpu.create_texture_layout();

        let mesh_pipeline = gpu.create_mesh_pipeline(&uniform_layout, &texture_layout);
        let line_pipeline = gpu.create_line_pipeline(&uniform_layout);
        let gizmo_pipeline = gpu.create_gizmo_pipeline(&uniform_layout);

        let camera_uniform = gpu.create_uniform(&uniform_layout);
        let textures = HashMap::new();
        let sampler = gpu.create_sampler();

        Ok(Self {
            gpu,
            depth_buffer,
            uniform_layout,
            texture_layout,
            mesh_pipeline,
            line_pipeline,
            gizmo_pipeline,
            camera_uniform,
            textures,
            sampler,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.configure(width, height);
        self.gpu.set_uniform(
            &self.camera_uniform,
            &perspective(Deg(80.0), width as f32 / height as f32, 0.1, 100.0).into(),
        );
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
    }

    pub fn render(&self, scene: &mut Scene) -> Result<(), RenderError> {
        let mut frame = self.gpu.next_frame()?;

        {
            let mut pass = frame.begin_pass(&self.depth_buffer, [0.1; 3]);
            pass.set_uniform(0, &self.camera_uniform);

            pass.set_mesh_pipeline(&self.mesh_pipeline);
            for mesh_object in &scene.mesh_objects {
                if let Some(texture) = self.textures.get(&mesh_object.texture_id.0) {
                    pass.set_uniform(1, &mesh_object.transform.uniform);
                    pass.set_texture(&texture.inner);
                    pass.draw_indexed(&mesh_object.mesh.vertices, &mesh_object.mesh.triangles);
                }
            }

            pass.set_line_pipeline(&self.line_pipeline);
            for line_object in &scene.line_objects {
                pass.set_uniform(1, &line_object.transform.uniform);
                pass.draw(&line_object.lines.vertices);
            }

            pass.set_gizmo_pipeline(&self.gizmo_pipeline);
            for gizmo_object in &scene.gizmo_objects {
                pass.draw_gizmos(
                    &gizmo_object.mesh.vertices,
                    &gizmo_object.mesh.triangles,
                    &gizmo_object.instances.buffer,
                )
            }
        }

        frame.draw(&self.gpu);

        Ok(())
    }

    pub fn load_texture(&mut self, id: TextureID, buf: &[u8]) -> Result<(), LoadTextureError> {
        let data = image::load_from_memory(buf)?.into_rgba8();
        let (width, height) = data.dimensions();
        self.textures.insert(
            id.0,
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

struct Texture {
    inner: gpu::data::Texture,
    _width: u32,
    _height: u32,
}
