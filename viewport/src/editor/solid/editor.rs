use std::rc::Rc;

use cgmath::{vec3, ElementWise, InnerSpace, Vector2, Vector3};

use crate::{
    editor::{camera::WorldCamera, config::NEW_BRUSH_MIN_SCREEN_DISTANCE, ActionBinding::*},
    input::InputMapper,
    math::{MinMax, SolidUtil},
    render::{LineBatch, LineFactory, LineVertex, Scene, SolidFactory, TextureBank},
};

use super::container::SolidContainer;

#[derive(Default)]
pub struct SolidEditor {
    container: SolidContainer,
    mode: EditState,
}

impl SolidEditor {
    pub fn process(&mut self, ctx: SolidEditorContext) {
        if ctx.input.is_active_once(SwitchMode) {
            self.mode.switch();
        }
        self.mode.process(&ctx, &mut self.container);
    }

    pub fn render(&self, scene: &mut Scene) {
        self.mode.render(scene, &self.container);
    }
}

pub struct SolidEditorContext<'a> {
    pub input: &'a InputMapper,
    pub world_camera: &'a WorldCamera,
    pub solid_factory: &'a SolidFactory,
    pub line_factory: &'a LineFactory,
    pub texture_bank: &'a TextureBank,
}

enum EditState {
    Solid(SolidState),
    Face(FaceState),
    Point(PointState),
}

impl Default for EditState {
    fn default() -> Self {
        Self::Solid(Default::default())
    }
}

impl EditState {
    fn switch(&mut self) {
        *self = match self {
            Self::Solid(_) => Self::Face(Default::default()),
            Self::Face(_) => Self::Point(Default::default()),
            Self::Point(_) => Self::Solid(Default::default()),
        };
    }

    fn process(&mut self, ctx: &SolidEditorContext, container: &mut SolidContainer) {
        match self {
            EditState::Solid(state) => state.process(ctx, container),
            EditState::Face(state) => state.process(ctx, container),
            EditState::Point(state) => state.process(ctx, container),
        };
        container.rebuild(ctx.solid_factory, ctx.texture_bank);
    }

    fn render(&self, scene: &mut Scene, container: &SolidContainer) {
        match self {
            EditState::Solid(state) => state.render(scene, container),
            EditState::Face(state) => state.render(scene, container),
            EditState::Point(state) => state.render(scene, container),
        }
    }
}

#[derive(Default)]
struct SolidState {
    new_solid: Option<NewSolid>,
}

impl SolidState {
    fn process(&mut self, ctx: &SolidEditorContext, container: &mut SolidContainer) {
        if ctx.input.is_active_once(AddBrush) {
            if let Some(raycast) =
                container.raycast(ctx.world_camera.screen_ray(ctx.input.mouse_pos()))
            {
                let world = raycast.point.grid(1.0);
                let screen = ctx.input.mouse_pos();

                self.new_solid = Some(NewSolid {
                    start: NewSolidPoint { world, screen },
                    end: NewSolidPoint { world, screen },
                    mesh: ctx.line_factory.create(&[]),
                });
            }
        }

        if let (true, Some(new_solid), Some(raycast)) = (
            ctx.input.is_active(AddBrush),
            self.new_solid.as_mut(),
            container.raycast(ctx.world_camera.screen_ray(ctx.input.mouse_pos())),
        ) {
            new_solid.end = NewSolidPoint {
                world: (raycast.point + raycast.normal * 0.01).grid(1.0),
                screen: ctx.input.mouse_pos(),
            };
            if new_solid.enough_mouse_distance() {
                new_solid.build_mesh(1.0, ctx.line_factory);
            }
        }

        let mut can_select = true;

        if let (true, Some(new_solid)) =
            (ctx.input.was_active_once(AddBrush), self.new_solid.as_ref())
        {
            if new_solid.enough_mouse_distance() {
                let (origin, extent) = new_solid.origin_extent(1.0);
                container.add(origin, extent);
                can_select = false;
            }
            self.new_solid = None;
        }

        if ctx.input.was_active_once(Select) && can_select {
            if !ctx.input.is_active(EnableMultiSelect) {
                container.deselect();
            }
            container.select_solid(ctx.world_camera, ctx.input.mouse_pos());
        }

        if ctx.input.is_active_once(DeleteBrush) {
            container.delete_selected();
        }
    }

    fn render(&self, scene: &mut Scene, container: &SolidContainer) {
        scene.world_pass.solid_batches = container.mesh();
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
    fn process(&mut self, ctx: &SolidEditorContext, container: &mut SolidContainer) {}

    fn render(&self, scene: &mut Scene, container: &SolidContainer) {}
}

#[derive(Default)]
struct PointState;

impl PointState {
    fn process(&mut self, ctx: &SolidEditorContext, container: &mut SolidContainer) {}

    fn render(&self, scene: &mut Scene, container: &SolidContainer) {}
}

struct NewSolid {
    start: NewSolidPoint,
    end: NewSolidPoint,
    mesh: Rc<LineBatch>,
}

impl NewSolid {
    fn origin_extent(&self, grid_length: f32) -> (Vector3<f32>, Vector3<f32>) {
        let min = self.start.world.min(self.end.world).cast::<f32>().unwrap() * grid_length;
        let max = self.start.world.max(self.end.world).cast::<f32>().unwrap() * grid_length;

        let origin = min;
        let extent = max - min + vec3(1.0, 1.0, 1.0) * grid_length;

        (origin, extent)
    }

    fn enough_mouse_distance(&self) -> bool {
        (self.end.screen - self.start.screen).magnitude2()
            > NEW_BRUSH_MIN_SCREEN_DISTANCE * NEW_BRUSH_MIN_SCREEN_DISTANCE
    }

    fn mesh(&self) -> Rc<LineBatch> {
        self.mesh.clone()
    }

    fn build_mesh(&mut self, grid_length: f32, factory: &LineFactory) {
        let (origin, extent) = self.origin_extent(grid_length);

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

        self.mesh = factory.create(&vertices);
    }
}

struct NewSolidPoint {
    world: Vector3<i32>,
    screen: Vector2<f32>,
}
