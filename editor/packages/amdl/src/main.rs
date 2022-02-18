use std::fs::{self, read_dir, write};

use clap::{Command, Arg};

use asset::{BoundingBox, Prop, PropMesh, PropVertex, TextureID};
use gltf::mesh::util::{ReadIndices, ReadTexCoords};

fn main() {
    let matches = Command::new("agzm")
        .arg(Arg::new("in").required(true))
        .arg(Arg::new("out").required(true))
        .get_matches();

    let indir = matches.value_of("in").unwrap();
    let outdir = matches.value_of("out").unwrap();

    for entry in read_dir(indir).unwrap().flatten() {
        if entry.file_name().to_str().unwrap().contains("gltf") {
            let (document, buffers, _) = gltf::import(entry.path()).unwrap();
            let textures = fs::read_to_string(
                entry
                    .path()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace("gltf", "att"),
            )
            .unwrap()
            .split(' ')
            .map(|str| str.parse().unwrap())
            .collect::<Vec<u32>>();

            let mut box_min = [std::f32::INFINITY; 3];
            let mut box_max = [std::f32::NEG_INFINITY; 3];
            let mut meshes = Vec::new();

            for (i, mesh) in document.meshes().enumerate() {
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

                let texcoords =
                    if let ReadTexCoords::F32(texcoords) = reader.read_tex_coords(0).unwrap() {
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
                    texture: TextureID(textures[i]),
                    vertices,
                    triangles,
                });
            }

            let out = Prop {
                bounds: BoundingBox {
                    min: box_min.into(),
                    max: box_max.into(),
                },
                meshes,
            }
            .encode()
            .unwrap();

            let path = format!(
                "{}/{}",
                outdir,
                entry.file_name().to_str().unwrap().replace("gltf", "amdl")
            );
            write(path, &out).unwrap();
        }
    }
}
