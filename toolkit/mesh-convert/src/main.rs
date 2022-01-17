use std::fs::write;

use anyhow::{bail, ensure, Context, Result};
use asset_id::TextureID;
use clap::{App, Arg, ArgMatches};
use gltf::mesh::{
    util::{ReadIndices, ReadTexCoords},
    Mode,
};

fn main() {
    let matches = App::new("meshtool")
        .subcommand(
            App::new("build-amdl")
                .arg(Arg::new("src").required(true))
                .arg(Arg::new("dst").required(true))
                .arg(Arg::new("texture").required(true)),
        )
        .subcommand(
            App::new("build-agzm")
                .arg(Arg::new("src").required(true))
                .arg(Arg::new("dst").required(true)),
        )
        .get_matches();

    if let Some((subcommand, matches)) = matches.subcommand() {
        if let Err(err) = run(subcommand, matches) {
            println!("Error: {}\n\nCaused by:\n    {}", err, err.root_cause());
        }
    }
}

fn run(subcommand: &str, matches: &ArgMatches) -> Result<()> {
    let dst = matches.value_of("dst").unwrap();

    let buf = match subcommand {
        "build-amdl" => build_amdl(matches)?,
        "build-agzm" => build_agzm(matches)?,
        _ => unreachable!(),
    };

    write(dst, &buf).context("couldn't write to destination file")?;
    Ok(())
}

fn build_amdl(matches: &ArgMatches) -> Result<Vec<u8>> {
    let src = matches.value_of("src").unwrap();
    let texture: u32 = matches
        .value_of("texture")
        .unwrap()
        .parse()
        .context("couldn't parse texture id")?;

    let (document, buffers, _) = gltf::import(&src).context("couldn't import gltf file")?;
    let mesh = document.meshes().next().context("couldn't get mesh")?;

    let primitive = mesh
        .primitives()
        .next()
        .context("couldn't get mesh primitives")?;

    ensure!(
        primitive.mode() == Mode::Triangles,
        "mesh primitives aren't triangles"
    );

    let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

    let mut box_min = [std::f32::INFINITY; 3];
    let mut box_max = [std::f32::NEG_INFINITY; 3];

    let positions = reader
        .read_positions()
        .context("couldn't read positions")?
        .collect::<Vec<_>>();

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

    let normals = reader
        .read_normals()
        .context("couldn't read normals")?
        .collect::<Vec<_>>();

    let texcoords = if let ReadTexCoords::F32(texcoords) = reader
        .read_tex_coords(0)
        .context("couldn't read texcoords")?
    {
        texcoords
    } else {
        bail!("bad texcoords");
    }
    .collect::<Vec<_>>();

    ensure!(positions.len() == normals.len());
    ensure!(normals.len() == texcoords.len());

    let vertices = positions
        .into_iter()
        .zip(normals.into_iter())
        .zip(texcoords.into_iter())
        .map(|((p, n), t)| mesh::Vertex {
            position: p.into(),
            normal: n.into(),
            texcoord: t.into(),
        })
        .collect::<Vec<_>>();

    let indices = if let ReadIndices::U16(indices) =
        reader.read_indices().context("couldn't read indices")?
    {
        indices
    } else {
        bail!("wrong indices")
    }
    .collect::<Vec<_>>();

    let triangles = indices
        .chunks_exact(3)
        .map(|w| w.try_into().unwrap())
        .collect();

    let model = amdl::Model {
        texture_id: TextureID(texture),
        bounding_box: amdl::BoundingBox {
            min: box_min.into(),
            max: box_max.into(),
        },
        mesh: mesh::Mesh {
            vertices,
            triangles,
        },
    };

    model.encode().context("couldn't encode model")
}

fn build_agzm(matches: &ArgMatches) -> Result<Vec<u8>> {
    let src = matches.value_of("src").unwrap();

    let (document, buffers, _) = gltf::import(&src).context("couldn't import gltf file")?;
    let mesh = document.meshes().next().context("couldn't get mesh")?;

    let primitive = mesh
        .primitives()
        .next()
        .context("couldn't get mesh primitives")?;

    ensure!(
        primitive.mode() == Mode::Triangles,
        "mesh primitives aren't triangles"
    );

    let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

    let vertices = reader
        .read_positions()
        .context("couldn't read positions")?
        .map(|p| p.into())
        .collect::<Vec<_>>();

    let indices = if let ReadIndices::U16(indices) =
        reader.read_indices().context("couldn't read indices")?
    {
        indices
    } else {
        bail!("wrong indices")
    }
    .collect::<Vec<_>>();

    let triangles = indices
        .chunks_exact(3)
        .map(|w| w.try_into().unwrap())
        .collect();

    let gizmo = agzm::Mesh {
        vertices,
        triangles,
    };

    gizmo.encode().context("couldn't encode mesh")
}
