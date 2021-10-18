use winit::event::{ElementState, MouseButton, VirtualKeyCode};

macro_rules! keys {
    ($($name:ident),*,) => {
        #[derive(PartialEq, Eq, Hash, Debug)]
        pub enum KeyKind {
            $($name,)*
            Unknown,
        }

        impl From<VirtualKeyCode> for KeyKind {
            fn from(value: VirtualKeyCode) -> Self {
                match value {
                    $(VirtualKeyCode::$name => KeyKind::$name,)*
                    _ => KeyKind::Unknown,
                }
            }
        }
    };
}

#[derive(Clone, Copy)]
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

keys![LShift, LControl, W, S, A, D, E, Q,];

#[derive(PartialEq, Eq, Hash)]
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
