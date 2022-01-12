use crate::{api::camera::Camera, intersectables::triangle::Triangle, textures::texture_repo::TextureRepository};

pub mod gltf;
pub mod amdl;

pub trait Loader {
    type C: Camera;
    type Tex: TextureRepository;
    fn get_triangles(&self) -> &Vec<Triangle>;
    fn get_camera(&self) -> &Self::C;
    fn get_textures(&self) -> &Self::Tex;
}
