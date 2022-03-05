mod comms;

use std::{
    fs,
    sync::mpsc::channel,
    time::{SystemTime, UNIX_EPOCH},
};

use app::{run, Host, Init, Resource, ResourceKind, ToHost, Winit};
use comms::AsyncStdin;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let (sender, receiver) = channel();
    let _stdin = AsyncStdin::new(sender);
    run(Init {
        winit: winit(),
        resources: resources(),
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

fn resources() -> Vec<Resource> {
    vec![
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/nodraw.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../../../assets/ground.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/vertex.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../../../assets/arrow.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 2,
            buf: include_bytes!("../../../assets/plane.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 3,
            buf: include_bytes!("../../../assets/arc.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/cube.amdl").to_vec(),
            kind: ResourceKind::Prop,
        },
    ]
}
