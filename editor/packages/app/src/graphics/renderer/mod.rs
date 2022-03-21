mod pipelines;
mod resources;

use std::rc::Rc;

use asset::{GizmoID, PropID, TextureID};
use gpu::{
    Buffer, BufferUsages, DepthBuffer, Gpu, InstanceConfig, MsaaFramebuffer, Surface, Texture,
    Uniform,
};

use self::{pipelines::Pipelines, resources::Resources};

use super::{structures::CameraMatrices, Canvas};

pub use resources::{GizmoMesh, PropMesh, PropModel};

pub struct Renderer {
    gpu: Rc<Gpu>,
    surface: Rc<Surface>,
    depth_buffer: DepthBuffer,
    msaa_buffer: MsaaFramebuffer,
    pipelines: Pipelines,
    resources: Resources,
    camera: Uniform<CameraMatrices>,
    grid: Uniform<[i32; 4]>,
    solid_tris: Buffer<[u16; 3]>,
}

impl Renderer {
    pub(super) fn new(gpu: Rc<Gpu>, surface: Rc<Surface>) -> Self {
        let depth_buffer = gpu.create_depth_buffer(800, 600);
        let msaa_buffer = gpu.create_msaa_framebuffer(&surface, 800, 600);
        let pipelines = Pipelines::new(&gpu, &surface);
        let resources = Resources::default();
        let camera = gpu.create_uniform(&CameraMatrices::default());
        let grid = gpu.create_uniform(&[100; 4]);
        let solid_tris = gpu.create_buffer(&gen_solid_tris(), BufferUsages::INDEX);

        Self {
            gpu,
            surface,
            depth_buffer,
            msaa_buffer,
            pipelines,
            resources,
            camera,
            grid,
            solid_tris,
        }
    }

    pub fn add_texture(&mut self, id: TextureID, texture: Texture) {
        self.resources.add_texture(id, texture);
    }

    pub fn add_prop(&mut self, id: PropID, model: PropModel) {
        self.resources.add_prop(id, model);
    }

    pub fn add_gizmo(&mut self, id: GizmoID, mesh: GizmoMesh) {
        self.resources.add_gizmo(id, mesh);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.configure(&self.gpu, width, height);
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
        self.msaa_buffer = self
            .gpu
            .create_msaa_framebuffer(&self.surface, width, height);
    }

    pub fn render(&self, canvas: Canvas) {
        self.gpu.set_uniform(&self.camera, &canvas.camera_matrices);
        self.gpu.set_uniform(&self.grid, &[canvas.grid_len; 4]);

        let mut frame = self.gpu.begin_frame(&self.surface);

        {
            let mut pass =
                frame.begin_pass(&self.depth_buffer, &self.msaa_buffer, &[0.537, 0.847, 1.0]);

            pass.set_uniform(0, &self.camera);

            pass.set_pipeline(&self.pipelines.line);
            for mesh in &canvas.line_meshes {
                pass.draw(&mesh.vertices);
            }

            pass.set_pipeline(&self.pipelines.solid);
            pass.set_uniform(2, &self.grid);
            pass.set_triangles(&self.solid_tris);

            for (texture, geometry) in canvas.solids {
                if let Some(texture) = self.resources.texture(texture) {
                    pass.set_texture(1, texture);
                    for (vertices, faces) in geometry {
                        // should be safe
                        let vertices = unsafe { &*vertices };
                        pass.set_vertices(vertices);
                        for face in faces {
                            pass.draw_face(face);
                        }
                    }
                }
            }

            pass.set_pipeline(&self.pipelines.ground);
            pass.set_uniform(2, &self.grid);
            for mesh in &canvas.ground_meshes {
                if let Some(texture) = self.resources.texture(mesh.texture) {
                    pass.set_texture(1, texture);
                    pass.draw_triangles(&mesh.vertices, &mesh.triangles);
                }
            }

            pass.set_pipeline(&self.pipelines.prop);
            for (prop, data) in &canvas.props {
                if let Some(prop) = self.resources.prop(*prop) {
                    for mesh in &prop.meshes {
                        if let Some(texture) = self.resources.texture(mesh.texture) {
                            pass.set_texture(2, texture);
                            for data in data {
                                pass.set_uniform(1, &data.uniform);
                                pass.draw_triangles(&mesh.vertices, &mesh.triangles);
                            }
                        }
                    }
                }
            }

            pass.set_pipeline(&self.pipelines.gizmo);
            for group in &canvas.gizmo_groups {
                if let Some(mesh) = self.resources.gizmo(group.gizmo) {
                    pass.draw_triangles_instanced(
                        &mesh.vertices,
                        &mesh.triangles,
                        InstanceConfig {
                            slot: 1,
                            buffer: &group.instances.buffer,
                            range: 0..group.instances.len,
                        },
                    );
                }
            }
        }

        {
            let mut pass = frame.begin_pass_no_clear(&self.depth_buffer, &self.msaa_buffer);
            pass.set_uniform(0, &self.camera);

            pass.set_pipeline(&self.pipelines.gizmo);
            for group in &canvas.gizmo_groups_no_depth {
                if let Some(mesh) = self.resources.gizmo(group.gizmo) {
                    pass.draw_triangles_instanced(
                        &mesh.vertices,
                        &mesh.triangles,
                        InstanceConfig {
                            slot: 1,
                            buffer: &group.instances.buffer,
                            range: 0..group.instances.len,
                        },
                    );
                }
            }
        }

        self.gpu.end_frame(frame);
    }
}

fn gen_solid_tris() -> Vec<[u16; 3]> {
    (0..6)
        .into_iter()
        .map(|t0| {
            let t0 = t0 * 4;
            [[t0, t0 + 1, t0 + 2], [t0, t0 + 2, t0 + 3]].into_iter()
        })
        .flatten()
        .collect()
}
