use tools::{
    app::{App, MainLoop},
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
        console!("Hello, World!");
    }
}
