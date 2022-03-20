mod canvas;
mod loader;
mod renderer;

pub mod structures;

use std::{mem::size_of, rc::Rc};

use asset::TextureID;
use gpu::{BufferUsages, Gpu};
use winit::window::Window;

pub use canvas::*;
pub use loader::*;
pub use renderer::Renderer;

use loader::ResourceLoader;

use self::structures::{GizmoInstance, GroundVertex, LineVertex, SolidVertex, TransformTint};

pub fn init(window: &Window) -> (Renderer, Graphics, ResourceLoader) {
    let (gpu, surface) = gpu::init(window);
    let gpu = Rc::new(gpu);
    let surface = Rc::new(surface);
    let sampler = gpu.create_sampler();

    {
        let (width, height) = window.inner_size().into();
        surface.configure(&gpu, width, height);
    }

    (
        Renderer::new(gpu.clone(), surface),
        Graphics { gpu: gpu.clone() },
        ResourceLoader::new(gpu, sampler),
    )
}

pub struct Graphics {
    gpu: Rc<Gpu>,
}

impl Graphics {
    pub fn create_line_mesh(&self, descriptor: LineMeshDescriptor) -> LineMesh {
        LineMesh {
            vertices: Rc::new(
                self.gpu
                    .create_buffer(descriptor.vertices, BufferUsages::VERTEX),
            ),
        }
    }

    pub fn create_line_mesh_uninit(&self, num_points: usize) -> LineMesh {
        LineMesh {
            vertices: Rc::new(self.gpu.create_buffer_uninit(
                size_of::<LineVertex>() * num_points,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
            )),
        }
    }

    pub fn write_line_mesh(&self, mesh: &LineMesh, vertices: &[LineVertex]) {
        self.gpu.write_buffer(&mesh.vertices, vertices);
    }

    pub fn create_solid_mesh(&self) -> SolidMesh {
        SolidMesh {
            vertices: self.gpu.create_buffer_uninit(
                size_of::<SolidVertex>() * 24,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
            ),
        }
    }

    pub fn write_solid_mesh(&self, mesh: &SolidMesh, vertices: &[SolidVertex]) {
        self.gpu.write_buffer(&mesh.vertices, vertices);
    }

    pub fn create_ground_mesh(&self, descriptor: GroundMeshDescriptor) -> GroundMesh {
        GroundMesh {
            texture: descriptor.texture,
            vertices: Rc::new(
                self.gpu
                    .create_buffer(descriptor.vertices, BufferUsages::VERTEX),
            ),
            triangles: Rc::new(
                self.gpu
                    .create_buffer(descriptor.triangles, BufferUsages::INDEX),
            ),
        }
    }

    pub fn create_prop_data(&self, data: &TransformTint) -> PropData {
        PropData {
            uniform: Rc::new(self.gpu.create_uniform(data)),
        }
    }

    pub fn create_gizmo_instances(&self, len: usize) -> GizmoInstances {
        GizmoInstances {
            buffer: Rc::new(
                self.gpu
                    .create_buffer_uninit(len, BufferUsages::VERTEX | BufferUsages::COPY_DST),
            ),
            len: len as u32,
        }
    }

    pub fn write_gizmo_instances(&self, instances: &GizmoInstances, data: &[GizmoInstance]) {
        self.gpu.write_buffer(&instances.buffer, data);
    }
}

pub struct LineMeshDescriptor<'v> {
    pub vertices: &'v [LineVertex],
}

pub struct GroundMeshDescriptor<'v, 't> {
    pub texture: TextureID,
    pub vertices: &'v [GroundVertex],
    pub triangles: &'t [[u16; 3]],
}

pub trait Share {
    fn share(&self) -> Self;
}
