use asset_id::TextureID;
use cgmath::Vector3;
use mesh::Mesh;

pub trait Model<F: Face, S: Solid<F>> {
    fn solids(&self) -> &[S];
    fn generate_meshes(&self) -> Vec<SolidMesh>;
}

pub trait Solid<F: Face> {
    fn faces(&self) -> &[F];
    fn points(&self) -> &[Vector3<f32>];
}

pub trait Face {
    fn texture_id(&self) -> TextureID;
    fn points(&self) -> &[usize];
}

pub struct SolidMesh {
    pub texture_id: TextureID,
    pub mesh: Mesh,
}
