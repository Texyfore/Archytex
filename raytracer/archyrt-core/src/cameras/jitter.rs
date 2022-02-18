use rand::thread_rng;
use rand_distr::{Uniform, Distribution};

use crate::utilities::math::Matrix3x3;
use crate::{
    api::{camera::Camera, fragment_render::FragmentContext},
    matrix,
    utilities::{
        math::{Matrix, Vec2, Vec3},
        ray::Ray,
    },
    vector,
};

#[derive(Debug, Clone)]
pub struct JitterCamera<C: Camera> {
    pub inner: C,
    pub x_dist: Uniform<f64>,
    pub y_dist: Uniform<f64>,
}

impl<C: Camera> JitterCamera<C> {
    pub fn new(inner: C, width: usize, height: usize) -> Self {
        let width = 1.0/(width as f64)*0.5;
        let height = 1.0/(height as f64)*0.5;
        let x_dist = Uniform::new_inclusive(-width, width);
        let y_dist = Uniform::new_inclusive(-height, height);
        Self {
            inner, x_dist, y_dist
        }
    }
}

impl<C: Camera> Camera for JitterCamera<C> {
    fn get_ray(&self, ctx: &FragmentContext, pos: Vec2) -> Ray {
        let mut rng = thread_rng();
        let x: f64 = self.x_dist.sample(&mut rng);
        let y: f64 = self.y_dist.sample(&mut rng);
        let jitter = vector![x, y];
        self.inner.get_ray(ctx, pos + jitter)
    }
}
