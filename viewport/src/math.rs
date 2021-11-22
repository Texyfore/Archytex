use cgmath::{vec3, InnerSpace, Vector3};

pub trait IntersectionPoint<O> {
    fn intersection_point(&self, other: &O) -> Option<Vector3<f32>>;
}

pub trait BoxUtil {
    fn min(&self, rhs: &Self) -> Self;
    fn max(&self, rhs: &Self) -> Self;
    fn boxify(&self, length: f32) -> Self;
    fn snap(&self, length: f32) -> Self;
    fn coplanar(&self, rhs: &Self, length: f32) -> bool;
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub end: Vector3<f32>,
}

pub struct Triangle {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
}

pub struct Plane {
    pub origin: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Triangle {
    pub fn normal(&self) -> Vector3<f32> {
        let edge0 = self.b - self.a;
        let edge1 = self.c - self.a;
        edge0.cross(edge1).normalize()
    }
}

impl Ray {
    pub fn vec(&self) -> Vector3<f32> {
        self.end - self.origin
    }
}

impl IntersectionPoint<Triangle> for Ray {
    fn intersection_point(&self, other: &Triangle) -> Option<Vector3<f32>> {
        const EPSILON: f32 = 0.0001;
        let dir = self.vec().normalize();

        let a_to_b = other.b - other.a;
        let a_to_c = other.c - other.a;
        let u_vec = dir.cross(a_to_c);
        let det = a_to_b.dot(u_vec);

        if det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let a_to_origin = self.origin - other.a;
        let u = a_to_origin.dot(u_vec) * inv_det;

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let v_vec = a_to_origin.cross(a_to_b);
        let v = dir.dot(v_vec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let dist = a_to_c.dot(v_vec) * inv_det;

        if dist > EPSILON {
            Some(self.origin + dir * dist)
        } else {
            None
        }
    }
}

impl IntersectionPoint<Plane> for Ray {
    fn intersection_point(&self, other: &Plane) -> Option<Vector3<f32>> {
        let t = (other.origin - self.origin).dot(other.normal) / self.vec().dot(other.normal);
        Some(self.origin + self.vec() * t)
    }
}

impl BoxUtil for Vector3<f32> {
    fn min(&self, rhs: &Self) -> Self {
        vec3(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    fn max(&self, rhs: &Self) -> Self {
        vec3(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    fn boxify(&self, length: f32) -> Self {
        vec3(self.x.max(length), self.y.max(length), self.z.max(length))
    }

    fn snap(&self, length: f32) -> Self {
        vec3(
            snap(self.x, length),
            snap(self.y, length),
            snap(self.z, length),
        )
    }

    fn coplanar(&self, rhs: &Self, length: f32) -> bool {
        let diff = rhs - self;
        diff.x < length || diff.y < length || diff.z < length
    }
}

fn snap(x: f32, y: f32) -> f32 {
    (x / y).round() * y
}
