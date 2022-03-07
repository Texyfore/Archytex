mod comms;

use std::{
    fs,
    sync::mpsc::channel,
    time::{SystemTime, UNIX_EPOCH},
};

use app::{run, FromHost, Host, Init, Resource, ToHost, Winit};
use comms::AsyncStdin;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let (sender, receiver) = channel();

    for resource in builtin_resources() {
        sender.send(FromHost::LoadResource(resource)).unwrap();
    }

    let _stdin = AsyncStdin::new(sender);

    run(Init {
        winit: winit(),
        host: Box::new(NativeHost),
        receiver,
    });
}

pub struct NativeHost;

impl Host for NativeHost {
    fn callback(&self, data: ToHost) {
        match data {
            ToHost::SceneSaved(_, scene) => {
                let stamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let fname = format!("{}.ascn", stamp);
                fs::write(&fname, &scene).unwrap();
                println!("[native-runner] saving scene `{}`", fname);
            }
            ToHost::Button(button) => {
                println!("[native-runner] button feedback for {}", button);
            }
        }
    }
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_title("Archytex")
        .build(&event_loop)
        .unwrap();

    Winit { event_loop, window }
}

macro_rules! resource {
    ($ty:ident $id:literal -> $path:literal) => {
        app::Resource {
            id: $id,
            buf: include_bytes!(concat!("../../../assets/", $path)).to_vec(),
            kind: app::ResourceKind::$ty,
        }
    };
}

fn builtin_resources() -> Vec<Resource> {
    vec![
        resource!(Texture 0 -> "nodraw.png"),
        resource!(Texture 1 -> "ground.png"),
        resource!(Gizmo 0 -> "vertex.agzm"),
        resource!(Gizmo 1 -> "arrow.agzm"),
        resource!(Gizmo 2 -> "plane.agzm"),
        resource!(Gizmo 3 -> "arc.agzm"),
    ]
}
