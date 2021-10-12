use std::{cell::Cell, collections::HashMap};
use tools::app::{
    event::RawInputKind,
    input::{ButtonKind, InputState, KeyKind},
};

#[derive(Default)]
pub struct InputMapper {
    key_state: HashMap<KeyKind, InputState>,
    button_state: HashMap<ButtonKind, InputState>,
    mouse_pos: (f32, f32),
    actions: HashMap<String, Action>,
}

impl InputMapper {
    pub fn process_raw_input(&mut self, input: RawInputKind) {
        match input {
            RawInputKind::Key(state, key) => {
                self.key_state.insert(key, state);
            }
            RawInputKind::Button(state, button) => {
                self.button_state.insert(button, state);
            }
            RawInputKind::Movement(x, y) => {
                self.mouse_pos = (x, y);
            }
        };
    }

    pub fn register_action(&mut self, name: &str, elements: Vec<ElementKind>) {
        let name = name.to_owned();
        self.actions.insert(
            name,
            Action {
                elements,
                queried: Cell::new(false),
            },
        );
    }

    fn query_action(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            if self.query_action_inner(action) {
                action.queried.set(true);
                return true;
            }
        }
        false
    }

    fn query_action_once(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            if self.query_action_inner(action) && !action.queried.get() {
                action.queried.set(true);
                return true;
            }
        }
        false
    }

    pub fn query_mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }

    fn query_action_inner(&self, action: &Action) -> bool {
        for element in &action.elements {
            match element {
                ElementKind::Key(key) => {
                    if matches!(self.key_state.get(key), None | Some(InputState::Released)) {
                        action.queried.set(false);
                        return false;
                    }
                }
                ElementKind::Button(button) => {
                    if matches!(
                        self.button_state.get(button),
                        None | Some(InputState::Released)
                    ) {
                        action.queried.set(false);
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub enum ElementKind {
    Key(KeyKind),
    Button(ButtonKind),
}

struct Action {
    elements: Vec<ElementKind>,
    queried: Cell<bool>,
}
