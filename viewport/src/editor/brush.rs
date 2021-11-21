use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use cgmath::{vec2, vec3, ElementWise, InnerSpace, Vector3};

use crate::{
    input::InputMapper,
    math::{IntersectionPoint, Ray, Triangle},
    render::{SolidBatch, SolidFactory, SolidVertex, Sprite, TextureID},
};

use super::{
    camera::WorldCamera,
    config::{FACE_HIGHLIGHT_COLOR, POINT_SELECT_RADIUS, VERTEX_HIGHLIGHT_COLOR},
    ActionBinding::*,
    EditMode,
};

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
    point: Vector3<f32>,
}

impl BrushBank {
    pub fn process(
        &mut self,
        mode: &EditMode,
        input: &InputMapper,
        solid_batches: &mut Vec<(TextureID, Rc<SolidBatch>)>,
        sprites: &mut HashMap<TextureID, Vec<Sprite>>,
        solid_factory: &SolidFactory,
        camera: &WorldCamera,
    ) {
        match mode {
            EditMode::Brush => self.brush_mode(input, camera, solid_factory),
            EditMode::Face => self.face_mode(input, camera, solid_factory),
            EditMode::Vertex => self.vertex_mode(input, camera, sprites),
        }

        *solid_batches = self.batches.clone();
    }

    fn brush_mode(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        solid_factory: &SolidFactory,
    ) {
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

    fn face_mode(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        solid_factory: &SolidFactory,
    ) {
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

    fn vertex_mode(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        sprites: &mut HashMap<TextureID, Vec<Sprite>>,
    ) {
        if input.is_active_once(Select) {
            if !input.is_active(EnableMultiSelect) {
                for brush in &mut self.brushes {
                    for point in &mut brush.points {
                        point.selected = false;
                    }
                }
            }

            let mut selection_candidates = Vec::new();

            for (i, brush) in self.brushes.iter_mut().enumerate() {
                for (j, point) in brush.points.iter_mut().enumerate() {
                    if let Some(screen_pos) = camera.project(point.position, 0.0) {
                        let screen_pos = vec2(screen_pos.x, screen_pos.y);
                        let dist = (screen_pos - input.mouse_pos()).magnitude2();
                        if dist < POINT_SELECT_RADIUS * POINT_SELECT_RADIUS {
                            selection_candidates.push((i, j, dist));
                        }
                    }
                }
            }

            selection_candidates.sort_unstable_by(|(_, _, a), (_, _, b)| {
                a.partial_cmp(&b).unwrap_or(Ordering::Equal)
            });

            if let Some((i, j, _)) = selection_candidates.get(0).copied() {
                let point = &self.brushes[i].points[j];

                let ray = Ray {
                    origin: camera.position(),
                    end: point.position,
                };

                let can_select = if let Some(RaycastResult {
                    point: hit_point, ..
                }) = self.raycast(ray)
                {
                    let a = (point.position - ray.origin).magnitude2();
                    let b = (hit_point - ray.origin).magnitude2();
                    if (a - b).abs() > 0.1 {
                        a < b
                    } else {
                        true
                    }
                } else {
                    true
                };

                if can_select {
                    let point = &mut self.brushes[i].points[j];
                    point.selected = !point.selected;
                }
            }
        }

        let mut vertex_sprites = Vec::new();
        for brush in &self.brushes {
            for point in &brush.points {
                let color = if point.selected {
                    VERTEX_HIGHLIGHT_COLOR
                } else {
                    [0.0, 0.0, 0.0, 1.0]
                };

                if let Some(origin) = camera.project(point.position, -0.001) {
                    vertex_sprites.push(Sprite {
                        origin: origin - vec3(5.0, 5.0, 0.0),
                        extent: vec2(10.0, 10.0),
                        color,
                    });
                }
            }
        }
        sprites.insert(1, vertex_sprites);
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
                    FACE_HIGHLIGHT_COLOR
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

                if let Some(point) = ray.intersection_point(&tri0) {
                    hits.push((i, j, point));
                } else if let Some(point) = ray.intersection_point(&tri1) {
                    hits.push((i, j, point));
                }
            }
        }

        hits.sort_unstable_by(|(_, _, c1), (_, _, c2)| {
            let a = (c1 - ray.origin).magnitude2();
            let b = (c2 - ray.origin).magnitude2();
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        if let Some((brush_id, face_id, point)) = hits.get(0).copied() {
            Some(RaycastResult {
                brush_id,
                face_id,
                point,
            })
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
