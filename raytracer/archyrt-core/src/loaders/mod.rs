use crate::{api::camera::Camera, intersectables::triangle::Triangle};

pub mod gltf;

pub trait Loader {
    type C: Camera;
    fn get_triangles(&self) -> Option<Vec<Triangle>>;
    fn get_camera(&self) -> Option<Self::C>;
}
