use crate::{
    api::{
        camera::Camera,
        fragment_render::{FragmentContext, FragmentRender},
    },
    utilities::{
        math::{Vec2, Vec3},
        ray::Intersectable,
    },
};

pub struct AlbedoRenderer<T: Camera, K: Intersectable> {
    pub camera: T,
    pub object: K,
}

impl<T: Camera, K: Intersectable> FragmentRender for AlbedoRenderer<T, K> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        let ray = self.camera.get_ray(ctx, pos);
        match self.object.intersect(ray) {
            Some(intersection) => intersection.get_color(ctx.repo),
            None => Vec3::default(),
        }
    }
}
