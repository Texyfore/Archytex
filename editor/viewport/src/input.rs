use std::collections::HashMap;

use cgmath::{Vector2, Zero};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

pub struct Input {
    key_states: HashMap<VirtualKeyCode, ActionState>,
    button_states: HashMap<MouseButton, ActionState>,
    mouse_pos_before: Vector2<f32>,
    mouse_pos: Vector2<f32>,
    mouse_wheel: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            key_states: HashMap::new(),
            button_states: HashMap::new(),
            mouse_pos_before: Vector2::zero(),
            mouse_pos: Vector2::zero(),
            mouse_wheel: 0.0,
        }
    }
}

#[allow(dead_code)]
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

    pub fn mouse_pos(&self) -> Vector2<f32> {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> Vector2<f32> {
        self.mouse_pos - self.mouse_pos_before
    }

    pub fn mouse_wheel(&self) -> f32 {
        self.mouse_wheel
    }

    pub fn process(&mut self) {
        for state in self.key_states.values_mut() {
            state.increment();
        }

        for state in self.button_states.values_mut() {
            state.increment();
        }

        self.mouse_pos_before = self.mouse_pos;
        self.mouse_wheel = 0.0;
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.key_states.entry(key).or_default().set(state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.button_states.entry(button).or_default().set(state);
    }

    pub fn mouse_movement(&mut self, new_pos: Vector2<f32>) {
        self.mouse_pos = new_pos;
    }

    pub fn mouse_wheel_movement(&mut self, movement: f32) {
        self.mouse_wheel = movement;
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
    use cgmath::{Vector2, Zero};
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

    #[test]
    fn mouse_pos() {
        let mut input = Input::default();

        assert_eq!(input.mouse_pos(), Vector2::zero());
        input.mouse_movement(Vector2::new(1.0, 1.0));
        assert_eq!(input.mouse_pos(), Vector2::new(1.0, 1.0));
        input.process();
        assert_eq!(input.mouse_pos(), Vector2::new(1.0, 1.0));
    }

    #[test]
    fn mouse_delta() {
        let mut input = Input::default();

        assert_eq!(input.mouse_delta(), Vector2::zero());
        input.mouse_movement(Vector2::new(1.0, 1.0));
        assert_eq!(input.mouse_delta(), Vector2::new(1.0, 1.0));
        input.process();
        assert_eq!(input.mouse_delta(), Vector2::zero());
    }

    #[test]
    fn mouse_wheel() {
        let mut input = Input::default();

        assert_eq!(input.mouse_wheel(), 0.0);
        input.mouse_wheel_movement(1.0);
        assert_eq!(input.mouse_wheel(), 1.0);
        input.process();
        assert_eq!(input.mouse_wheel(), 0.0);
    }
}
