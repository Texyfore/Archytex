use crate::utilities::math::Vec3;

pub trait ColorProvider{
    fn get_color(&self) -> Vec3;
}

#[derive(Default)]
pub struct SolidColor(pub Vec3);

impl ColorProvider for SolidColor {
    fn get_color(&self) -> Vec3 {
        self.0
    }
}