use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn fill(x: f32) -> Self {
        Self { x, y: x, z: x }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(vec: Vec3) -> Self {
        [vec.x, vec.y, vec.z]
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl $trait for Vec3 {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }
    };
}

macro_rules! impl_op_scalar {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl $trait<f32> for Vec3 {
            type Output = Self;
            fn $fn(self, rhs: f32) -> Self::Output {
                Self {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl $trait for Vec3 {
            fn $fn(&mut self, rhs: Self) {
                self.x = self.x $op rhs.x;
                self.y = self.y $op rhs.y;
                self.z = self.z $op rhs.z;
            }
        }
    };
}

macro_rules! impl_op_assign_scalar {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl $trait<f32> for Vec3 {
            fn $fn(&mut self, rhs: f32) {
                self.x = self.x $op rhs;
                self.y = self.y $op rhs;
                self.z = self.z $op rhs;
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
impl_op_scalar!(Mul, mul, *);
impl_op_scalar!(Div, div, /);

impl_op_assign!(AddAssign, add_assign, +);
impl_op_assign!(SubAssign, sub_assign, -);
impl_op_assign!(MulAssign, mul_assign, *);
impl_op_assign!(DivAssign, div_assign, /);
impl_op_assign_scalar!(MulAssign, mul_assign, *);
impl_op_assign_scalar!(DivAssign, div_assign, /);
