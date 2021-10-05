use tools::app::{App, MainLoop};

pub struct Viewport;

impl Default for Viewport {
    fn default() -> Self {
        Self
    }
}

impl MainLoop for Viewport {
    fn process(app: &mut App) {}
}
