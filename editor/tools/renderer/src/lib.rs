pub mod data;
pub mod scene;

use gpu::{
    data::{
        texture::TextureLayout,
        uniform::{Uniform, UniformLayout},
    },
    handle::GpuHandle,
    pipelines::mesh::MeshPipeline,
};
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;
use tk3d::math::{perspective, Deg};

use self::scene::Scene;

pub struct Renderer {
    gpu: GpuHandle,
    uniform_layout: UniformLayout,
    texture_layout: TextureLayout,
    mesh_pipeline: MeshPipeline,
    camera_uniform: Uniform<[[f32; 4]; 4]>,
}

impl Renderer {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, NewError> {
        let gpu = GpuHandle::new(window_handle)?;
        gpu.configure(1024, 768);

        let uniform_layout = gpu.create_uniform_layout();
        let texture_layout = gpu.create_texture_layout();
        let mesh_pipeline = gpu.create_mesh_pipeline(&uniform_layout, &texture_layout);
        let camera_uniform = gpu.create_uniform(&uniform_layout);

        Ok(Self {
            gpu,
            uniform_layout,
            texture_layout,
            mesh_pipeline,
            camera_uniform,
        })
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.gpu.configure(width, height);
        self.gpu.set_uniform(
            &self.camera_uniform,
            &perspective(Deg(80.0), width as f32 / height as f32, 0.1, 100.0).into(),
        );
    }

    pub fn render(&self, scene: &mut Scene) -> Result<(), RenderError> {
        let mut frame = self.gpu.next_frame()?;

        {
            let mut pass = frame.begin_pass([0.1; 3]);
            pass.set_mesh_pipeline(&self.mesh_pipeline);
            pass.set_uniform(0, &self.camera_uniform);

            for mesh_object in &scene.mesh_objects {
                pass.set_uniform(1, &mesh_object.transform.uniform);
                pass.draw_mesh(&mesh_object.mesh.vertices, &mesh_object.mesh.triangles);
            }
        }

        frame.draw(&self.gpu);

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
    #[error("Couldn't render frame: {0}")]
    NoNextFrame(#[from] gpu::frame::NextFrameError),
}
