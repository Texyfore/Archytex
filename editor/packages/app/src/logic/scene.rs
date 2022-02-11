use std::collections::HashMap;

use asset::TextureID;
use cgmath::Vector3;

use crate::graphics::{Canvas, Graphics};

use super::element::{ElementKind, Prop, Solid};

#[derive(Default)]
pub struct Scene {
    solids: HashMap<usize, Solid>,
    props: HashMap<usize, Prop>,
    next_elem_id: usize,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl Scene {
    pub fn act(&mut self, ctx: Context, action: Action) {
        if let Some(reaction) = self.execute(ctx, action) {
            self.undo_stack.push(reaction);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, ctx: Context) {
        if let Some(action) = self.undo_stack.pop() {
            if let Some(reaction) = self.execute(ctx, action) {
                self.redo_stack.push(reaction);
            }
        }
    }

    pub fn redo(&mut self, ctx: Context) {
        if let Some(action) = self.redo_stack.pop() {
            if let Some(reaction) = self.execute(ctx, action) {
                self.undo_stack.push(reaction);
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        for solid in self.solids.values() {
            solid.render(canvas);
        }

        for prop in self.props.values() {
            prop.render(canvas);
        }
    }

    fn execute(&mut self, _ctx: Context, _action: Action) -> Option<Action> {
        None
    }
}

pub struct Context<'g> {
    graphics: &'g Graphics,
}

pub enum Action {
    NewSolids(Vec<Solid>),
    NewProps(Vec<Prop>),

    AddSolids(Vec<(usize, Solid)>),
    AddProps(Vec<(usize, Prop)>),

    RemoveSolids(Vec<usize>),
    RemoveProps(Vec<usize>),

    Select {
        kind: ElementKind,
        ids: Vec<usize>,
    },

    DeselectAll(ElementKind),

    Move {
        kind: ElementKind,
        delta: Vector3<i32>,
    },

    RotateProps(Vector3<i32>),
    AssignTexture(TextureID),
}
