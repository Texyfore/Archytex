mod input;
mod mesh;
mod model;

use input::InputMapper;
use tools::app::{event::Event, App, MainLoop};

pub struct Viewport {
    input_mapper: InputMapper,
}

impl Default for Viewport {
    fn default() -> Self {
        let input_mapper = InputMapper::default();
        Self { input_mapper }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {}
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }
    }
}
