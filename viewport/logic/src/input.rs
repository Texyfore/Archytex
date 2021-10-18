use std::{cell::Cell, collections::HashMap};
use tools::{
    app::{
        event::RawInputKind,
        input::{ButtonKind, InputState, KeyKind},
    },
    math::{Vector2, Zero},
};

pub struct InputMapper {
    key_state: HashMap<KeyKind, InputState>,
    button_state: HashMap<ButtonKind, InputState>,
    mouse_pos_before: Vector2<f32>,
    mouse_pos: Vector2<f32>,
    wheel_delta: f32,
    actions: HashMap<String, Action>,
}

impl Default for InputMapper {
    fn default() -> Self {
        Self {
            key_state: Default::default(),
            button_state: Default::default(),
            mouse_pos_before: Vector2::zero(),
            mouse_pos: Vector2::zero(),
            wheel_delta: 0.0,
            actions: Default::default(),
        }
    }
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
                self.mouse_pos = Vector2::new(x, y);
            }
            RawInputKind::Wheel(delta) => {
                self.wheel_delta = delta;
            }
        };
    }

    pub fn clear(&mut self) {
        self.wheel_delta = 0.0;
        self.mouse_pos_before = self.mouse_pos
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

    pub fn query_action(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            if self.query_action_inner(action) {
                action.queried.set(true);
                return true;
            }
        }
        false
    }

    pub fn query_action_once(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            if self.query_action_inner(action) && !action.queried.get() {
                action.queried.set(true);
                return true;
            }
        }
        false
    }

    pub fn query_mouse_pos(&self) -> Vector2<f32> {
        self.mouse_pos
    }

    pub fn query_mouse_delta(&self) -> Vector2<f32> {
        self.mouse_pos - self.mouse_pos_before
    }

    pub fn query_wheel_delta(&self) -> f32 {
        self.wheel_delta
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
