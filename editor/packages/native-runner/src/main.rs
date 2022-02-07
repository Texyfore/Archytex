use app::{run, Callbacks, Winit};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let winit = winit();
    let callbacks = callbacks();
    run(winit, callbacks);
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_title("Archytex")
        .build(&event_loop)
        .unwrap();

    Winit { event_loop, window }
}

fn callbacks() -> Callbacks {
    Callbacks {
        save: Box::new(save),
    }
}

fn save(scene: &[u8]) {
    println!("[runner] ignoring saved scene (size: {}b)", scene.len());
}
