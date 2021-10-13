use tools::{
    gfx::{Color, Graphics, LineMesh, LineVert},
    math::{Mat4, SquareMatrix},
};

pub struct Grid {
    mesh: LineMesh,
}

impl Grid {
    pub fn new(lines: i32, color: Color, gfx: &Graphics) -> Self {
        let width = lines as f32;
        let color = color.into();

        let mut verts = Vec::new();

        for i in -lines..=lines {
            verts.push(LineVert {
                pos: [i as f32, 0.0, -width],
                color,
            });
            verts.push(LineVert {
                pos: [i as f32, 0.0, width],
                color,
            });

            verts.push(LineVert {
                pos: [-width, 0.0, i as f32],
                color,
            });
            verts.push(LineVert {
                pos: [width, 0.0, i as f32],
                color,
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
