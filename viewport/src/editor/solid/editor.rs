use crate::{
    editor::ActionBinding::*,
    input::InputMapper,
    render::{LineFactory, Scene, SolidFactory, TextureBank},
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
        self.mode.process(&ctx);
    }

    pub fn render(&self, scene: &mut Scene) {
        self.mode.render(scene);
    }
}

pub struct SolidEditorContext<'a> {
    pub input: &'a InputMapper,
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

    fn process(&mut self, ctx: &SolidEditorContext) {
        match self {
            EditState::Solid(state) => state.process(ctx),
            EditState::Face(state) => state.process(ctx),
            EditState::Point(state) => state.process(ctx),
        }
    }

    fn render(&self, scene: &mut Scene) {
        match self {
            EditState::Solid(state) => state.render(scene),
            EditState::Face(state) => state.render(scene),
            EditState::Point(state) => state.render(scene),
        }
    }
}

#[derive(Default)]
struct SolidState;

impl SolidState {
    fn process(&mut self, ctx: &SolidEditorContext) {}

    fn render(&self, scene: &mut Scene) {}
}

#[derive(Default)]
struct FaceState;

impl FaceState {
    fn process(&mut self, ctx: &SolidEditorContext) {}

    fn render(&self, scene: &mut Scene) {}
}

#[derive(Default)]
struct PointState;

impl PointState {
    fn process(&mut self, ctx: &SolidEditorContext) {}

    fn render(&self, scene: &mut Scene) {}
}
