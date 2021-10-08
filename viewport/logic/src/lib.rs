use tools::{
    app::{
        event::{Event, InputState, KeyKind, RawInputKind},
        App, MainLoop,
    },
    console,
};

pub struct Viewport;

impl Default for Viewport {
    fn default() -> Self {
        Self
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(Event::RawInput(RawInputKind::Key(state, key))) = app.poll_event() {
            let state = match state {
                InputState::Pressed => "pressed",
                InputState::Released => "released",
            };

            let key = match key {
                KeyKind::LControl => "lctrl",
                KeyKind::Unknown => "unknown",
            };

            console!("Key {} is {}", key, state);
        }
    }
}
