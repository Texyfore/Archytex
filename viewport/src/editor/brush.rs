use std::{collections::HashMap, rc::Rc};

use cgmath::{vec3, ElementWise, InnerSpace, Vector3};

use crate::{
    input::InputMapper,
    render::{SolidBatch, SolidFactory, SolidVertex, TextureID, WorldPass},
};

use super::ActionBinding::*;

#[derive(Default)]
pub struct BrushBank {
    brushes: Vec<Brush>,
    batches: Vec<(TextureID, Rc<SolidBatch>)>,
}

struct Brush {
    points: [Point; 8],
    faces: [Face; 6],
    selected: bool,
}

struct Point {
    position: Vector3<f32>,
    selected: bool,
}

struct Face {
    quad: [u16; 4],
    texture: TextureID,
    selected: bool,
}

impl BrushBank {
    pub fn process(
        &mut self,
        input: &InputMapper,
        world_pass: &mut WorldPass,
        solid_factory: &SolidFactory,
    ) {
        if input.is_active_once(AddBrush) {
            self.brushes
                .push(Brush::cuboid(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0)));
            self.rebuild(&solid_factory);
        }

        world_pass.solid_batches = self.batches.clone();
    }

    fn rebuild(&mut self, factory: &SolidFactory) {
        let mut batches = HashMap::new();
        for brush in &self.brushes {
            for face in &brush.faces {
                if !batches.contains_key(&face.texture) {
                    batches.insert(face.texture, (Vec::new(), Vec::new()));
                }

                let (vertices, triangles) = batches.get_mut(&face.texture).unwrap();

                let t0 = vertices.len() as u16;
                triangles.push([t0 + 0, t0 + 1, t0 + 2]);
                triangles.push([t0 + 0, t0 + 2, t0 + 3]);

                let points = face.quad.map(|i| brush.points[i as usize].position);
                let edge0 = points[1] - points[0];
                let edge1 = points[3] - points[0];
                let normal = (edge0.cross(edge1)).normalize();

                for point in points {
                    vertices.push(SolidVertex {
                        position: point.into(),
                        normal: normal.into(),
                        texcoord: [0.0, 0.0],
                        color: [1.0; 4],
                    });
                }
            }
        }

        self.batches = batches
            .iter()
            .map(|(t, (v, i))| (*t, factory.create(&v, &i)))
            .collect();
    }
}

impl Brush {
    fn cuboid(origin: Vector3<f32>, extent: Vector3<f32>) -> Self {
        let points = [
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            vec3(1.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 1.0, 0.0),
            vec3(1.0, 1.0, 1.0),
            vec3(0.0, 1.0, 1.0),
        ]
        .map(|p| Point {
            position: origin + p.mul_element_wise(extent),
            selected: false,
        });

        let faces = [
            [5, 6, 2, 1],
            [7, 4, 0, 3],
            [7, 6, 5, 4],
            [2, 3, 0, 1],
            [6, 7, 3, 2],
            [4, 5, 1, 0],
        ]
        .map(|f| Face {
            quad: f,
            texture: 0,
            selected: false,
        });

        Self {
            points,
            faces,
            selected: false,
        }
    }
}
