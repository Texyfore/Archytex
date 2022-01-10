use std::rc::Rc;

use cgmath::{vec2, vec3, InnerSpace, Matrix3, SquareMatrix, Vector2, Vector3};

use crate::{
    editor::{
        camera::WorldCamera,
        config::{NEW_BRUSH_MIN_SCREEN_DISTANCE, PRIMARY_COLOR},
        util,
        ActionBinding::*,
    },
    input::InputMapper,
    math::{IntersectionPoint, MinMax, Plane, SolidUtil},
    net,
    render::{LineBatch, LineFactory, Scene, SolidFactory, Sprite, TextureBank, TextureID},
};

use super::container::{AsSelectAllKind, SelectAllKind, SolidContainer};

pub struct SolidEditor {
    container: SolidContainer,
    mode: EditorMode,
    move_op: Option<Move>,
    current_texture_id: TextureID,
}

impl Default for SolidEditor {
    fn default() -> Self {
        Self {
            container: Default::default(),
            mode: Default::default(),
            move_op: Default::default(),
            current_texture_id: 1,
        }
    }
}

impl SolidEditor {
    pub fn container(&self) -> &SolidContainer {
        &self.container
    }

    pub fn process(&mut self, behave: bool, state: SolidEditorContext) {
        if !behave {
            self.container
                .rebuild(state.solid_factory, state.texture_bank);
            return;
        }

        let mut changed_mode = false;

        if state.input.is_active_once(SolidMode) {
            self.mode = EditorMode::Solid(Default::default());
            changed_mode = true;
        } else if state.input.is_active_once(FaceMode) {
            self.mode = EditorMode::Face(Default::default());
            changed_mode = true;
        } else if state.input.is_active_once(VertexMode) {
            self.mode = EditorMode::Point(Default::default());
            changed_mode = true;
        }

        if changed_mode {
            if self.move_op.is_some() {
                self.move_op = None;
                self.container.abort_move();
            }

            self.container.deselect();

            net::send_packet(format!(
                r#"{{ "message": "set-solid-editor-mode", "mode": {} }}"#,
                self.mode.as_i32()
            ));
        }

        let mut solids_copied = false;

        self.mode.process(
            &state,
            &mut self.container,
            &mut solids_copied,
            self.current_texture_id,
        );

        if !state.input.is_active(MoveCamera) && state.input.is_active_once(SelectAll) {
            self.container.select_all(&self.mode)
        }

        self.move_logic(&state, solids_copied);
    }

    pub fn render(&self, scene: &mut Scene, camera: &WorldCamera) {
        scene.world_pass.solid_batches = self.container.mesh();
        self.mode.render(scene, camera, &self.container);
    }

    pub fn set_mode(&mut self, mode: i32) {
        match mode {
            0 => self.mode = EditorMode::Solid(Default::default()),
            1 => self.mode = EditorMode::Face(Default::default()),
            2 => self.mode = EditorMode::Point(Default::default()),
            _ => {}
        }
        self.container.deselect();
    }

    pub fn deselect_all(&mut self) {
        self.container.deselect();
    }

    pub fn save(&self) -> mdl::Model {
        self.container.export()
    }

    pub fn load(&mut self, model: mdl::Model) {
        self.container = SolidContainer::load(model);
    }

    pub fn set_texture(&mut self, texture_id: TextureID) {
        self.current_texture_id = texture_id;
    }

    fn move_logic(&mut self, ctx: &SolidEditorContext, solids_copied: bool) {
        let mut begin_move = false;

        if ctx.input.is_active_once(Move) {
            if self.move_op.is_none() {
                begin_move = true;
            } else {
                self.move_op = None;
                self.container.abort_move();
            }
        }
        if solids_copied {
            begin_move = true;
        }

        if begin_move {
            let ray = ctx.world_camera.screen_ray(ctx.input.mouse_pos());
            let plane = self.container.move_plane(ray);

            if let Some(plane) = plane {
                let start = ray.intersection_point(&plane);
                if let Some(start) = start {
                    let start = (start + plane.normal * 0.01).snap(ctx.grid_length);
                    self.move_op = Some(Move {
                        plane,
                        start,
                        end: start,
                    })
                }
            }
        }

        if let Some(move_op) = self.move_op.as_mut() {
            let ray = ctx.world_camera.screen_ray(ctx.input.mouse_pos());
            if let Some(end) = ray.intersection_point(&move_op.plane) {
                let end = (end + move_op.plane.normal * 0.01).snap(ctx.grid_length);
                if (end - move_op.end).magnitude2() > 0.01 {
                    let vec = end - move_op.start;
                    self.container.move_selected(vec);
                    move_op.end = end;
                }
            }

            if ctx.input.is_active_once(ConfirmMove) {
                self.container.confirm_move();
                self.move_op = None;
            }

            if ctx.input.is_active_once(AbortMove) | ctx.input.is_active_once(AbortMoveAlt) {
                self.move_op = None;
                self.container.abort_move();
            }
        }
    }
}

