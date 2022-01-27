use crate::{utilities::{ray::{Intersectable, Ray, Intersection}, math::Vec3}, textures::{color_provider::ColorProvider, texture_repo::TextureRepository}, renderers::path_tracer::Material};

pub struct UnionIntersector<A: Intersectable, B: Intersectable>(pub A, pub B);

#[derive(Clone)]
pub enum UnionColorProvider<A: ColorProvider + Clone, B: ColorProvider + Clone>{
    A(A),
    B(B)
}

impl<A: ColorProvider + Clone, B: ColorProvider + Clone> ColorProvider for UnionColorProvider<A, B>{
    fn get_color<R: TextureRepository>(&self, repo: &R) -> Vec3 {
        match self{
            UnionColorProvider::A(a) => a.get_color(repo),
            UnionColorProvider::B(b) => b.get_color(repo),
        }
    }
    fn get_material(&self) -> Material {
        match self{
            UnionColorProvider::A(a) => a.get_material(),
            UnionColorProvider::B(b) => b.get_material(),
        }
    }
}



impl<A: Intersectable, B: Intersectable> Intersectable for UnionIntersector<A, B> where A::C: Clone, B::C: Clone{
    type C = UnionColorProvider<A::C, B::C>;

    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        let a = self.0.intersect(ray);
        let b = self.1.intersect(ray);
        match (a, b){
            (None, None) => None,
            (None, Some(b)) => {
                Some(b.with_color_provider(Self::C::B(b.get_color_provider())))
            },
            (Some(a), None) => {
                Some(a.with_color_provider(Self::C::A(a.get_color_provider())))
            }
            (Some(a), Some(b)) if b.get_distance_squared() < a.get_distance_squared() => {
                Some(b.with_color_provider(Self::C::B(b.get_color_provider())))
            }
            (Some(a), Some(_)) => {
                Some(a.with_color_provider(Self::C::A(a.get_color_provider())))
            }
        }
    }
}