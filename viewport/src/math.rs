use cgmath::{InnerSpace, Vector3};

pub trait IntersectsSphere {
    fn intersects_sphere(&self, sphere: &Sphere) -> bool;
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub end: Vector3<f32>,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub origin: Vector3<f32>,
    pub radius: f32,
}

impl Ray {
    pub fn vec(&self) -> Vector3<f32> {
        self.end - self.origin
    }
}

impl IntersectsSphere for Ray {
    fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        let ap = sphere.origin - self.origin;
        let ab = self.vec();
        let proj = self.origin + ab * (ap.dot(ab) / ab.magnitude2());
        (proj - sphere.origin).magnitude2() <= sphere.radius * sphere.radius
    }
}
