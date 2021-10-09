use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

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
    pub fn sum(self) -> f64{
        self.dot(Self::ones())
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
