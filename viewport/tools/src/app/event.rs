use super::input::{ButtonKind, InputState, KeyKind};
use serde::Deserialize;

pub enum Event {
    Initialized,
    Resized(u32, u32),
    RawInput(RawInputKind),
    FrontendMessage(MessageKind),
}

pub enum RawInputKind {
    Key(InputState, KeyKind),
    Button(InputState, ButtonKind),
    Movement(f32, f32),
    Wheel(f32),
}

#[derive(Debug, Deserialize)]
pub enum MessageKind {
    Dummy(String),
}
