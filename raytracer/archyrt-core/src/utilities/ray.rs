use super::math::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

#[derive(Default)]
pub struct IntersectionBuilder {
    pub ray: Ray,
    /// At least one of pos, distance or distance_squared are required
    pub pos: Option<Vec3>,
    /// At least one of pos, distance or distance_squared are required
    pub distance: Option<f64>,
    /// At least one of pos, distance or distance_squared are required
    pub distance_squared: Option<f64>,
    pub normal: Vec3,
    pub color: Vec3,
}
impl IntersectionBuilder {
    pub fn build(self) -> Intersection {
        Intersection(self)
    }
}

pub struct Intersection(IntersectionBuilder);

impl Intersection {
    pub fn get_ray(&self) -> Ray {
        self.0.ray
    }
    pub fn get_pos(&self) -> Vec3 {
        if let Some(pos) = self.0.pos {
            return pos;
        }
        if let Some(distance) = self.0.distance {
            return distance * self.0.ray.direction + self.0.ray.origin;
        }
        if let Some(_) = self.0.distance_squared {
            return self.get_distance() * self.0.ray.direction + self.0.ray.origin;
        }
        panic!("Invalid intersection object: could not reconstruct position");
    }
    pub fn get_distance(&self) -> f64 {
        if let Some(distance) = self.0.distance {
            return distance;
        }
        if let Some(_) = self.0.distance_squared {
            return self.get_distance_squared().sqrt();
        }
        if let Some(_) = self.0.pos {
            return self.get_distance_squared().sqrt();
        }
        panic!("Invalid intersection object: could not reconstruct distance")
    }
    pub fn get_distance_squared(&self) -> f64 {
        if let Some(distance_squared) = self.0.distance_squared {
            return distance_squared;
        }
        if let Some(distance) = self.0.distance {
            return distance * distance;
        }
        if let Some(pos) = self.0.pos {
            return (pos - self.0.ray.origin).length_squared();
        }
        panic!("Invalid intersection object: could not reconstruct distance^2")
    }
    pub fn get_normal(&self) -> Vec3 {
        self.0.normal
    }
    pub fn get_color(&self) -> Vec3 {
        self.0.color
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}
