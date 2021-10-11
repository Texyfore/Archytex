use crate::utilities::{
    math::Vec3,
    ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
};

pub struct Surface {
    pub normal: Vec3,
    pub distance: f64,
    pub color: Vec3,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            normal: Vec3::new(0.0, 1.0, 0.0),
            distance: 0.0,
            color: Vec3::from_single(1.0),
        }
    }
}

impl Surface {
    pub fn from_points(points: [Vec3; 3], color: Vec3) -> Self {
        let [a, b, c] = points;
        let b = b - a;
        let c = c - a;
        let normal = b.cross(c);
        let distance = normal.dot(a);
        Self {
            color,
            normal,
            distance,
        }
    }
}

impl Intersectable for Surface {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        //Solving   self.normal*(ray.origin+ray.direction*t)=self.distance              ->
        //          self.normal*ray.origin+self.normal*ray.direction*t=self.distance    ->
        //          self.normal*ray.direction*t=self.distance-self.normal*ray.origin    ->
        //          t=(self.distance-self.normal*ray.origin)/(self.normal*ray.direction)
        let left_side = self.normal.dot(ray.direction);
        //Backface culling
        if left_side >= 0.0 {
            return None;
        }
        let right_side = self.distance - self.normal.dot(ray.origin);
        let t = right_side / left_side;
        if t < 0.0 {
            return None;
        }
        Some(
            IntersectionBuilder {
                distance: Some(t),
                normal: self.normal,
                color: self.color,
                ray,
                ..Default::default()
            }
            .build(),
        )
    }
}
