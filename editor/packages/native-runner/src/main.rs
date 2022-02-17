use std::{
    fs,
    io::stdin,
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
    time::{SystemTime, UNIX_EPOCH},
};

use app::{run, FromHost, Host, Init, Resource, ResourceKind, ToHost, Winit};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let (sender, receiver) = channel();
    let handle = stdin_listen(sender);
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
            ToHost::SceneSaved(scene) => {
                let stamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let fname = format!("{}.ascn", stamp);
                fs::write(&fname, &scene).unwrap();
                println!("[native-runner] saving scene `{}`", fname);
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
            buf: include_bytes!("../assets/nodraw.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../assets/bricks.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../assets/vertex.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../assets/table.amdl").to_vec(),
            kind: ResourceKind::Prop,
        },
    ]
}

fn stdin_listen(tx: Sender<FromHost>) -> JoinHandle<()> {
    let handle = thread::spawn(move || loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        match buf.as_str() {
            "r" => {
                tx.send(FromHost::Resolution {
                    width: 400,
                    height: 400,
                })
                .unwrap();
            }
            "s" => {
                tx.send(FromHost::SaveScene).unwrap();
            }
            "e" => break,
            _ => (),
        };
    });

    handle
}
