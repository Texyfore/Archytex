mod canvas;
mod pipelines;
mod renderer;

pub mod structures;

use std::rc::Rc;

use asset::TextureID;
use gpu::{BufferUsages, Gpu, Surface};
use winit::window::Window;

pub use canvas::{Canvas, LineMesh, SolidMesh};
pub use renderer::Renderer;

use self::structures::{LineVertex, SolidVertex};

pub fn init(window: &Window) -> (Renderer, Graphics) {
    let (gpu, surface) = gpu::init(window);
    let gpu = Rc::new(gpu);
    let surface = Rc::new(surface);

    (
        Renderer::new(gpu.clone(), surface.clone()),
        Graphics { gpu, surface },
    )
}

pub struct Graphics {
    gpu: Rc<Gpu>,
    surface: Rc<Surface>,
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

    pub fn create_solid_mesh(&self, descriptor: SolidMeshDescriptor) -> SolidMesh {
        SolidMesh {
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
}

pub struct LineMeshDescriptor<'v> {
    pub vertices: &'v [LineVertex],
}

pub struct SolidMeshDescriptor<'v, 't> {
    pub texture: TextureID,
    pub vertices: &'v [SolidVertex],
    pub triangles: &'t [[u16; 3]],
}

pub trait Share {
    fn share(&self) -> Self;
}
