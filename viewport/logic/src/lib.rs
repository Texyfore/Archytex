mod input;

use input::InputMapper;
use tools::app::{App, MainLoop};

pub struct Viewport {
    input_mapper: InputMapper,
}

impl Default for Viewport {
    fn default() -> Self {
        let mut input_mapper = InputMapper::default();
        Self { input_mapper }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {}
}
