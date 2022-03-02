use std::{f64::consts::PI};

use rand_distr::{Distribution, UnitSphere};

use crate::{
    api::{
        camera::Camera,
        fragment_render::{FragmentContext, FragmentRender},
    },
    textures::{
        color_provider::ColorProvider,
        samplers::{nearest::NearestSampler, TextureSampler},
        texture_repo::TextureRepository,
        TextureID,
    },
    utilities::{
        math::{Vec2, Vec3},
        ray::{Intersectable, Intersection, Ray},
    },
    vector,
};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse,
    Emissive { power: f64 },
    DiffuseAndEmissive {emissive_texture: TextureID},
}

impl Default for Material {
    fn default() -> Self {
        Self::Diffuse
    }
}

const EPSILON: f64 = 0.00001;

impl Material {
    pub fn reflect<C: ColorProvider>(self, intersection: Intersection<C>) -> Option<Ray> {
        match self {
            Material::Diffuse | Material::DiffuseAndEmissive{emissive_texture: _} => {
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
            Material::Emissive { power: _ } => None
        }
    }
    pub fn color<C: ColorProvider>(
        self,
        intersection: &Intersection<C>,
        repo: &TextureRepository,
        emissive: &mut Vec3,
        diffusive: &mut Vec3,
    ) {
        match self {
            Material::Diffuse => {
                (*diffusive) *= intersection.get_color(repo);
            }
            Material::Emissive { power } => {
                (*emissive) += intersection.get_color(repo) * power * (*diffusive);
            }
            Material::DiffuseAndEmissive { emissive_texture } => {
                (*emissive) += intersection.ref_color_provider().sample(repo, emissive_texture) * 50.0 * (*diffusive);
                (*diffusive) *= intersection.get_color(repo);
                
            },
        }
    }
}

pub struct PathTracer<T: Camera, K: Intersectable> {
    pub camera: T,
    pub object: K,
    pub bounces: usize,
    pub skybox: Option<TextureID>,
}

impl<T: Camera, K: Intersectable> FragmentRender for PathTracer<T, K> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        let mut ray = self.camera.get_ray(ctx, pos);
        let mut emissive = Vec3::default();
        let mut diffusive = Vec3::from_single(1.0);
        for bounce in 0..self.bounces {
            match self.object.intersect(ray) {
                Some(intersection) => {
                    let normal = intersection.get_normal();
                    let material = intersection.get_material();
                    material.color(&intersection, ctx.repo, &mut emissive, &mut diffusive);
                    ray = match material.reflect(intersection) {
                        Some(ray) => {
                            diffusive *= ray.direction.dot(normal);
                            let mut ray = ray;
                            ray.origin += normal * EPSILON;
                            ray
                        }
                        None => break,
                    };
                }
                None => {
                    //The sky is blue
                    let sky_color = match self.skybox {
                        //Skybox color
                        Some(skybox) => { 
                            let sampler = NearestSampler {};
                            let texture = ctx.repo.get(skybox).unwrap();
                            let longitude = ray.direction.x().atan2(ray.direction.z());
                            let latitude = -(ray.direction.y() / ray.direction.length()).asin();
                            let longitude = (longitude / PI + 1.0) * 0.5;
                            let latitude = (latitude / (PI / 2.0) + 1.0) * 0.5;
                            let res = sampler.sample(texture, vector![longitude, latitude]);
                            if bounce == 0{
                                res
                            }else{
                                res * 3.0
                            }
                        }
                        //Default skybox color
                        _ => {
                            Vec3::default()
                        }
                    };
                    emissive += diffusive * sky_color;
                    break;
                }
            }
        }
        emissive
    }
}
