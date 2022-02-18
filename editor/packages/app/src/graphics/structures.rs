use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix, Vector2, Vector3};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CameraMatrices {
    pub world_to_clip: Matrix4<f32>,
    pub view_to_world: Matrix4<f32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LineVertex {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SolidVertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
    pub tint: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GroundVertex {
    pub position: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TransformTint {
    pub transform: Matrix4<f32>,
    pub tint: [f32; 4],
}

impl Default for CameraMatrices {
    fn default() -> Self {
        Self {
            view_to_world: Matrix4::identity(),
            world_to_clip: Matrix4::identity(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GizmoInstance {
    pub matrix: Matrix4<f32>,
    pub color: [f32; 3],
}

unsafe impl Zeroable for CameraMatrices {}
unsafe impl Pod for CameraMatrices {}

unsafe impl Zeroable for LineVertex {}
unsafe impl Pod for LineVertex {}

unsafe impl Zeroable for SolidVertex {}
unsafe impl Pod for SolidVertex {}

unsafe impl Zeroable for GroundVertex {}
unsafe impl Pod for GroundVertex {}

unsafe impl Zeroable for TransformTint {}
unsafe impl Pod for TransformTint {}

unsafe impl Zeroable for GizmoInstance {}
unsafe impl Pod for GizmoInstance {}
