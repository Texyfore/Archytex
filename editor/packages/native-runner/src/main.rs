use std::sync::mpsc::channel;

use app::{run, Host, Init, Resource, ResourceKind, ToHost, Winit};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let (_sender, receiver) = channel();
    run(Init {
        winit: winit(),
        resources: vec![Resource {
            id: 0,
            buf: include_bytes!("../assets/nodraw.png").to_vec(),
            kind: ResourceKind::Texture,
        }],
        host: Box::new(NativeHost),
        receiver,
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

pub struct NativeHost;

impl Host for NativeHost {
    fn callback(&self, _data: ToHost) {
        println!("[native-runner] ignoring callback");
    }
}
