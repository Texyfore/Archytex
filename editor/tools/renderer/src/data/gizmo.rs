use bytemuck::{Pod, Zeroable};
use gpu::{data::Buffer, BufferUsages};
use tk3d::{math::Matrix4, Triangle};

use crate::Renderer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GizmoInstance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
}

impl GizmoInstance {
    pub fn new(matrix: Matrix4<f32>, color: [f32; 3]) -> Self {
        Self {
            matrix: matrix.into(),
            color: [color[0], color[1], color[2], 1.0],
        }
    }
}

pub struct GizmoInstances {
    pub(crate) buffer: Buffer<GizmoInstance>,
}

pub struct GizmoMesh {
    pub(crate) vertices: Buffer<tk3d::agzm::Vertex>,
    pub(crate) triangles: Buffer<Triangle>,
}

impl Renderer {
    pub fn create_gizmo_instances(&self, instances: &[GizmoInstance]) -> GizmoInstances {
        GizmoInstances {
            buffer: self.gpu.create_buffer(instances, BufferUsages::VERTEX),
        }
    }

    pub fn create_gizmo_mesh(
        &self,
        vertices: &[tk3d::agzm::Vertex],
        triangles: &[Triangle],
    ) -> GizmoMesh {
        GizmoMesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(triangles, BufferUsages::INDEX),
        }
    }
}
