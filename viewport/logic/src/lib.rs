mod input;

use input::InputMapper;
use tools::app::{App, MainLoop};

#[derive(Default)]
pub struct Viewport {
    input_mapper: InputMapper,
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {

    }
}
