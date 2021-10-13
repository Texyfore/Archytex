use tools::{
    gfx::{Graphics, LineMesh, LineVert},
    math::{Mat4, SquareMatrix},
};

pub struct Grid {
    mesh: LineMesh,
}

impl Grid {
    pub fn new(lines: i32, gfx: &Graphics) -> Self {
        let mut verts = Vec::new();
        let width = lines as f32;

        for i in -lines..=lines {
            verts.push(LineVert {
                pos: [i as f32, 0.0, -width],
            });
            verts.push(LineVert {
                pos: [i as f32, 0.0, width],
            });

            verts.push(LineVert {
                pos: [-width, 0.0, i as f32],
            });
            verts.push(LineVert {
                pos: [width, 0.0, i as f32],
            });
        }

        Self {
            mesh: LineMesh::new(gfx, &verts),
        }
    }

    pub fn draw(&self, gfx: &Graphics) {
        self.mesh.draw(gfx, Mat4::identity());
    }
}
