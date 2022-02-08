use bytemuck::cast_slice;
use gpu::{BufferUsages, DepthBuffer, Gpu, Pipeline, Res, Surface, Uniform};
use winit::window::Window;

use super::{line, Camera, Canvas};

pub struct Renderer {
    gpu: Gpu,
    surface: Surface,
    depth_buffer: DepthBuffer,
    pipelines: Pipelines,
    camera: Res<Uniform<Camera>>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let (width, height) = window.inner_size().into();

        let (gpu, surface) = gpu::init(window);
        let depth_buffer = gpu.create_depth_buffer(width, height);
        let pipelines = Pipelines::new(&gpu, &surface);
        let camera = gpu.create_uniform(&Camera::default());

        surface.configure(&gpu, width, height);

        Self {
            gpu,
            surface,
            depth_buffer,
            pipelines,
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
        }

        self.gpu.end_frame(frame);
    }
}

// create_*_object implementations
impl Renderer {
    pub fn create_line_object(&self, mesh: line::Mesh) -> line::Object {
        line::Object {
            vertices: self
                .gpu
                .create_buffer(cast_slice(&mesh.vertices), BufferUsages::VERTEX),
        }
    }
}

struct Pipelines {
    line: Pipeline,
}

impl Pipelines {
    fn new(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            line: line::pipeline(gpu, surface),
        }
    }
}
