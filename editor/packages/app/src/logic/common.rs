use std::f32::consts::PI;

use cgmath::{vec3, Deg, Matrix4, Quaternion, Rotation3, SquareMatrix, Vector2, Vector3};

use crate::{color, graphics::structures::LineVertex};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn all() -> [Self; 3] {
        [Self::X, Self::Y, Self::Z]
    }

    pub fn others(&self) -> [Self; 2] {
        match self {
            Self::X => [Self::Y, Self::Z],
            Self::Y => [Self::X, Self::Z],
            Self::Z => [Self::X, Self::Y],
        }
    }

    pub fn unit(self) -> Vector3<f32> {
        match self {
            Self::X => Vector3::unit_x(),
            Self::Y => Vector3::unit_y(),
            Self::Z => Vector3::unit_z(),
        }
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            Self::X => color!("ec4659"),
            Self::Y => color!("85cc34"),
            Self::Z => color!("5084d4"),
        }
    }

    pub fn line_vertices(&self, center: Vector3<f32>) -> [LineVertex; 2] {
        let (min, max) = match self {
            Self::X => (vec3(-1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)),
            Self::Y => (vec3(0.0, -1.0, 0.0), vec3(0.0, 1.0, 0.0)),
            Self::Z => (vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, 1.0)),
        };

        let color = self.color().map(|x| x * 2.0);

        [
            LineVertex {
                position: center + min * 1000.0,
                color,
            },
            LineVertex {
                position: center + max * 1000.0,
                color,
            },
        ]
    }

    pub fn rotation_from_y(&self) -> Matrix4<f32> {
        match self {
            Self::X => Matrix4::from_angle_z(Deg(-90.0)),
            Self::Y => Matrix4::identity(),
            Self::Z => Matrix4::from_angle_x(Deg(90.0)),
        }
    }

    pub fn plane_rotation_from_y(&self) -> Matrix4<f32> {
        match self {
            Self::X => Matrix4::from_angle_y(Deg(-90.0)),
            Self::Y => Matrix4::from_angle_x(Deg(90.0)),
            Self::Z => Matrix4::identity(),
        }
    }

    pub fn angle(&self, angle: f32, forward: Vector3<f32>) -> Quaternion<f32> {
        match self {
            Self::X => Quaternion::from_angle_x(Deg(angle * -forward.x.signum())),
            Self::Y => Quaternion::from_angle_y(Deg(angle * -forward.y.signum())),
            Self::Z => Quaternion::from_angle_z(Deg(angle * -forward.z.signum())),
        }
    }
}

pub enum Snap {
    None,
    Deg15,
}

impl Snap {
    pub fn snap(&self, x: i32) -> i32 {
        match self {
            Snap::None => x,
            Snap::Deg15 => (x as f32 / 15.0) as i32 * 15,
        }
    }
}

pub fn calc_angle(origin: Vector2<f32>, pos: Vector2<f32>) -> i32 {
    let vector = pos - origin;
    let rad = vector.y.atan2(vector.x);
    let deg = rad * (180.0 / PI);
    -deg as i32
}
