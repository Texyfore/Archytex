use crate::utilities::{ray::{Intersectable, Ray, Intersection}, math::Matrix3x3};

pub struct ApplyMatrix<T: Intersectable>{
    pub inner: T,
    pub matrix: Matrix3x3
}

impl<T: Intersectable> Intersectable for ApplyMatrix<T>{
    type C = T::C;

    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        let origin = self.matrix*ray.origin;
        let direction = self.matrix*ray.direction;
        let ray = Ray::new(origin, direction);
        self.inner.intersect(ray)
    }
}