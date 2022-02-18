use std::cmp::Ordering;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use cgmath::{Vector2, Vector3};

#[derive(Copy, Clone)]
pub enum Axis3 {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    pub inner: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn from_array(array: [f64; N]) -> Self {
        Self { inner: array }
    }
    pub fn dot(self, b: Self) -> f64 {
        self.inner.iter().zip(b.inner).map(|(a, b)| a * b).sum()
    }
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalized(self) -> Self {
        self / self.length()
    }
    pub fn from_single(single: f64) -> Self {
        Self { inner: [single; N] }
    }
    pub fn ones() -> Self {
        Self::from_single(1.0)
    }
    pub fn sum(self) -> f64 {
        self.dot(Self::ones())
    }
    pub fn powi(self, n: i32) -> Self {
        let mut o = self;
        for v in o.inner.iter_mut() {
            *v = v.powi(n);
        }
        o
    }
    pub fn powf(self, n: f64) -> Self {
        let mut o = self;
        for v in o.inner.iter_mut() {
            *v = v.powf(n);
        }
        o
    }
    pub fn min(self, rhs: Self) -> Self {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a = a.min(b);
        }
        o
    }
    pub fn max(self, rhs: Self) -> Self {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a = a.max(b);
        }
        o
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self { inner: [0.0; N] }
    }
}

impl<const N: usize> Debug for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
impl<const N: usize> Mul for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a *= b;
        }
        o
    }
}
impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a += b;
        }
        o
    }
}
impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a -= b;
        }
        o
    }
}
impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl<const N: usize> MulAssign for Vector<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut o = self;
        for a in o.inner.iter_mut() {
            *a *= rhs;
        }
        o
    }
}
impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        rhs * self
    }
}
impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut o = self;
        for a in o.inner.iter_mut() {
            *a /= rhs;
        }
        o
    }
}
impl<const N: usize> DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut o = self;
        for v in o.inner.iter_mut() {
            *v = -*v;
        }
        o
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize, const M: usize> {
    pub inner: [Vector<N>; M],
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut o: Self = Default::default();
        for i in 0..N {
            o[i][i] = 1.0;
        }
        o
    }
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn transpose(self) -> Matrix<M, N> {
        let mut o: Matrix<M, N> = Default::default();
        for x in 0..N {
            let vec = &mut o[x];
            for y in 0..M {
                vec[y] = self[y][x];
            }
        }
        o
    }

    pub fn from_vectors(vectors: [Vector<N>; M]) -> Self {
        Self { inner: vectors }
    }
}

impl<const N: usize, const M: usize> Default for Matrix<N, M> {
    fn default() -> Self {
        Self {
            inner: [Default::default(); M],
        }
    }
}

impl<const N: usize, const M: usize> Index<usize> for Matrix<N, M> {
    type Output = Vector<N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for Matrix<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const N: usize, const M: usize> Debug for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<const N: usize, const M: usize> Mul<Vector<M>> for Matrix<N, M> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<M>) -> Self::Output {
        let mut o = Vector::from_array([0.0; N]);
        for (scalar, vector) in rhs.inner.iter().zip(self.inner) {
            o += *scalar * vector;
        }
        o
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<Matrix<N, M>> for Matrix<M, K> {
    type Output = Matrix<N, K>;

    fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut o: Self::Output = Default::default();
        for (i, vector) in self.inner.iter().enumerate() {
            o[i] = rhs * (*vector);
        }
        o
    }
}

#[macro_export]
macro_rules! vector {
    ( $($x:expr),* ) => {
        $crate::utilities::math::Vector::from_array([$($x, )*])
    };
}
#[macro_export]
macro_rules! matrix {
    ( $($x:expr),* ) => {
        $crate::utilities::math::Matrix::from_vectors([$($x, )*])
    };
}

