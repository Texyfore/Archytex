use app::{run, Init, OnSave, Winit};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    run(Init {
        winit: winit(),
        save_handler: Box::new(IgnoreSave),
        resources: vec![],
    });
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_title("Archytex")
        .build(&event_loop)
        .unwrap();

    Winit { event_loop, window }
}

pub struct IgnoreSave;

impl OnSave for IgnoreSave {
    fn on_save(&self, buf: &[u8]) {
        println!("[runner] ignoring saved scene ({} bytes)", buf.len());
    }
}
