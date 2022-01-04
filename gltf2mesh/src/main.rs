use gltf::mesh::{util::ReadIndices, Mode};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let (document, buffers, _) = gltf::import(&path).expect("Not a gltf file");
        let mesh = document.meshes().next().expect("No mesh");
        let primitive = mesh.primitives().next().expect("No primitive");
        assert_eq!(primitive.mode(), Mode::Triangles);

        let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

        let positions = reader
            .read_positions()
            .expect("No positions")
            .map(|p| mdl::Vector3 {
                x: p[0],
                y: p[1],
                z: p[2],
            })
            .collect::<Vec<_>>();

        let normals = reader
            .read_normals()
            .expect("No normals")
            .map(|n| mdl::Vector3 {
                x: n[0],
                y: n[1],
                z: n[2],
            })
            .collect::<Vec<_>>();

        let texcoords = reader
            .read_positions()
            .expect("No texcoords")
            .map(|t| mdl::Vector2 { x: t[0], y: t[1] })
            .collect::<Vec<_>>();

        assert_eq!(positions.len(), normals.len());
        assert_eq!(normals.len(), texcoords.len());

        let vertices = positions
            .into_iter()
            .zip(normals.into_iter())
            .zip(texcoords.into_iter())
            .map(|((p, n), t)| mdl::Vertex {
                position: p,
                normal: n,
                texcoord: t,
            })
            .collect::<Vec<_>>();

        let indices = if let ReadIndices::U16(indices) = reader.read_indices().expect("No indices")
        {
            indices
        } else {
            panic!("Wrong indices");
        }
        .collect::<Vec<_>>();

        let triangles = indices
            .windows(3)
            .map(|w| mdl::Triangle {
                indices: w.try_into().expect("Not a triangle"),
            })
            .collect();

        let mesh = mdl::Mesh {
            vertices,
            triangles,
        };

        std::fs::write(path.replace("gltf", "amdl"), mesh.encode().unwrap())
            .expect("Couldn't save");
    }
}
