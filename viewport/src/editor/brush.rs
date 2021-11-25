#![allow(clippy::too_many_arguments)]

use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use cgmath::{vec2, vec3, ElementWise, InnerSpace, Vector2, Vector3, Zero};

use crate::{
    input::InputMapper,
    math::{IntersectionPoint, MinMax, Plane, Ray, SolidUtil, Triangle},
    render::{
        LineBatch, LineFactory, LineVertex, SolidBatch, SolidFactory, SolidVertex, Sprite,
        TextureID,
    },
};

use super::{
    camera::WorldCamera,
    config::{
        FACE_HIGHLIGHT_COLOR, NEW_BRUSH_MIN_SCREEN_DISTANCE, POINT_SELECT_RADIUS,
        VERTEX_HIGHLIGHT_COLOR,
    },
    ActionBinding::*,
    EditMode,
};

pub struct BrushBank {
    brushes: Vec<Brush>,
    batches: Vec<(TextureID, Rc<SolidBatch>)>,
    new_brush: NewBrush,
    move_operation: Option<MoveOperation>,
    needs_rebuild: bool,
    previous_mode: EditMode,
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

#[derive(Default)]
struct NewBrush {
    start: Option<NewBrushPoint>,
    end: Option<NewBrushPoint>,
    bocks: Option<NewBrushBox>,
}

struct NewBrushPoint {
    world: Vector3<i32>,
    screen: Vector2<f32>,
}

struct NewBrushBox {
    origin: Vector3<f32>,
    extent: Vector3<f32>,
    batch: Rc<LineBatch>,
}

struct RaycastResult {
    brush: Option<RaycastBrush>,
    point: Vector3<f32>,
    normal: Vector3<f32>,
}

struct RaycastBrush {
    brush_id: usize,
    face_id: usize,
}

struct MoveOperation {
    plane: Plane,
    start: Vector3<f32>,
    end: Option<Vector3<f32>>,
    points: Vec<MovePoint>,
}

struct MovePoint {
    brush: usize,
    point: usize,
    original_pos: Vector3<f32>,
}

enum MoveKind {
    Brush,
    Face,
    Vertex,
}

impl Default for BrushBank {
    fn default() -> Self {
        // Phantom brush workaround for #161
        let brushes = vec![Brush::cuboid(
            vec3(1000000.0, 1000000.0, 1000000.0),
            vec3(0.001, 0.001, 0.001),
        )];

        Self {
            brushes,
            batches: Default::default(),
            new_brush: Default::default(),
            move_operation: None,
            needs_rebuild: false,
            previous_mode: EditMode::Brush,
        }
    }
}

impl BrushBank {
    pub fn process(
        &mut self,
        mode: &EditMode,
        input: &InputMapper,
        camera: &WorldCamera,
        solid_factory: &SolidFactory,
        line_factory: &LineFactory,
        solid_batches: &mut Vec<(TextureID, Rc<SolidBatch>)>,
        line_batches: &mut Vec<Rc<LineBatch>>,
        sprites: &mut HashMap<TextureID, Vec<Sprite>>,
        grid_length: f32,
    ) {
        match mode {
            EditMode::Brush => {
                self.brush_mode(input, camera, line_factory, line_batches, grid_length)
            }
            EditMode::Face => self.face_mode(input, camera, grid_length),
            EditMode::Vertex => self.vertex_mode(input, camera, sprites, grid_length),
        }

        if mode != &self.previous_mode {
            self.deselect_all();
            self.needs_rebuild = true;
        }

        if self.needs_rebuild {
            let mut batches = HashMap::new();
            for brush in &self.brushes {
                for face in &brush.faces {
                    batches
                        .entry(face.texture)
                        .or_insert_with(|| (Vec::new(), Vec::new()));

                    let (vertices, triangles) = batches.get_mut(&face.texture).unwrap();

                    let t0 = vertices.len() as u16;
                    triangles.push([t0, t0 + 1, t0 + 2]);
                    triangles.push([t0, t0 + 2, t0 + 3]);

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
                        let texcoord = point.texcoord(normal);
                        vertices.push(SolidVertex {
                            position: point.into(),
                            normal: normal.into(),
                            texcoord: texcoord.into(),
                            color,
                        });
                    }
                }
            }

            self.batches = batches
                .iter()
                .map(|(t, (v, i))| (*t, solid_factory.create(v, i)))
                .collect();

            self.needs_rebuild = false;
        }