pub struct SolidEditorContext<'a> {
    pub input: &'a InputMapper,
    pub world_camera: &'a WorldCamera,
    pub solid_factory: &'a SolidFactory,
    pub line_factory: &'a LineFactory,
    pub texture_bank: &'a TextureBank,
    pub grid_length: f32,
}

enum EditorMode {
    Solid(SolidState),
    Face(FaceState),
    Point(PointState),
}

impl Default for EditorMode {
    fn default() -> Self {
        Self::Solid(Default::default())
    }
}

impl EditorMode {
    fn as_i32(&self) -> i32 {
        match self {
            EditorMode::Solid(_) => 0,
            EditorMode::Face(_) => 1,
            EditorMode::Point(_) => 2,
        }
    }
}

impl EditorMode {
    fn process(
        &mut self,
        ctx: &SolidEditorContext,
        container: &mut SolidContainer,
        solids_copied: &mut bool,
        current_texture_id: TextureID,
    ) {
        match self {
            EditorMode::Solid(state) => state.process(ctx, container, solids_copied),
            EditorMode::Face(state) => state.process(ctx, container, current_texture_id),
            EditorMode::Point(state) => state.process(ctx, container),
        };
        container.rebuild(ctx.solid_factory, ctx.texture_bank);
    }

    fn render(&self, scene: &mut Scene, camera: &WorldCamera, container: &SolidContainer) {
        match self {
            EditorMode::Solid(state) => state.render(scene),
            EditorMode::Face(state) => state.render(scene),
            EditorMode::Point(state) => state.render(scene, camera, container),
        }
    }
}

#[derive(Default)]
struct SolidState {
    new_solid: Option<NewSolid>,
}

impl SolidState {
    fn process(
        &mut self,
        state: &SolidEditorContext,
        container: &mut SolidContainer,
        solids_copied: &mut bool,
    ) {
        if state.input.is_active_once(EditorAdd) {
            if let Some(raycast) =
                container.raycast(state.world_camera.screen_ray(state.input.mouse_pos()), true)
            {
                let world = (raycast.point + raycast.normal * 0.01).grid(state.grid_length);
                let screen = state.input.mouse_pos();

                self.new_solid = Some(NewSolid {
                    start: NewSolidPoint { world, screen },
                    end: NewSolidPoint { world, screen },
                    mesh: state.line_factory.create(&[]),
                });
            }
        }

        if let (true, Some(new_solid), Some(raycast)) = (
            state.input.is_active(EditorAdd),
            self.new_solid.as_mut(),
            container.raycast(state.world_camera.screen_ray(state.input.mouse_pos()), true),
        ) {
            new_solid.end = NewSolidPoint {
                world: (raycast.point + raycast.normal * 0.01).grid(state.grid_length),
                screen: state.input.mouse_pos(),
            };
            if new_solid.enough_mouse_distance() {
                new_solid.build_mesh(state.grid_length, state.line_factory);
            }
        }

        let mut can_select = true;

        if let (true, Some(new_solid)) = (
            state.input.was_active_once(EditorAdd),
            self.new_solid.as_ref(),
        ) {
            if new_solid.enough_mouse_distance() {
                let (origin, extent) = new_solid.origin_extent(state.grid_length);
                container.add(origin, extent);
                can_select = false;
            }
            self.new_solid = None;
        }

        if state.input.was_active_once(Select) && can_select {
            if !state.input.is_active(EnableMultiSelect) {
                container.deselect();
            }
            container.select_solid(state.world_camera, state.input.mouse_pos());
        }

        if state.input.is_active_once(EditorDel) {
            container.delete_selected();
        }

        if state.input.is_active_once(EditorCopy) {
            container.copy_solids();
            *solids_copied = true;
        }
    }

