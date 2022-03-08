use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use asset::{BoundingBox, PropMesh, PropVertex, TextureID};
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use image::{imageops::FilterType, DynamicImage, ImageFormat};

use thiserror::Error;

use crate::{
    indexed::{Entry, Indexed, PropEntry},
    require::Require,
};

pub fn save(root: &Path, indexed: &Indexed) {
    {
        let public = mkdir(root.join("out/public/textures"));
        let raytracer = mkdir(root.join("out/raytracer/textures"));

        for texture in &indexed.textures {
            let reader = BufReader::new(File::open(&texture.path).require());
            let base_image = image::load(reader, image::ImageFormat::Png).require();

            let file_name = format!("{}.png", texture.name);
            save_resized(&base_image, &raytracer.join(&file_name), 1024);
            save_resized(&base_image, &public.join(&file_name), 256);
        }
    }

    {
        let public = mkdir(root.join("out/public/props"));
        let raytracer = mkdir(root.join("out/raytracer/props"));

        for prop in &indexed.props {
            let amdl = gltf_to_amdl(prop, &indexed.textures);
            let buf = amdl.encode().unwrap();
            let file_name = format!("{}.amdl", prop.entry.name);
            fs::write(public.join(&file_name), &buf).require();
            fs::write(raytracer.join(&file_name), &buf).require();
        }
    }
}

fn mkdir(path: PathBuf) -> PathBuf {
    fs::create_dir_all(&path).require();
    path
}

fn save_resized(original: &DynamicImage, path: &Path, size: u32) {
    let image = original.resize_exact(size, size, FilterType::CatmullRom);
    image.save_with_format(path, ImageFormat::Png).require();
}

fn gltf_to_amdl(prop: &PropEntry, textures: &[Entry]) -> asset::Prop {
    let (document, buffers, _) = gltf::import(&prop.entry.path).unwrap();
    let mut box_min = [std::f32::INFINITY; 3];
    let mut box_max = [std::f32::NEG_INFINITY; 3];
    let mut meshes = Vec::new();

    for mesh in document.meshes() {
        let name = mesh.name().unwrap_or("Mesh");

        let texture = if let Some(mesh_textures) = &prop.textures {
            mesh_textures
                .get(name)
                .and_then(|t| textures.iter().find(|tt| &tt.name == t).map(|t| t.id))
        } else {
            textures
                .iter()
                .find(|t| t.name == prop.entry.name)
                .map(|t| t.id)
        }
        .ok_or(NoTextureError {
            model: &prop.entry.name,
            mesh: name,
        })
        .require();

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

#[derive(Debug, Error)]
#[error("No texture assigned for mesh `{mesh}` in `{model}`")]
struct NoTextureError<'a> {
    model: &'a str,
    mesh: &'a str,
}
