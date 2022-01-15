pub trait IpcHost {
    fn recv(&self) -> Option<IpcMessage>;
    fn fatal_error(&self, message: String);
}

pub enum IpcMessage {
    Comment(String),
}
