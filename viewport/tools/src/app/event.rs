pub enum Event {
    Initialized,
    RawInput(RawInputKind),
}

pub enum RawInputKind {
    Key(InputState, KeyKind),
    Button(InputState, ButtonKind),
    Movement(f32, f32),
}

pub enum InputState {
    Pressed,
    Released,
}

pub enum KeyKind {
    Unknown,
}

pub enum ButtonKind {
    Left,
    Right,
    Middle,
    Unknown,
}