    fn render(&self, scene: &mut Scene) {
        if let Some(new_solid) = self.new_solid.as_ref() {
            if new_solid.enough_mouse_distance() {
                scene.world_pass.line_batches.push(new_solid.mesh());
            }
        }
    }
}

#[derive(Default)]
struct FaceState;

impl FaceState {
    fn process(
        &mut self,
        state: &SolidEditorContext,
        container: &mut SolidContainer,
        current_texture_id: TextureID,
    ) {
        if state.input.was_active_once(Select) {
            if !state.input.is_active(EnableMultiSelect) {
                container.deselect();
            }
            container.select_face(state.world_camera, state.input.mouse_pos());
        }

        if state.input.is_active_once(SetTexture) {
            container.set_texture(current_texture_id);
        }
    }

    fn render(&self, _scene: &mut Scene) {}
}

#[derive(Default)]
struct PointState;

impl PointState {
    fn process(&mut self, state: &SolidEditorContext, container: &mut SolidContainer) {
        if state.input.was_active_once(Select) {
            if !state.input.is_active(EnableMultiSelect) {
                container.deselect();
            }
            container.select_point(state.world_camera, state.input.mouse_pos());
        }
    }

    fn render(&self, scene: &mut Scene, camera: &WorldCamera, container: &SolidContainer) {
        scene.sprite_pass.sprites.insert(
            0,
            container
                .point_graphics()
                .iter()
                .map(|pg| {
                    camera.project(pg.position, -0.001).map(|p| Sprite {
                        origin: p - vec3(5.0, 5.0, 0.0),
                        extent: vec2(10.0, 10.0),
                        color: if pg.selected {
                            [PRIMARY_COLOR[0], PRIMARY_COLOR[1], PRIMARY_COLOR[2], 1.0]
                        } else {
                            [0.0, 0.0, 0.0, 1.0]
                        },
                    })
                })
                .flatten()
                .collect(),
        );
    }
}

struct NewSolid {
    start: NewSolidPoint,
    end: NewSolidPoint,
    mesh: Rc<LineBatch>,
}

impl NewSolid {
    fn min_max(&self, grid_length: f32) -> (Vector3<f32>, Vector3<f32>) {
        let min = self.start.world.min(self.end.world).cast::<f32>().unwrap() * grid_length;
        let max = self.start.world.max(self.end.world).cast::<f32>().unwrap() * grid_length;
        (min, max)
    }

    fn origin_extent(&self, grid_length: f32) -> (Vector3<f32>, Vector3<f32>) {
        let (min, max) = self.min_max(grid_length);
        let origin = min;
        let extent = max - min + vec3(1.0, 1.0, 1.0) * grid_length;
        (origin, extent)
    }

    fn center_half_extent(&self, grid_length: f32) -> (Vector3<f32>, Vector3<f32>) {
        let (min, max) = self.min_max(grid_length);
        let half_extent = (max - min + vec3(1.0, 1.0, 1.0) * grid_length) * 0.5;
        let center = min + half_extent;
        (center, half_extent)
    }

    fn enough_mouse_distance(&self) -> bool {
        (self.end.screen - self.start.screen).magnitude2()
            > NEW_BRUSH_MIN_SCREEN_DISTANCE * NEW_BRUSH_MIN_SCREEN_DISTANCE
    }

    fn mesh(&self) -> Rc<LineBatch> {
        self.mesh.clone()
    }

    fn build_mesh(&mut self, grid_length: f32, factory: &LineFactory) {
        let (center, half_extent) = self.center_half_extent(grid_length);
        self.mesh = factory.create(&util::line_cuboid(
            center,
            half_extent,
            Matrix3::identity(),
            0.01,
            [0.0; 3],
        ));
    }
}

struct NewSolidPoint {
    world: Vector3<i32>,
    screen: Vector2<f32>,
}

struct Move {
    plane: Plane,
    start: Vector3<f32>,
    end: Vector3<f32>,
}

impl AsSelectAllKind for EditorMode {
    fn as_select_all_kind(&self) -> SelectAllKind {
        match self {
            EditorMode::Solid(_) => SelectAllKind::Solids,
            EditorMode::Face(_) => SelectAllKind::Faces,
            EditorMode::Point(_) => SelectAllKind::Points,
        }
    }
}
