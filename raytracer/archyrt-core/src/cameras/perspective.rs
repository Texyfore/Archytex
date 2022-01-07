use crate::{
    api::{camera::Camera, fragment_render::FragmentContext},
    matrix,
    utilities::{
        math::{Matrix, Vec2, Vec3},
        ray::Ray,
    },
    vector,
};
use crate::utilities::math::Matrix3x3;

#[derive(Debug)]
pub struct PerspectiveCamera {
    pub matrix: Matrix<3, 3>,
    pub position: Vec3,
    pub focal_distance: f64,
}

impl PerspectiveCamera {
    pub fn new(position: Vec3, direction: Vec3, focal_distance: f64) -> Self {
        Self {
            matrix: Self::look_at_matrix(direction),
            position,
            focal_distance,
        }
    }
    pub fn look_at_matrix(direction: Vec3) -> Matrix<3, 3> {
        let forward = direction;
        let left = Vec3::new(0.0, 1.0, 0.0).cross(forward).normalized();
        let up = forward.cross(left);
        matrix!(left, up, forward)
    }

    pub fn from_euler(position: Vec3, euler_direction: Vec3, focal_distance: f64) -> Self {
        Self {
            position,
            focal_distance,
            matrix: Matrix3x3::identity().rotate_z(euler_direction.z()).rotate_y(euler_direction.y()).rotate_x(euler_direction.x()),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, ctx: &FragmentContext, pos: Vec2) -> Ray {
        let uv = {
            //Calculate center-origin coordinates
            let mut uv = vector!(pos.x() - 0.5, 0.5 - pos.y());
            //Compensate for aspect ratio
            uv.inner[0] *= ctx.width / ctx.height;
            uv
        };
        let dir = vector!(uv.x(), uv.y(), self.focal_distance).normalized();
        //Apply rotation matrix
        let dir = self.matrix * dir;
        Ray {
            origin: self.position,
            direction: dir,
        }
    }
}
