use super::input::{ButtonKind, InputState, KeyKind};

pub enum Event {
    Initialized,
    RawInput(RawInputKind),
}

pub enum RawInputKind {
    Key(InputState, KeyKind),
    Button(InputState, ButtonKind),
    Movement(f32, f32),
}
