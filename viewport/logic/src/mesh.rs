use tools::{
    gfx::{Graphics, Mesh, Tri, Vert},
    math::Vec3,
};

#[derive(Default)]
pub struct MeshBuilder {
    verts: Vec<Vert>,
    tris: Vec<Tri>,
}

impl MeshBuilder {
    pub fn push_quad(&mut self, points: [Vec3; 4], normal: Vec3) {
        let t0 = self.verts.len() as u16;

        self.tris.push(Tri {
            idx: [t0, t0 + 1, t0 + 2],
        });

        self.tris.push(Tri {
            idx: [t0, t0 + 2, t0 + 3],
        });

        // TODO: Dirty hack, remove ASAP
        const uv: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

        for (point, i) in points.iter().copied().zip(0..points.len()) {
            self.verts.push(Vert {
                pos: point.into(),
                normal: normal.into(),
                uv: uv[i],
            })
        }
    }

    pub fn build(self, gfx: &Graphics) -> Mesh {
        Mesh::new(gfx, &self.verts, &self.tris)
    }
}
