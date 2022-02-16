use cgmath::{InnerSpace, Vector3};

pub struct Ray {
    pub start: Vector3<f32>,
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

pub struct Sphere {
    pub origin: Vector3<f32>,
    pub radius: f32,
}

pub trait Intersects<O> {
    fn intersects(&self, other: &O) -> Option<Intersection>;
}

pub struct Intersection {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Ray {
    pub fn vector(&self) -> Vector3<f32> {
        self.end - self.start
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.vector().normalize()
    }
}

impl Intersects<Triangle> for Ray {
    fn intersects(&self, other: &Triangle) -> Option<Intersection> {
        const EPSILON: f32 = 0.0001;
        let dir = self.direction();

        let a_to_b = other.b - other.a;
        let a_to_c = other.c - other.a;
        let u_vec = dir.cross(a_to_c);
        let det = a_to_b.dot(u_vec);

        if det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let a_to_origin = self.start - other.a;
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
            Some(Intersection {
                point: self.start + dir * dist,
                normal: other.normal(),
            })
        } else {
            None
        }
    }
}

impl Intersects<Plane> for Ray {
    fn intersects(&self, other: &Plane) -> Option<Intersection> {
        let denom = other.normal.dot(self.direction());
        if denom.abs() > 0.0001 {
            let t = (other.origin - self.start).dot(other.normal) / denom;
            (t >= 0.0).then(|| Intersection {
                point: self.start + self.direction() * t,
                normal: other.normal,
            })
        } else {
            None
        }
    }
}

impl Intersects<Sphere> for Ray {
    fn intersects(&self, other: &Sphere) -> Option<Intersection> {
        let dir = self.direction();
        let sphere_to_ray = self.start - other.origin;
        let b = dir.dot(sphere_to_ray) * 2.0;
        let c = sphere_to_ray.magnitude2() - other.radius * other.radius;

        let b2 = b * b;
        let c4 = 4.0 * c;
        let b2_c4 = b2 - c4;

        (b2_c4 >= 0.0).then(|| {
            let dist = (-b - b2_c4.sqrt()) / 2.0;
            let point = self.start + dir * dist;
            let normal = (point - other.origin).normalize();
            Intersection { point, normal }
        })
    }
}

impl Triangle {
    pub fn normal(&self) -> Vector3<f32> {
        let edge0 = self.b - self.a;
        let edge1 = self.c - self.a;
        edge0.cross(edge1).normalize()
    }
}

pub trait MinMax {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

impl MinMax for Vector3<i32> {
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl MinMax for Vector3<f32> {
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

pub trait Snap {
    fn snap(self, step: i32) -> Vector3<i32>;
}

impl Snap for Vector3<f32> {
    fn snap(self, step: i32) -> Vector3<i32> {
        self.map(|e| {
            let step = step as f32 / 100.0;
            let res = ((e / step).floor() * step * 100.0) as i32;
            res.clamp(-10000, 10000)
        })
    }
}

#[cfg(test)]
mod tests {
    mod ray {
        use cgmath::{assert_relative_eq, vec3};

        use crate::math::{Intersects, Plane, Ray, Sphere, Triangle};

        #[test]
        fn vector() {
            let ray = Ray {
                start: vec3(0.0, 0.0, 0.0),
                end: vec3(2.0, 0.0, 0.0),
            };
            assert_relative_eq!(ray.vector(), vec3(2.0, 0.0, 0.0))
        }

        #[test]
        fn direction() {
            let ray = Ray {
                start: vec3(0.0, 0.0, 0.0),
                end: vec3(2.0, 0.0, 0.0),
            };
            assert_relative_eq!(ray.direction(), vec3(1.0, 0.0, 0.0))
        }

        #[test]
        fn intersects_triangle() {
            let triangle = Triangle {
                a: vec3(0.0, -1.0, -1.0),
                b: vec3(0.0, -1.0, 1.0),
                c: vec3(0.0, 1.0, 0.0),
            };

            let ray = Ray {
                start: vec3(-1.0, 0.0, 0.0),
                end: vec3(1.0, 0.0, 0.0),
            };

            let intersection = ray.intersects(&triangle).unwrap();
            assert_relative_eq!(intersection.point, vec3(0.0, 0.0, 0.0));
            assert_relative_eq!(intersection.normal, vec3(-1.0, 0.0, 0.0));
        }

        #[test]
        fn intersects_plane() {
            let plane = Plane {
                origin: vec3(0.0, 0.0, 0.0),
                normal: vec3(0.0, 1.0, 0.0),
            };

            let ray = Ray {
                start: vec3(0.0, 1.0, 0.0),
                end: vec3(0.0, -1.0, 0.0),
            };

            let intersection = ray.intersects(&plane).unwrap();
            assert_relative_eq!(intersection.point, vec3(0.0, 0.0, 0.0));
            assert_relative_eq!(intersection.normal, vec3(0.0, 1.0, 0.0));
        }

        #[test]
        fn intersects_sphere() {
            let sphere = Sphere {
                origin: vec3(0.0, 0.0, 0.0),
                radius: 1.0,
            };

            let ray = Ray {
                start: vec3(-2.0, 0.0, 0.0),
                end: vec3(2.0, 0.0, 0.0),
            };

            let intersection = ray.intersects(&sphere).unwrap();
            assert_relative_eq!(intersection.point, vec3(-1.0, 0.0, 0.0));
            assert_relative_eq!(intersection.normal, vec3(-1.0, 0.0, 0.0));
        }
    }
}
