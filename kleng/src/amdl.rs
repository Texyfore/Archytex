use std::{collections::HashMap, path::Path};

use asset::{BoundingBox, PropMesh, PropVertex, TextureID};
use gltf::mesh::util::{ReadIndices, ReadTexCoords};

use crate::{
    input::{Prop, Texture},
    Indexed,
};

pub fn gltf_to_amdl(
    root: &Path,
    prop: &Prop,
    textures: &HashMap<String, Indexed<Texture>>,
) -> asset::Prop {
    let (document, buffers, _) = gltf::import(root.join("props").join(&prop.source)).unwrap();
    let mut box_min = [std::f32::INFINITY; 3];
    let mut box_max = [std::f32::NEG_INFINITY; 3];
    let mut meshes = Vec::new();

    for node in document.nodes() {
        let name = node.name().unwrap();
        let mesh = node.mesh().unwrap();

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

        let indices = match reader.read_indices().unwrap() {
            ReadIndices::U8(indices) => indices.into_iter().map(|i| i as u16).collect::<Vec<_>>(),
            ReadIndices::U16(indices) => indices.collect::<Vec<_>>(),
            ReadIndices::U32(indices) => indices.into_iter().map(|i| i as u16).collect::<Vec<_>>(),
        };

        let triangles = indices
            .chunks_exact(3)
            .map(|w| w.try_into().unwrap())
            .collect();

        let texture = textures.get(prop.textures.get(name).unwrap()).unwrap().id;

        meshes.push(PropMesh {
            texture: TextureID(texture),
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
