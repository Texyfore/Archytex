use std::collections::HashMap;
use tools::app::{
    event::RawInputKind,
    input::{ButtonKind, InputState, KeyKind},
};

#[derive(Default)]
pub struct InputMapper {
    key_state: HashMap<KeyKind, InputState>,
    button_state: HashMap<ButtonKind, InputState>,
    mouse_pos: (f32, f32),
    actions: HashMap<String, Vec<ActionKind>>,
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

    pub fn register_action(&mut self, name: &str, elements: &[ActionKind]) {
        let name = name.to_owned();
        let elements = elements.to_vec();
        self.actions.insert(name, elements);
    }

    pub fn query_action(&self, action: &str) -> bool {
        if let Some(actions) = self.actions.get(action) {
            for action in actions {
                match action {
                    ActionKind::Key(key) => {
                        if !self.query_key(key) {
                            return false;
                        }
                    }
                    ActionKind::Button(button) => {
                        if !self.query_button(button) {
                            return false;
                        }
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn query_mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }

    fn query_key(&self, key: &KeyKind) -> bool {
        matches!(self.key_state.get(key), Some(InputState::Pressed))
    }

    fn query_button(&self, button: &ButtonKind) -> bool {
        matches!(self.button_state.get(button), Some(InputState::Pressed))
    }
}

#[derive(Clone, Copy)]
pub enum ActionKind {
    Key(KeyKind),
    Button(ButtonKind),
}
