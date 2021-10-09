use winit::event::{ElementState, MouseButton, VirtualKeyCode};

pub enum InputState {
    Pressed,
    Released,
}

impl From<ElementState> for InputState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

pub enum KeyKind {
    LControl,
    Unknown,
}

impl From<VirtualKeyCode> for KeyKind {
    fn from(value: VirtualKeyCode) -> Self {
        match value {
            VirtualKeyCode::LControl => Self::LControl,
            _ => Self::Unknown,
        }
    }
}

pub enum ButtonKind {
    Left,
    Right,
    Middle,
    Unknown,
}

impl From<MouseButton> for ButtonKind {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
            MouseButton::Other(_) => Self::Unknown,
        }
    }
}
