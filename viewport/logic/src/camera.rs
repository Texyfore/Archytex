use tools::math::{perspective, Deg, Mat4, SquareMatrix};

pub struct Camera {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub view: Mat4,
    pub projection: Mat4,
}

impl Camera {
    pub fn new(fov: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            near,
            far,
            view: Mat4::identity(),
            projection: Mat4::identity(),
        }
    }

    pub fn calculate_projection(&mut self, aspect: f32) {
        self.projection = perspective(Deg(self.fov), aspect, self.near, self.far)
    }
}
