use crate::utilities::{math::Vec2, ray::Ray};

use super::fragment_render::FragmentContext;

pub trait Camera{
    fn get_ray(&self, ctx: &FragmentContext, pos: Vec2) -> Ray;
}