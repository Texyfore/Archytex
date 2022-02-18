use crate::{
    api::{
        camera::Camera,
        fragment_render::{FragmentContext, FragmentRender},
    },
    cameras::perspective::PerspectiveCamera,
    intersectables::sphere::Sphere,
    utilities::{
        math::{Vec2, Vec3},
        ray::Intersectable,
    },
};

pub struct BasicRenderer<T: Camera, K: Intersectable> {
    pub camera: T,
    pub object: K,
    pub lamp: Vec3,
}

impl Default for BasicRenderer<PerspectiveCamera, Sphere> {
    fn default() -> Self {
        Self {
            camera: PerspectiveCamera::new(Vec3::default(), Vec3::new(0.0, 0.0, 1.0), 1.0),
            object: Sphere {
                origin: Vec3::new(0.0, 0.0, 3.0),
                radius: 0.5,
                color: Vec3::new(0.0, 1.0, 0.0),
                ..Default::default()
            },
            lamp: Vec3::new(5.0, 5.0, 0.0),
        }
    }
}

impl<T: Camera, K: Intersectable> FragmentRender for BasicRenderer<T, K> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        let ray = self.camera.get_ray(ctx, pos);
        match self.object.intersect(ray) {
            Some(intersection) => {
                let pos = intersection.get_pos();
                let normal = intersection.get_normal();
                let base = intersection.get_color(ctx.repo);
                let lamp_direction = self.lamp - pos;
                let shadow = lamp_direction.dot(normal) / lamp_direction.length();
                let shadow = shadow.clamp(0.0, 1.0);

                base * shadow
            }
            None => Vec3::new(1.0, 0.0, 220.0 / 255.0),
        }
    }
}
