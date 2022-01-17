use std::collections::HashMap;

use asset_id::TextureID;
use cgmath::{vec2, InnerSpace, Vector3};
use mesh::{Mesh, Vertex};
use thiserror::Error;

pub trait Model<F: Face, S: Solid<F>> {
    fn solids(&self) -> &[S];
}

pub trait Solid<F: Face> {
    fn faces(&self) -> &[F];
    fn points(&self) -> &[Vector3<f32>; 8];
}

pub trait Face {
    fn texture_id(&self) -> TextureID;
    fn points(&self) -> &[usize; 4];
}

pub struct SolidMesh {
    pub texture_id: TextureID,
    pub mesh: Mesh,
}

pub fn mesh_gen<F: Face, S: Solid<F>, M: Model<F, S>>(
    model: &M,
) -> Result<Vec<SolidMesh>, MeshGenError> {
    let mut batches = HashMap::<TextureID, Mesh>::new();

    for solid in model.solids() {
        for face in solid.faces() {
            let points = if let (Some(p0), Some(p1), Some(p2), Some(p3)) = (
                solid.points().get(face.points()[0]),
                solid.points().get(face.points()[1]),
                solid.points().get(face.points()[2]),
                solid.points().get(face.points()[3]),
            ) {
                [*p0, *p1, *p2, *p3]
            } else {
                return Err(MeshGenError);
            };

            let normal = {
                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                edge0.cross(edge1).normalize()
            };

            let mesh = batches.entry(face.texture_id()).or_default();
            let t0 = mesh.vertices.len() as u16;

            mesh.vertices.append(
                &mut points
                    .into_iter()
                    .map(|position| {
                        let texcoord = if normal.x.abs() > normal.y.abs() {
                            if normal.x.abs() > normal.z.abs() {
                                vec2(position.y, position.z)
                            } else {
                                vec2(position.x, position.y)
                            }
                        } else if normal.y.abs() > normal.z.abs() {
                            vec2(position.x, position.z)
                        } else {
                            vec2(position.x, position.y)
                        } / 4.0;

                        Vertex {
                            position,
                            normal,
                            texcoord,
                        }
                    })
                    .collect(),
            );

            mesh.triangles.push([t0, t0 + 1, t0 + 2]);
            mesh.triangles.push([t0, t0 + 2, t0 + 3]);
        }
    }

    todo!()
}

#[derive(Error, Debug)]
#[error("couldn't generate mesh: broken model")]
pub struct MeshGenError;
