use winit::event_loop::EventLoop;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self
    }
}

impl App {
    pub fn run<M: MainLoop>(mut self, mut main_loop: M) {
        let event_loop = EventLoop::new();
    }
}

pub trait MainLoop {
    fn process(app: &mut App);
}