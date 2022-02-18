use std::fs::{read_dir, write};

use clap::{Arg, Command};

use asset::{Gizmo, GizmoVertex};
use gltf::mesh::util::ReadIndices;

fn main() {
    let matches = Command::new("agzm")
        .arg(Arg::new("in").required(true))
        .arg(Arg::new("out").required(true))
        .get_matches();

    let indir = matches.value_of("in").unwrap();
    let outdir = matches.value_of("out").unwrap();

    for entry in read_dir(indir).unwrap().flatten() {
        let (document, buffers, _) = gltf::import(entry.path()).unwrap();
        let mesh = document.meshes().next().unwrap();
        let primitive = mesh.primitives().next().unwrap();
        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));
        let vertices = reader
            .read_positions()
            .unwrap()
            .map(|p| GizmoVertex { position: p.into() })
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

        let out = Gizmo {
            vertices,
            triangles,
        }
        .encode()
        .unwrap();

        let path = format!(
            "{}/{}",
            outdir,
            entry.file_name().to_str().unwrap().replace("gltf", "agzm")
        );
        write(path, &out).unwrap();
    }
}
