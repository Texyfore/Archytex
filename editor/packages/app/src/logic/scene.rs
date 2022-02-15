use std::collections::HashMap;

use asset::TextureID;
use cgmath::{Vector2, Vector3};

use crate::graphics::{Canvas, Graphics};

use super::{
    camera::Camera,
    elements::{
        self, ElementKind, FaceLocator, PointLocator, Prop, RaycastHit, RaycastInput, Solid,
    },
};

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

    pub fn raycast(&self, screen_pos: Vector2<f32>, camera: &Camera) -> RaycastHit {
        elements::raycast(RaycastInput {
            solids: &self.solids,
            props: &self.props,
            camera,
            screen_pos,
        })
    }

    pub fn render(&self, canvas: &mut Canvas, mask: ElementKind) {
        for solid in self.solids.values() {
            solid.render(canvas, matches!(mask, ElementKind::Point));
        }

        if matches!(mask, ElementKind::Prop) {
            for prop in self.props.values() {
                prop.render(canvas);
            }
        }
    }

    fn execute(&mut self, ctx: Context, action: Action) -> Option<Action> {
        match action {
            Action::NewSolids(solids) => {
                let ids = solids
                    .into_iter()
                    .map(|solid| {
                        let id = self.next_elem_id;
                        self.solids.insert(id, solid);
                        id
                    })
                    .collect::<Vec<_>>();

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            // NewProps
            Action::AddSolids(solids) => {
                let mut ids = Vec::new();

                for (id, solid) in solids {
                    self.solids.insert(id, solid);
                    ids.push(id);
                }

                (!ids.is_empty()).then(|| Action::RemoveSolids(ids))
            }

            // AddProps
            Action::RemoveSolids(ids) => {
                let mut solids = Vec::new();
                for id in ids {
                    solids.push((id, self.solids.remove(&id).unwrap()));
                }

                (!solids.is_empty()).then(|| Action::AddSolids(solids))
            }

            // RemoveProps
            Action::SelectSolids(ids) => {
                for id in &ids {
                    let solid = self.solids.get_mut(id).unwrap();
                    solid.set_selected(!solid.selected());
                    solid.recalc(ctx.graphics);
                }
                (!ids.is_empty()).then(|| Action::SelectSolids(ids))
            }

            Action::SelectFaces(locators) => {
                for locator in &locators {
                    let solid = self.solids.get_mut(&locator.solid).unwrap();
                    solid.set_face_selected(locator.face, !solid.face_selected(locator.face));
                    solid.recalc(ctx.graphics);
                }
                (!locators.is_empty()).then(|| Action::SelectFaces(locators))
            }

            // SelectPoints
            // SelectProps
            Action::DeselectAll(kind) => match kind {
                ElementKind::Solid => {
                    let mut ids = Vec::new();
                    for (id, solid) in &mut self.solids {
                        if solid.selected() {
                            solid.set_selected(false);
                            solid.recalc(ctx.graphics);
                            ids.push(*id);
                        }
                    }
                    (!ids.is_empty()).then(|| Action::SelectSolids(ids))
                }
                ElementKind::Face => {
                    let mut locators = Vec::new();

                    for (sid, solid) in &mut self.solids {
                        let mut recalc = false;

                        for fid in 0..6 {
                            if solid.face_selected(fid) {
                                solid.set_face_selected(fid, false);
                                locators.push(FaceLocator {
                                    solid: *sid,
                                    face: fid,
                                });
                                recalc = true;
                            }
                        }

                        if recalc {
                            solid.recalc(ctx.graphics);
                        }
                    }

                    (!locators.is_empty()).then(|| Action::SelectFaces(locators))
                }
                ElementKind::Point => todo!(),
                ElementKind::Prop => todo!(),
            },

            // Move
            // RotateProps
            // AssignTexture
            _ => todo!(),
        }
    }
}

pub struct Context<'a> {
    pub graphics: &'a Graphics,
}

pub enum Action {
    NewSolids(Vec<Solid>),
    NewProps(Vec<Prop>),

    AddSolids(Vec<(usize, Solid)>),
    AddProps(Vec<(usize, Prop)>),

    RemoveSolids(Vec<usize>),
    RemoveProps(Vec<usize>),

    SelectSolids(Vec<usize>),
    SelectFaces(Vec<FaceLocator>),
    SelectPoints(Vec<PointLocator>),
    SelectProps(Vec<usize>),
    DeselectAll(ElementKind),

    Move {
        kind: ElementKind,
        delta: Vector3<i32>,
    },

    RotateProps(Vector3<i32>),
    AssignTexture(TextureID),
}
