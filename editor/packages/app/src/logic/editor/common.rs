use cgmath::{vec3, Deg, Matrix4, SquareMatrix, Vector3};

use crate::graphics::structures::LineVertex;

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
            Self::X => [1.0, 0.0, 0.0],
            Self::Y => [0.0, 1.0, 0.0],
            Self::Z => [0.0, 0.0, 1.0],
        }
    }

    pub fn line_vertices(&self, center: Vector3<f32>) -> [LineVertex; 2] {
        let (min, max) = match self {
            Self::X => (vec3(-1.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)),
            Self::Y => (vec3(0.0, -1.0, 0.0), vec3(0.0, 1.0, 0.0)),
            Self::Z => (vec3(0.0, 0.0, -1.0), vec3(0.0, 0.0, 1.0)),
        };

        let color = match self {
            Self::X => [1.0, 0.0, 0.0],
            Self::Y => [0.0, 1.0, 0.0],
            Self::Z => [0.0, 0.0, 1.0],
        };

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
}