        self.previous_mode = *mode;
        *solid_batches = self.batches.clone();
    }

    fn brush_mode(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        line_factory: &LineFactory,
        line_batches: &mut Vec<Rc<LineBatch>>,
        grid_length: f32,
    ) {
        self.move_logic(input, camera, MoveKind::Brush, grid_length);

        if input.is_active_once(AddBrush) {
            self.new_brush.start = {
                let hit = self.raycast_or_xz(camera.screen_ray(input.mouse_pos()));
                Some(NewBrushPoint {
                    world: hit.point.grid(grid_length),
                    screen: input.mouse_pos(),
                })
            };
        }

        if input.is_active(AddBrush) {
            self.new_brush.end = {
                let hit = self.raycast_or_xz(camera.screen_ray(input.mouse_pos()));
                Some(NewBrushPoint {
                    world: hit.point.grid(grid_length),
                    screen: input.mouse_pos(),
                })
            };

            if let (Some(start), Some(end)) =
                (self.new_brush.start.as_ref(), self.new_brush.end.as_ref())
            {
                const MIN_SQR: f32 = NEW_BRUSH_MIN_SCREEN_DISTANCE * NEW_BRUSH_MIN_SCREEN_DISTANCE;
                let dist_sqr = (end.screen - start.screen).magnitude2();

                if dist_sqr > MIN_SQR {
                    let min = start.world.min(end.world).cast::<f32>().unwrap() * grid_length;
                    let max = start.world.max(end.world).cast::<f32>().unwrap() * grid_length;

                    let origin = min;
                    let extent = max - min + vec3(1.0, 1.0, 1.0) * grid_length;

                    self.new_brush.bocks = Some(NewBrushBox {
                        origin,
                        extent,
                        batch: line_factory.create(&build_grid_box(origin, extent)),
                    });
                }
            }
        }

        let mut can_select = true;
        if input.was_active_once(AddBrush) {
            if let Some(bocks) = self.new_brush.bocks.as_ref() {
                self.brushes.push(Brush::cuboid(bocks.origin, bocks.extent));

                for brush in &mut self.brushes {
                    brush.selected = false;
                }

                self.new_brush.start = None;
                self.new_brush.end = None;
                self.new_brush.bocks = None;
                self.needs_rebuild = true;
                can_select = false;
            }
        }

        if input.is_active_once(DeleteBrush) {
            self.brushes.retain(|b| !b.selected);
            self.needs_rebuild = true;
        }

        if input.was_active_once(Select) && can_select {
            if !input.is_active(EnableMultiSelect) {
                for brush in &mut self.brushes {
                    brush.selected = false;
                }
                self.needs_rebuild = true;
            }

            if let Some(RaycastResult {
                brush: Some(brush), ..
            }) = self.raycast(camera.screen_ray(input.mouse_pos()))
            {
                let selected = &mut self.brushes[brush.brush_id].selected;
                *selected = !*selected;
                self.needs_rebuild = true;
            }
        }

        if let Some(bocks) = self.new_brush.bocks.as_ref() {
            line_batches.push(bocks.batch.clone());
        }
    }

    fn face_mode(&mut self, input: &InputMapper, camera: &WorldCamera, grid_length: f32) {
        self.move_logic(input, camera, MoveKind::Face, grid_length);

        if input.was_active_once(Select) {
            if !input.is_active(EnableMultiSelect) {
                for brush in &mut self.brushes {
                    for face in &mut brush.faces {
                        face.selected = false;
                    }
                }
                self.needs_rebuild = true;
            }

            if let Some(RaycastResult {
                brush: Some(brush), ..
            }) = self.raycast(camera.screen_ray(input.mouse_pos()))
            {
                let selected = &mut self.brushes[brush.brush_id].faces[brush.face_id].selected;
                *selected = !*selected;
                self.needs_rebuild = true;
            }
        }
    }

