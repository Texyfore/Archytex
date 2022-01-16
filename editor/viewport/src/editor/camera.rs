use tk3d::math::{
    perspective, vec2, vec3, Deg, Matrix3, Matrix4, SquareMatrix, Transform, Vector2, Vector3,
};

const SPEED_MULTIPLIER: f32 = 1.1;

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    projection: Matrix4<f32>,
    speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: vec3(20.0, 20.0, 20.0),
            rotation: vec2(-45.0, 45.0),
            projection: Matrix4::identity(),
            speed: 8.0,
        }
    }
}

impl Camera {
    pub fn recreate_projection(&mut self, width: u32, height: u32) {
        self.projection = perspective(Deg(80.0), width as f32 / height as f32, 0.1, 256.0);
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.projection
            * (Matrix4::from_translation(self.position)
                * Matrix4::from_angle_y(Deg(self.rotation.y))
                * Matrix4::from_angle_x(Deg(self.rotation.x)))
            .inverse_transform()
            .unwrap()
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.forward() * self.speed * delta;
    }

    pub fn move_backward(&mut self, delta: f32) {
        self.position -= self.forward() * self.speed * delta;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.position += self.right() * self.speed * delta;
    }

    pub fn move_left(&mut self, delta: f32) {
        self.position -= self.right() * self.speed * delta;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position += Vector3::unit_y() * self.speed * delta;
    }

    pub fn move_down(&mut self, delta: f32) {
        self.position -= Vector3::unit_y() * self.speed * delta;
    }

    pub fn look(&mut self, mouse_delta: Vector2<f32>, delta: f32) {
        const SENSITIVITY: f32 = 4.0;
        self.rotation.y -= mouse_delta.x * SENSITIVITY * delta;
        self.rotation.x =
            (self.rotation.x - mouse_delta.y * SENSITIVITY * delta).clamp(-90.0, 90.0);
    }

    pub fn increase_speed(&mut self) {
        self.speed *= SPEED_MULTIPLIER;
    }

    pub fn decrease_speed(&mut self) {
        self.speed /= SPEED_MULTIPLIER;
    }

    fn forward(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * -Vector3::unit_z()
    }

    fn right(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * Vector3::unit_x()
    }
}

// TODO: Tests
