use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use cgmath::{vec3, ElementWise, InnerSpace, Vector3};

use crate::{
    input::InputMapper,
    math::{Intersects, Ray, Triangle},
    render::{SolidBatch, SolidFactory, SolidVertex, TextureID, WorldPass},
};

use super::{camera::WorldCamera, config::HIGHLIGHT_COLOR, ActionBinding::*, EditMode};

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

struct RaycastResult {
    brush_id: usize,
    face_id: usize,
}

impl BrushBank {
    pub fn process(
        &mut self,
        mode: &EditMode,
        input: &InputMapper,
        world_pass: &mut WorldPass,
        solid_factory: &SolidFactory,
        camera: &WorldCamera,
    ) {
        match mode {
            EditMode::Brush => {
                if input.is_active_once(Select) {
                    let mut needs_rebuild = false;

                    if !input.is_active(EnableMultiSelect) {
                        for brush in &mut self.brushes {
                            brush.selected = false;
                        }
                        needs_rebuild = true;
                    }

                    if let Some(hit) = self.raycast(camera.screen_ray(input.mouse_pos())) {
                        let selected = &mut self.brushes[hit.brush_id].selected;
                        *selected = !*selected;
                        needs_rebuild = true;
                    }

                    if needs_rebuild {
                        self.rebuild(&solid_factory);
                    }
                }

                if input.is_active_once(AddBrush) && input.is_active(EnableAddBrush) {
                    self.brushes
                        .push(Brush::cuboid(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0)));
                    self.rebuild(&solid_factory);
                }
            }
            EditMode::Face => {
                if input.is_active_once(Select) {
                    let mut needs_rebuild = false;

                    if !input.is_active(EnableMultiSelect) {
                        for brush in &mut self.brushes {
                            for face in &mut brush.faces {
                                face.selected = false;
                            }
                        }
                        needs_rebuild = true;
                    }

                    if let Some(hit) = self.raycast(camera.screen_ray(input.mouse_pos())) {
                        let selected = &mut self.brushes[hit.brush_id].faces[hit.face_id].selected;
                        *selected = !*selected;
                        needs_rebuild = true;
                    }

                    if needs_rebuild {
                        self.rebuild(&solid_factory);
                    }
                }
            }
            EditMode::Vertex => {}
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

                let color = if face.selected || brush.selected {
                    HIGHLIGHT_COLOR
                } else {
                    [1.0; 4]
                };

                for point in points {
                    vertices.push(SolidVertex {
                        position: point.into(),
                        normal: normal.into(),
                        texcoord: [0.0, 0.0],
                        color,
                    });
                }
            }
        }

        self.batches = batches
            .iter()
            .map(|(t, (v, i))| (*t, factory.create(&v, &i)))
            .collect();
    }

    fn raycast(&self, ray: Ray) -> Option<RaycastResult> {
        let mut hits = Vec::new();
        for (i, brush) in self.brushes.iter().enumerate() {
            for (j, face) in brush.faces.iter().enumerate() {
                let tri0 = Triangle {
                    a: brush.points[face.quad[0] as usize].position,
                    b: brush.points[face.quad[1] as usize].position,
                    c: brush.points[face.quad[2] as usize].position,
                };

                let tri1 = Triangle {
                    a: brush.points[face.quad[0] as usize].position,
                    b: brush.points[face.quad[2] as usize].position,
                    c: brush.points[face.quad[3] as usize].position,
                };

                let center = tri0.a * 0.25 + tri0.b * 0.25 + tri0.c * 0.25 + tri1.c * 0.25;

                if ray.intersects(&tri0) || ray.intersects(&tri1) {
                    hits.push((i, j, center));
                }
            }
        }

        hits.sort_unstable_by(|(_, _, c1), (_, _, c2)| {
            let a = (c1 - ray.origin).magnitude2();
            let b = (c2 - ray.origin).magnitude2();
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        if let Some((brush_id, face_id, _)) = hits.get(0).copied() {
            Some(RaycastResult { brush_id, face_id })
        } else {
            None
        }
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
