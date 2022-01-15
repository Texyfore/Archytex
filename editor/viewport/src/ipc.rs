pub trait IpcHost {
    fn recv(&self) -> Option<IpcMessage>;
    fn log(&self, message: String);
    fn error(&self, message: String);
}

pub enum IpcMessage {}
