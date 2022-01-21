use std::collections::HashMap;

use rand_distr::{Distribution, UnitSphere};

use crate::{
    api::{
        camera::Camera,
        fragment_render::{FragmentContext, FragmentRender},
    },
    textures::{color_provider::ColorProvider, texture_repo::TextureRepository, TextureID},
    utilities::{
        math::{Vec2, Vec3},
        ray::{Intersectable, Intersection, Ray},
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse,
    Emissive { power: Vec3 },
}

impl Default for Material{
    fn default() -> Self {
        Self::Diffuse
    }
}

const EPSILON: f64 = 0.00001;

impl Material {
    pub fn reflect<C: ColorProvider>(self, intersection: Intersection<C>) -> Option<Ray> {
        match self {
            Material::Diffuse => {
                let p: [f64; 3] = UnitSphere.sample(&mut rand::thread_rng());
                let p = Vec3::new(p[0], p[1], p[2]);
                let p = if intersection.get_normal().dot(p) < 0.0 {
                    -p
                } else {
                    p
                };
                Some(Ray {
                    origin: intersection.get_pos(),
                    direction: p,
                })
            }
            Material::Emissive { power: _ } => None,
        }
    }
    pub fn color<C: ColorProvider, R: TextureRepository>(
        self,
        intersection: &Intersection<C>,
        repo: &R,
        emissive: &mut Vec3,
        diffusive: &mut Vec3,
    ) {
        match self {
            Material::Diffuse => {
                (*diffusive) *= intersection.get_color(repo);
            }
            Material::Emissive { power } => {
                (*emissive) += intersection.get_color(repo) * power;
            }
        }
    }
}

pub struct PathTracer<T: Camera, K: Intersectable> {
    pub camera: T,
    pub object: K,
    pub bounces: usize,
}

impl<T: Camera, K: Intersectable> FragmentRender for PathTracer<T, K> {
    fn render_fragment<R: TextureRepository>(&self, ctx: &FragmentContext<R>, pos: Vec2) -> Vec3 {
        let mut ray = self.camera.get_ray(ctx, pos);
        let mut emissive = Vec3::default();
        let mut diffusive = Vec3::from_single(1.0);
        for _ in 0..self.bounces {
            match self.object.intersect(ray) {
                Some(intersection) => {
                    let normal = intersection.get_normal();
                    let material = intersection.get_material(); //TODO: Handle materials
                    material.color(&intersection, &ctx.repo, &mut emissive, &mut diffusive);
                    ray = match material.reflect(intersection) {
                        Some(ray) => {
                            let mut ray = ray;
                            ray.origin += normal * EPSILON;
                            ray
                        }
                        None => break,
                    };
                }
                None => {
                    //Make background yellow
                    emissive += (diffusive
                        * Vec3::new(
                            0xd4 as f64 / 255.0,
                            0xe6 as f64 / 255.0,
                            0xff as f64 / 255.0,
                        ))
                        * 1.0;
                    break;
                }
            }
        }
        emissive
    }
}
