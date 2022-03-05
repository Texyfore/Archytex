use asset::{BoundingBox, PropMesh, PropVertex, TextureID};
use gltf::mesh::util::{ReadIndices, ReadTexCoords};

use crate::report::bail;

pub fn parse_gltf(prop: &crate::props::Prop) -> asset::Prop {
    let (document, buffers, _) = gltf::import(&prop.source).unwrap();
    let mut box_min = [std::f32::INFINITY; 3];
    let mut box_max = [std::f32::NEG_INFINITY; 3];
    let mut meshes = Vec::new();

    for mesh in document.meshes() {
        let name = match mesh.name() {
            Some(name) => name,
            None => bail(&format!("mesh has no name in `{}`", prop.name)),
        };

        if !prop.textures.contains_key(name) {
            bail(&format!(
                "no texture for mesh `{}` in `{}`",
                name, prop.name
            ));
        }

        let primitive = mesh.primitives().next().unwrap();
        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

        let positions = reader.read_positions().unwrap().collect::<Vec<_>>();

        positions.iter().for_each(|p| {
            for i in 0..3 {
                if p[i] < box_min[i] {
                    box_min[i] = p[i];
                }

                if p[i] > box_max[i] {
                    box_max[i] = p[i];
                }
            }
        });

        let normals = reader.read_normals().unwrap();

        let texcoords = if let ReadTexCoords::F32(texcoords) = reader.read_tex_coords(0).unwrap() {
            texcoords
        } else {
            panic!("Bad texcoords");
        };

        let vertices = positions
            .into_iter()
            .zip(normals.into_iter())
            .zip(texcoords.into_iter())
            .map(|((p, n), t)| PropVertex {
                position: p.into(),
                normal: n.into(),
                texcoord: t.into(),
            })
            .collect::<Vec<_>>();

        let indices = if let ReadIndices::U16(indices) = reader.read_indices().unwrap() {
            indices
        } else {
            panic!("Bad indices")
        }
        .collect::<Vec<_>>();

        let triangles = indices
            .chunks_exact(3)
            .map(|w| w.try_into().unwrap())
            .collect();

        meshes.push(PropMesh {
            texture: TextureID(*prop.textures.get(name).unwrap()),
            vertices,
            triangles,
        });
    }

    asset::Prop {
        bounds: BoundingBox {
            min: box_min.into(),
            max: box_max.into(),
        },
        meshes,
    }
}
