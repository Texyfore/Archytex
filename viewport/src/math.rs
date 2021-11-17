use cgmath::{InnerSpace, Vector3};

pub trait Intersects<O> {
    fn intersects(&self, other: &O) -> bool;
}

pub trait IntersectionPoint<O> {
    fn intersection_point(&self, other: &O) -> Vector3<f32>;
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

impl Ray {
    pub fn vec(&self) -> Vector3<f32> {
        self.end - self.origin
    }
}

impl Intersects<Triangle> for Ray {
    fn intersects(&self, triangle: &Triangle) -> bool {
        let a_to_b = triangle.b - triangle.a;
        let a_to_c = triangle.c - triangle.a;
        let dir = self.vec().normalize();

        let u_vec = dir.cross(a_to_c);

        let det = a_to_b.dot(u_vec);

        if det < 0.0001 {
            return false;
        }

        let inv_det = 1.0 / det;

        let a_to_origin = self.origin - triangle.a;

        let u = a_to_origin.dot(u_vec) * inv_det;

        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let v_vec = a_to_origin.cross(a_to_b);

        let v = dir.dot(v_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let dist = a_to_c.dot(v_vec) * inv_det;
        dist > 0.0001
    }
}

impl IntersectionPoint<Plane> for Ray {
    fn intersection_point(&self, other: &Plane) -> Vector3<f32> {
        let t = (other.origin - self.origin).dot(other.normal) / self.vec().dot(other.normal);
        self.origin + self.vec() * t
    }
}
