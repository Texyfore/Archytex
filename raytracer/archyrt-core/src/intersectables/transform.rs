use crate::utilities::{ray::{Intersectable, Ray, Intersection}, math::{Vec3}};

pub struct Transform<T: Intersectable>{
    pub inner: T,
    pub transformation: Vec3
}

impl<T: Intersectable> Intersectable for Transform<T>{
    type C = T::C;

    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        let origin = ray.origin-self.transformation;
        let ray = Ray::new(origin, ray.direction);
        let mut result = self.inner.intersect(ray)?.to_builder();

        if let Some(pos) = result.pos{
            result.pos = Some(pos + self.transformation);
        }

        Some(result.build())
    }
}