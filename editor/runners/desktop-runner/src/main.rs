use std::sync::mpsc::{channel, Receiver};

use asset_id::{GizmoID, TextureID};
use viewport::ipc::{IpcHost, IpcMessage};

fn main() {
    let (tx, rx) = channel();

    tx.send(IpcMessage::Resources {
        textures: vec![
            (
                TextureID(0),
                include_bytes!("../assets/nodraw.png").to_vec(),
            ),
            (
                TextureID(1),
                include_bytes!("../assets/bricks.png").to_vec(),
            ),
        ],
        props: vec![],
        gizmos: vec![(GizmoID(0), include_bytes!("../assets/gizmo.agzm").to_vec())],
    })
    .unwrap();

    tx.send(IpcMessage::CurrentTexture(TextureID(1))).unwrap();

    viewport::main(DummyIpcHost { receiver: rx });
}

struct DummyIpcHost {
    receiver: Receiver<IpcMessage>,
}

impl IpcHost for DummyIpcHost {
    fn recv(&self) -> Option<IpcMessage> {
        self.receiver.try_recv().ok()
    }

    fn send_editor_mode(&self, _mode: i32) {}

    fn send_camera_speed(&self, _speed: i32) {}

    fn send_grid_step(&self, _step: i32) {}
}
