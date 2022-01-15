use std::collections::HashMap;

use winit::event::{ElementState, MouseButton, VirtualKeyCode};

#[derive(Default)]
pub struct Input {
    key_states: HashMap<VirtualKeyCode, ActionState>,
    button_states: HashMap<MouseButton, ActionState>,
}

impl Input {
    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.key_states
            .get(&key)
            .map(|state| state.is_down())
            .unwrap_or(false)
    }

    pub fn is_key_down_once(&self, key: VirtualKeyCode) -> bool {
        self.key_states
            .get(&key)
            .map(|state| state.is_down_once())
            .unwrap_or(false)
    }

    pub fn was_key_down_once(&self, key: VirtualKeyCode) -> bool {
        self.key_states
            .get(&key)
            .map(|state| state.was_down_once())
            .unwrap_or(false)
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.button_states
            .get(&button)
            .map(|state| state.is_down())
            .unwrap_or(false)
    }

    pub fn is_button_down_once(&self, button: MouseButton) -> bool {
        self.button_states
            .get(&button)
            .map(|state| state.is_down_once())
            .unwrap_or(false)
    }

    pub fn was_button_down_once(&self, button: MouseButton) -> bool {
        self.button_states
            .get(&button)
            .map(|state| state.was_down_once())
            .unwrap_or(false)
    }

    pub fn process(&mut self) {
        for state in self.key_states.values_mut() {
            state.increment();
        }

        for state in self.button_states.values_mut() {
            state.increment();
        }
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.key_states.entry(key).or_default().set(state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.button_states.entry(button).or_default().set(state);
    }
}

enum ActionState {
    Active(i32),
    Inactive(i32),
}

impl Default for ActionState {
    fn default() -> Self {
        Self::Inactive(0)
    }
}

impl ActionState {
    fn set(&mut self, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if matches!(self, Self::Inactive(_)) {
                    *self = Self::Active(0);
                }
            }
            ElementState::Released => {
                if matches!(self, Self::Active(_)) {
                    *self = Self::Inactive(0);
                }
            }
        }
    }

    fn increment(&mut self) {
        match self {
            ActionState::Active(t) => *t += 1,
            ActionState::Inactive(t) => *t += 1,
        }
    }

    fn is_down(&self) -> bool {
        matches!(self, ActionState::Active(_))
    }

    fn is_down_once(&self) -> bool {
        if let ActionState::Active(t) = self {
            *t == 0
        } else {
            false
        }
    }

    fn was_down_once(&self) -> bool {
        if let ActionState::Inactive(t) = self {
            *t == 0
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use winit::event::{ElementState, MouseButton, VirtualKeyCode};

    use super::Input;

    #[test]
    fn is_key_down() {
        let mut input = Input::default();

        assert!(!input.is_key_down(VirtualKeyCode::A));
        input.keyboard_input(VirtualKeyCode::A, ElementState::Pressed);
        assert!(input.is_key_down(VirtualKeyCode::A));
        input.process();
        assert!(input.is_key_down(VirtualKeyCode::A));
        input.keyboard_input(VirtualKeyCode::A, ElementState::Released);
        assert!(!input.is_key_down(VirtualKeyCode::A));
    }

    #[test]
    fn is_key_down_once() {
        let mut input = Input::default();

        assert!(!input.is_key_down_once(VirtualKeyCode::A));
        input.keyboard_input(VirtualKeyCode::A, ElementState::Pressed);
        assert!(input.is_key_down_once(VirtualKeyCode::A));
        input.process();
        assert!(!input.is_key_down_once(VirtualKeyCode::A));
    }

    #[test]
    fn was_key_down_once() {
        let mut input = Input::default();

        assert!(!input.was_key_down_once(VirtualKeyCode::A));
        input.keyboard_input(VirtualKeyCode::A, ElementState::Pressed);
        assert!(!input.was_key_down_once(VirtualKeyCode::A));
        input.keyboard_input(VirtualKeyCode::A, ElementState::Released);
        assert!(input.was_key_down_once(VirtualKeyCode::A));
    }

    #[test]
    fn is_button_down() {
        let mut input = Input::default();

        assert!(!input.is_button_down(MouseButton::Left));
        input.mouse_input(MouseButton::Left, ElementState::Pressed);
        assert!(input.is_button_down(MouseButton::Left));
        input.process();
        assert!(input.is_button_down(MouseButton::Left));
        input.mouse_input(MouseButton::Left, ElementState::Released);
        assert!(!input.is_button_down(MouseButton::Left));
    }

    #[test]
    fn is_button_down_once() {
        let mut input = Input::default();

        assert!(!input.is_button_down_once(MouseButton::Left));
        input.mouse_input(MouseButton::Left, ElementState::Pressed);
        assert!(input.is_button_down_once(MouseButton::Left));
        input.process();
        assert!(!input.is_button_down_once(MouseButton::Left));
    }

    #[test]
    fn was_button_down_once() {
        let mut input = Input::default();

        assert!(!input.was_button_down_once(MouseButton::Left));
        input.mouse_input(MouseButton::Left, ElementState::Pressed);
        assert!(!input.was_button_down_once(MouseButton::Left));
        input.mouse_input(MouseButton::Left, ElementState::Released);
        assert!(input.was_button_down_once(MouseButton::Left));
    }
}