pub type Vec3 = Vector<3>;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self::from_array([x, y, z])
    }
    pub fn to_srgb(self) -> Self {
        self.powf(1.0 / 2.2)
    }
    pub fn from_srgb(self) -> Self {
        self.powf(2.2)
    }
    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
    pub fn z(self) -> f64 {
        self[2]
    }
    pub fn cross(self, rhs: Self) -> Self {
        vector!(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        )
    }

    pub fn get(self, axis: Axis3) -> f64 {
        match axis {
            Axis3::X => self.x(),
            Axis3::Y => self.y(),
            Axis3::Z => self.z(),
        }
    }

    pub fn max_axis(self) -> Axis3 {
        [Axis3::X, Axis3::Y, Axis3::Z]
            .into_iter()
            .max_by(|a, b| {
                self.get(*a)
                    .partial_cmp(&self.get(*b))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap()
    }
}

impl From<mdl::Vector3> for Vec3 {
    fn from(vec: mdl::Vector3) -> Self {
        (&vec).into()
    }
}

impl From<&mdl::Vector3> for Vec3 {
    fn from(vec: &mdl::Vector3) -> Self {
        vector![vec.x as f64, vec.y as f64, vec.z as f64]
    }
}

impl From<Vector3<f32>> for Vec3 {
    fn from(a: Vector3<f32>) -> Self {
        Self::new(a.x as f64, a.y as f64, a.z as f64)
    }
}
impl From<Vector2<f32>> for Vec2 {
    fn from(a: Vector2<f32>) -> Self {
        vector!(a.x as f64, a.y as f64)
    }
}

pub type Vec2 = Vector<2>;

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self::from_array([x, y])
    }
    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
}

impl From<mdl::Vector2> for Vec2 {
    fn from(vec: mdl::Vector2) -> Self {
        (&vec).into()
    }
}

impl From<&mdl::Vector2> for Vec2 {
    fn from(vec: &mdl::Vector2) -> Self {
        vector![vec.x as f64, vec.y as f64]
    }
}

pub type Matrix3x3 = Matrix<3, 3>;

impl Matrix3x3 {
    pub fn det(self) -> f64 {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2])
            - self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2])
            + self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }
    pub fn cramer(self, b: Vec3) -> Option<Vec3> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }
        let mut o = Vec3::from_single(0.0);
        for (i, v) in o.inner.iter_mut().enumerate() {
            let m = {
                let mut m = self;
                m[i] = b;
                m
            };
            *v = m.det() / det;
        }
        Some(o)
    }
    pub fn rotate_x(self, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        let matrix: Matrix3x3 = matrix!(
            vector![1.0, 0.0, 0.0],
            vector![0.0, cos, sin],
            vector![0.0, -sin, cos]
        );
        self * matrix
    }
    pub fn rotate_y(self, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        let matrix: Matrix3x3 = matrix!(
            vector![cos, 0.0, -sin],
            vector![0.0, 1.0, 0.0],
            vector![sin, 0.0, cos]
        );
        self * matrix
    }
    pub fn rotate_z(self, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        let matrix: Matrix3x3 = matrix!(
            vector![cos, sin, 0.0],
            vector![-sin, cos, 0.0],
            vector![0.0, 0.0, 1.0]
        );
        self * matrix
    }
}

pub enum QuadraticResult {
    TwoResults(f64, f64),
    OneResult(f64),
    NoResults,
}

fn solve_for_determinant_sqrt(d: f64, b: f64, a: f64) -> f64 {
    (-b + d) / (2.0 * a)
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticResult {
    let d = b * b - 4.0 * a * c;
    match d {
        n if n == 0.0 => QuadraticResult::OneResult(solve_for_determinant_sqrt(0.0, b, a)),
        n if n > 0.0 => QuadraticResult::TwoResults(
            solve_for_determinant_sqrt(d.sqrt(), b, a),
            solve_for_determinant_sqrt(-d.sqrt(), b, a),
        ),
        _ => QuadraticResult::NoResults,
    }
}
