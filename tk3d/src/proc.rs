use crate::{
    ascn::Model,
    error::MeshGenError,
    math::{vec2, InnerSpace},
    Mesh, Triangle, Vertex,
};

pub struct SolidMesh {
    pub faces: [Mesh; 6],
}

impl Model {
    pub fn generate_mesh(&self) -> Result<Vec<SolidMesh>, MeshGenError> {
        let mut solid_meshes = Vec::new();

        for (_, solid) in &self.solids {
            let mut face_meshes = Vec::new();

            for face_id in solid.faces {
                let face = &self
                    .faces
                    .iter()
                    .find(|(other_face_id, _)| face_id == *other_face_id)
                    .ok_or(MeshGenError::BadFaceRef(face_id))?
                    .1;

                for point_id in face.points {
                    if !self
                        .points
                        .iter()
                        .any(|(other_point_id, _)| point_id == *other_point_id)
                    {
                        return Err(MeshGenError::BadPointRef(point_id));
                    }
                }

                let points = face.points.map(|point_id| {
                    self.points
                        .iter()
                        .find(|(other_point_id, _)| point_id == *other_point_id)
                        .map(|(_, point)| point.position)
                        .unwrap()
                });

                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                let normal = edge0.cross(edge1).normalize();

                face_meshes.push(Mesh {
                    texture: face.texture,
                    vertices: points
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
                            };

                            Vertex {
                                position: point,
                                normal,
                                texcoord,
                            }
                        })
                        .to_vec(),
                    triangles: vec![
                        Triangle { indices: [0, 1, 2] },
                        Triangle { indices: [0, 2, 3] },
                    ],
                });
            }

            solid_meshes.push(SolidMesh {
                faces: face_meshes.try_into().ok().unwrap(),
            });
        }

        Ok(solid_meshes)
    }
}
