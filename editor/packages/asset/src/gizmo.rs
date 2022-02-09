use std::fmt::Debug;

use bytemuck::{Pod, Zeroable};
use cgmath::Vector3;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GizmoID(pub u32);

impl Debug for GizmoID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

pub struct Gizmo {
    pub vertices: Vec<GizmoVertex>,
    pub triangles: Vec<[u16; 3]>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GizmoVertex {
    pub position: Vector3<f32>,
}

unsafe impl Zeroable for GizmoVertex {}
unsafe impl Pod for GizmoVertex {}
