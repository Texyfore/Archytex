use crate::{api::camera::Camera, intersectables::triangle::Triangle};

pub mod ascn;
pub mod gltf;
pub mod amdl;

pub trait Loader {
    type C: Camera;
    fn get_triangles(&self) -> &Vec<Triangle>;
    fn get_camera(&self) -> &Self::C;
}
