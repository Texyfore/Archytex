use std::collections::HashMap;

use assets::TextureID;
use bytemuck::cast_slice;
use gpu::{BufferUsages, DepthBuffer, Gpu, Pipeline, Res, Surface, Uniform};
use winit::window::Window;

use super::{line, solid, Camera, Canvas};

pub struct Renderer {
    gpu: Gpu,
    surface: Surface,
    depth_buffer: DepthBuffer,
    pipelines: Pipelines,
    resources: Resources,
    camera: Res<Uniform<Camera>>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let (width, height) = window.inner_size().into();

        let (gpu, surface) = gpu::init(window);
        let depth_buffer = gpu.create_depth_buffer(width, height);
        let pipelines = Pipelines::new(&gpu, &surface);
        let resources = Resources::default();
        let camera = gpu.create_uniform(&Camera::default());

        surface.configure(&gpu, width, height);

        Self {
            gpu,
            surface,
            depth_buffer,
            pipelines,
            resources,
            camera,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.configure(&self.gpu, width, height);
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
    }

    pub fn render(&self, canvas: Canvas) {
        let mut frame = self.gpu.begin_frame(&self.surface);
        self.gpu.set_uniform(&self.camera, &canvas.camera);

        {
            let mut pass = frame.begin_pass(&self.depth_buffer, &[0.1; 3]);
            pass.set_uniform(0, &self.camera);

            pass.set_pipeline(&self.pipelines.line);
            for line in &canvas.lines {
                pass.draw(&line.vertices);
            }

            pass.set_pipeline(&self.pipelines.solid);
            for solid in &canvas.solids {}
        }

        self.gpu.end_frame(frame);
    }
}

// add_* implementations
impl Renderer {}

// create_*_object implementations
impl Renderer {
    pub fn create_line_object(&self, mesh: line::Mesh) -> line::Object {
        line::Object {
            vertices: self
                .gpu
                .create_buffer(cast_slice(&mesh.vertices), BufferUsages::VERTEX),
        }
    }

    pub fn create_solid_object(&self, mesh: solid::Mesh) -> solid::Object {
        solid::Object {
            texture: mesh.texture,
            vertices: self.gpu.create_buffer(&mesh.vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(&mesh.triangles, BufferUsages::INDEX),
        }
    }
}

struct Pipelines {
    line: Pipeline,
    solid: Pipeline,
}

impl Pipelines {
    fn new(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            line: line::pipeline(gpu, surface),
            solid: solid::pipeline(gpu, surface),
        }
    }
}

#[derive(Default)]
struct Resources {
    textures: HashMap<TextureID, Res<gpu::Texture>>,
}