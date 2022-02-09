mod pipelines;

use std::{collections::HashMap, rc::Rc};

use asset::{PropID, PropMesh, TextureID};
use gpu::{DepthBuffer, Gpu, Image, Sampler, Surface, Uniform};

use self::pipelines::Pipelines;

use super::{structures::CameraMatrices, Canvas};

pub struct Renderer {
    gpu: Rc<Gpu>,
    surface: Rc<Surface>,
    depth_buffer: DepthBuffer,
    sampler: Sampler,
    pipelines: Pipelines,
    resources: Resources,
    camera: Uniform<CameraMatrices>,
}

impl Renderer {
    pub(super) fn new(gpu: Rc<Gpu>, surface: Rc<Surface>) -> Self {
        let depth_buffer = gpu.create_depth_buffer(800, 600);
        let sampler = gpu.create_sampler();
        let pipelines = Pipelines::new(&gpu, &surface);
        let resources = Resources::default();
        let camera = gpu.create_uniform(&CameraMatrices::default());

        Self {
            gpu,
            surface,
            depth_buffer,
            sampler,
            pipelines,
            resources,
            camera,
        }
    }

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
        self.resources.props.insert(id, prop.meshes);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.configure(&self.gpu, width, height);
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
    }

    pub fn render(&self, canvas: Canvas) {
        self.gpu.set_uniform(&self.camera, &canvas.camera_matrices);
        let mut frame = self.gpu.begin_frame(&self.surface);

        {
            let mut pass = frame.begin_pass(&self.depth_buffer, &[0.1; 3]);
            pass.set_uniform(0, &self.camera);

            pass.set_pipeline(&self.pipelines.line);
            for mesh in &canvas.line_meshes {
                pass.draw(&mesh.vertices);
            }

            pass.set_pipeline(&self.pipelines.solid);
            for mesh in &canvas.solid_meshes {
                if let Some(texture) = self.resources.textures.get(&mesh.texture) {
                    pass.set_texture(1, texture);
                    pass.draw_triangles(&mesh.vertices, &mesh.triangles);
                }
            }

            pass.set_pipeline(&self.pipelines.prop);
            for instance in &canvas.prop_instances {
                pass.set_uniform(1, &instance.data.uniform);
                if let Some(meshes) = self.resources.props.get(&instance.prop) {
                    for mesh in meshes {
                        if let Some(texture) = self.resources.textures.get(&mesh.texture) {
                            pass.set_texture(2, texture);
                            // TODO
                        }
                    }
                }
            }
        }

        self.gpu.end_frame(frame);
    }
}

#[derive(Default)]
struct Resources {
    textures: HashMap<TextureID, gpu::Texture>,
    props: HashMap<PropID, Vec<PropMesh>>,
}
