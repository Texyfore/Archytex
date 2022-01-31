use viewport::ipc::{IpcHost, IpcMessage};

fn main() {
    viewport::main(DummyIpcHost);
}

struct DummyIpcHost;

impl IpcHost for DummyIpcHost {
    fn recv(&self) -> Option<IpcMessage> {
        None
    }

    fn send_editor_mode(&self, _mode: i32) {}

    fn send_camera_speed(&self, _speed: i32) {}

    fn send_grid_step(&self, _step: i32) {}
}
