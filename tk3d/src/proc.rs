use std::collections::HashMap;

use crate::{
    ascn::Model,
    error::MeshGenError,
    math::{vec2, InnerSpace},
    Mesh, TextureID, Triangle, Vertex,
};

impl Model {
    pub fn generate_mesh(&self) -> Result<Vec<Mesh>, MeshGenError> {
        let mut batches = HashMap::<u32, (Vec<Vertex>, Vec<Triangle>)>::new();

        for (_, face) in &self.faces {
            if face.points.iter().any(|point_id| {
                !self
                    .points
                    .iter()
                    .any(|(other_point_id, _)| point_id == other_point_id)
            }) {
                return Err(MeshGenError);
            }

            let points = face.points.map(|point_id| {
                self.points
                    .iter()
                    .find(|(other_point_id, _)| point_id == *other_point_id)
                    .map(|(_, point)| point.position)
                    .unwrap()
            });

            let normal = {
                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                edge0.cross(edge1).normalize()
            };

            let (vertices, triangles) = batches.entry(face.texture.0).or_default();
            let t0 = triangles.len() as u16;

            vertices.append(
                &mut points
                    .map(|point| {
                        let texcoord = if normal.x.abs() > normal.y.abs() {
                            if normal.x.abs() > normal.z.abs() {
                                vec2(point.y, point.z)
                            } else {
                                vec2(point.x, point.y)
                            }
                        } else if normal.y.abs() > normal.z.abs() {
                            vec2(point.x, point.z)
                        } else {
                            vec2(point.x, point.y)
                        } / 4.0;

                        Vertex {
                            position: point,
                            normal,
                            texcoord,
                        }
                    })
                    .to_vec(),
            );

            triangles.push(Triangle {
                indices: [t0, t0 + 1, t0 + 2],
            });

            triangles.push(Triangle {
                indices: [t0, t0 + 2, t0 + 3],
            });
        }

        Ok(batches
            .into_iter()
            .map(|(texture_id, (vertices, triangles))| Mesh {
                texture: TextureID(texture_id),
                vertices,
                triangles,
            })
            .collect())
    }
}
