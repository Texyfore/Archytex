use super::transform::Transform;
use tools::math::{perspective, Deg, Matrix4, SquareMatrix};

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub projection: Matrix4<f32>,
}

impl Camera {
    pub fn new(fov: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Transform::identity(),
            fov,
            near,
            far,
            projection: Matrix4::identity(),
        }
    }

    pub fn calculate_projection(&mut self, aspect: f32) {
        self.projection = perspective(Deg(self.fov), aspect, self.near, self.far)
    }
}