use viewport::ipc::{IpcHost, IpcMessage};

fn main() {
    viewport::main(DummyIpcHost);
}

struct DummyIpcHost;

impl IpcHost for DummyIpcHost {
    fn recv(&self) -> Option<IpcMessage> {
        None
    }

    fn log(&self, message: String) {
        println!("{}", message);
    }

    fn error(&self, message: String) {
        panic!("{}", message)
    }
}