    fn vertex_mode(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        sprites: &mut HashMap<TextureID, Vec<Sprite>>,
        grid_length: f32,
    ) {
        self.move_logic(input, camera, MoveKind::Vertex, grid_length);

        if input.was_active_once(Select) {
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
                a.partial_cmp(b).unwrap_or(Ordering::Equal)
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
                    hits.push((i, j, point, tri0.normal()));
                } else if let Some(point) = ray.intersection_point(&tri1) {
                    hits.push((i, j, point, tri1.normal()));
                }
            }
        }

        hits.sort_unstable_by(|(_, _, c1, _), (_, _, c2, _)| {
            let a = (c1 - ray.origin).magnitude2();
            let b = (c2 - ray.origin).magnitude2();
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        if let Some((brush_id, face_id, point, normal)) = hits.get(0).copied() {
            Some(RaycastResult {
                brush: Some(RaycastBrush { brush_id, face_id }),
                point,
                normal,
            })
        } else {
            None
        }
    }

    fn raycast_or_xz(&self, ray: Ray) -> RaycastResult {
        let mut result = self.raycast(ray).unwrap_or_else(|| {
            let plane = Plane {
                origin: Vector3::zero(),
                normal: Vector3::unit_y(),
            };

            ray.intersection_point(&plane)
                .map(|p| RaycastResult {
                    brush: None,
                    point: p,
                    normal: plane.normal,
                })
                .unwrap()
        });

        result.point += result.normal * 0.01;
        result
    }

    fn begin_move_brushes(&mut self, input: &InputMapper, camera: &WorldCamera, grid_length: f32) {
        let mut points = Vec::new();
        let mut center = Vector3::zero();
        let mut div = 0.0;

        for (i, brush) in self.brushes.iter().enumerate().filter(|(_, b)| b.selected) {
            for (j, point) in brush.points.iter().enumerate() {
                center += point.position;
                div += 1.0;
                points.push(MovePoint {
                    brush: i,
                    point: j,
                    original_pos: point.position,
                });
            }
        }

        center /= div;
        self.move_operation = MoveOperation::new(input, camera, center, points, grid_length);
    }

    fn begin_move_faces(&mut self, input: &InputMapper, camera: &WorldCamera, grid_length: f32) {
        let mut points = Vec::new();
        let mut center = Vector3::zero();
        let mut div = 0.0;

        for (i, brush) in self.brushes.iter().enumerate() {
            for face in brush.faces.iter().filter(|f| f.selected) {
                for j in face.quad {
                    let j = j as usize;
                    let point = &brush.points[j];

                    center += point.position;
                    div += 1.0;
                    points.push(MovePoint {
                        brush: i,
                        point: j,
                        original_pos: point.position,
                    });
                }
            }
        }

        center /= div;
        self.move_operation = MoveOperation::new(input, camera, center, points, grid_length);
    }

    fn begin_move_vertices(&mut self, input: &InputMapper, camera: &WorldCamera, grid_length: f32) {
        let mut points = Vec::new();
        let mut center = Vector3::zero();
        let mut div = 0.0;

        for (i, brush) in self.brushes.iter().enumerate() {
            for (j, point) in brush.points.iter().enumerate().filter(|(_, p)| p.selected) {
                center += point.position;
                div += 1.0;
                points.push(MovePoint {
                    brush: i,
                    point: j,
                    original_pos: point.position,
                });
            }
        }

        center /= div;
        self.move_operation = MoveOperation::new(input, camera, center, points, grid_length);
    }

    fn move_logic(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        kind: MoveKind,
        grid_length: f32,
    ) {
        if input.is_active_once(Move) {
            if self.move_operation.is_some() {
                self.move_operation
                    .take()
                    .unwrap()
                    .abort(&mut self.brushes, &mut self.needs_rebuild)
            } else {
                match kind {
                    MoveKind::Brush => self.begin_move_brushes(input, camera, grid_length),
                    MoveKind::Face => self.begin_move_faces(input, camera, grid_length),
                    MoveKind::Vertex => self.begin_move_vertices(input, camera, grid_length),
                }
            }
        } else if input.is_active_once(ConfirmMove) {
            self.move_operation = None;
        } else if input.is_active_once(AbortMove) && self.move_operation.is_some() {
            self.move_operation
                .take()
                .unwrap()
                .abort(&mut self.brushes, &mut self.needs_rebuild)
        }

        if let Some(move_operation) = self.move_operation.as_mut() {
            move_operation.update(
                input,
                camera,
                &mut self.brushes,
                &mut self.needs_rebuild,
                grid_length,
            );
        }
    }

    fn deselect_all(&mut self) {
        for brush in &mut self.brushes {
            brush.selected = false;

            for face in &mut brush.faces {
                face.selected = false;
            }

            for point in &mut brush.points {
                point.selected = false;
            }
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

impl MoveOperation {
    fn new(
        input: &InputMapper,
        camera: &WorldCamera,
        center: Vector3<f32>,
        points: Vec<MovePoint>,
        grid_length: f32,
    ) -> Option<Self> {
        let ray = camera.screen_ray(input.mouse_pos());
        let plane = Plane {
            origin: center.snap(grid_length),
            normal: (-camera.forward()).cardinal(),
        };

        ray.intersection_point(&plane).map(|isp| MoveOperation {
            plane,
            start: isp.snap(grid_length),
            end: None,
            points,
        })
    }

    fn update(
        &mut self,
        input: &InputMapper,
        camera: &WorldCamera,
        brushes: &mut [Brush],
        rebuild: &mut bool,
        grid_length: f32,
    ) {
        let ray = camera.screen_ray(input.mouse_pos());
        if let Some(isp) = ray.intersection_point(&self.plane) {
            let point = isp.snap(grid_length);
            if let Some(end) = self.end {
                if (end - point).magnitude2() > 0.01 {
                    let vec = point - self.start;
                    for move_point in &self.points {
                        let point = &mut brushes[move_point.brush].points[move_point.point];
                        point.position = (move_point.original_pos + vec).snap(grid_length);
                        *rebuild = true;
                    }
                    self.end = Some(point);
                }
            } else {
                self.end = Some(point);
            }
        }
    }

    fn abort(self, brushes: &mut [Brush], rebuild: &mut bool) {
        for move_point in &self.points {
            let point = &mut brushes[move_point.brush].points[move_point.point];
            point.position = move_point.original_pos;
            *rebuild = true;
        }
    }
}

fn build_grid_box(origin: Vector3<f32>, extent: Vector3<f32>) -> Vec<LineVertex> {
    const LIM: f32 = 0.01;

    let corrections = [
        vec3(LIM, LIM, LIM),
        vec3(-LIM, LIM, LIM),
        vec3(-LIM, LIM, -LIM),
        vec3(LIM, LIM, -LIM),
        vec3(LIM, -LIM, LIM),
        vec3(-LIM, -LIM, LIM),
        vec3(-LIM, -LIM, -LIM),
        vec3(LIM, -LIM, -LIM),
    ];

    let mut points = [
        vec3(0.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
        vec3(1.0, 1.0, 0.0),
        vec3(1.0, 1.0, 1.0),
        vec3(0.0, 1.0, 1.0),
    ];

    for (i, p) in points.iter_mut().enumerate() {
        *p = origin + p.mul_element_wise(extent) + corrections[i];
    }

    let lines = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7],
    ];

    let mut vertices = Vec::new();

    for line in lines {
        for point in line {
            vertices.push(LineVertex {
                position: points[point].into(),
                color: [0.0, 0.0, 0.0, 1.0],
            });
        }
    }

    vertices
}
