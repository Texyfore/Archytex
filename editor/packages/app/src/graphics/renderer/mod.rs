mod geom;

use std::{collections::HashMap, rc::Rc};

use asset::{PropID, TextureID};
use bytemuck::cast_slice;
use gpu::{BufferUsages, DepthBuffer, Gpu, Image, Pipeline, Sampler, Surface, Uniform};
use winit::window::Window;

use self::geom::PropModel;

use super::{line, prop, solid, Camera, Canvas};

pub struct Renderer {
    gpu: Gpu,
    surface: Surface,
    depth_buffer: DepthBuffer,
    pipelines: Pipelines,
    resources: Resources,
    sampler: Sampler,
    camera: Uniform<Camera>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let (width, height) = window.inner_size().into();

        let (gpu, surface) = gpu::init(window);
        let depth_buffer = gpu.create_depth_buffer(width, height);
        let pipelines = Pipelines::new(&gpu, &surface);
        let resources = Resources::default();
        let sampler = gpu.create_sampler();
        let camera = gpu.create_uniform(&Camera::default());

        surface.configure(&gpu, width, height);

        Self {
            gpu,
            surface,
            depth_buffer,
            pipelines,
            resources,
            sampler,
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
            for solid in &canvas.solids {
                if let Some(texture) = self.resources.textures.get(&solid.texture) {
                    pass.set_texture(1, texture);
                    pass.draw_triangles(&solid.vertices, &solid.triangles);
                }
            }

            pass.set_pipeline(&self.pipelines.prop);
            for prop in &canvas.props {
                if let Some(model) = self.resources.props.get(&prop.prop) {
                    pass.set_uniform(1, &prop.uniform);
                    for mesh in &model.meshes {
                        if let Some(texture) = self.resources.textures.get(&mesh.texture) {
                            pass.set_texture(2, texture);
                            pass.draw_triangles(&mesh.vertices, &mesh.triangles);
                        }
                    }
                }
            }
        }

        self.gpu.end_frame(frame);
    }
}

// add_* implementations
impl Renderer {
    pub fn add_texture(&mut self, id: TextureID, texture: asset::Texture) {
        self.resources.textures.insert(
            id,
            self.gpu.create_texture(
                &self.sampler,
                Image {
                    width: texture.width,
                    height: texture.height,
                    buf: &texture.rgba8,
                },
            ),
        );
    }

    pub fn add_prop(&mut self, id: PropID, prop: asset::Prop) {
        self.resources
            .props
            .insert(id, PropModel::new(&self.gpu, prop));
    }
}

// create_* implementations
impl Renderer {
    pub fn create_line_object(&self, mesh: line::Mesh) -> line::Object {
        line::Object {
            vertices: Rc::new(
                self.gpu
                    .create_buffer(cast_slice(mesh.vertices), BufferUsages::VERTEX),
            ),
        }
    }

    pub fn create_solid_object(&self, mesh: solid::Mesh) -> solid::Object {
        solid::Object {
            texture: mesh.texture,
            vertices: Rc::new(self.gpu.create_buffer(mesh.vertices, BufferUsages::VERTEX)),
            triangles: Rc::new(self.gpu.create_buffer(mesh.triangles, BufferUsages::INDEX)),
        }
    }

    pub fn create_prop_object(&self, prop: PropID) -> prop::Object {
        prop::Object {
            prop,
            uniform: Rc::new(self.gpu.create_uniform(&prop::Properties::default())),
        }
    }
}

// set_* implementations
impl Renderer {
    pub fn set_prop_properties(&self, object: &prop::Object, properties: prop::Properties) {
        self.gpu.set_uniform(&object.uniform, &properties);
    }
}

struct Pipelines {
    line: Pipeline,
    solid: Pipeline,
    prop: Pipeline,
}

impl Pipelines {
    fn new(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            line: line::pipeline(gpu, surface),
            solid: solid::pipeline(gpu, surface),
            prop: prop::pipeline(gpu, surface),
        }
    }
}

#[derive(Default)]
struct Resources {
    textures: HashMap<TextureID, gpu::Texture>,
    props: HashMap<PropID, PropModel>,
}
