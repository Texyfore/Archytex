use tools::{
    gfx::Graphics,
    math::{perspective, Deg, Mat4, SquareMatrix},
};

pub struct Camera {
    fov: f32,
    near: f32,
    far: f32,
    view: Mat4,
}

impl Camera {
    pub fn new(fov: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            near,
            far,
            view: Mat4::identity(),
        }
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = view;
    }

    pub fn update_graphics(&self, gfx: &Graphics, width: u32, height: u32) {
        gfx.set_camera_projection(perspective(
            Deg(self.fov),
            width as f32 / height as f32,
            self.near,
            self.far,
        ));
        gfx.set_camera_view(self.view);
    }
}
