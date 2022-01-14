pub trait IpcEndpoint {
    fn send(&self, message: IpcMessageTo);
    fn recv(&self) -> Option<IpcMessageFrom>;
}

pub enum IpcMessageTo {
    CommentToFrontend(String),
}

pub enum IpcMessageFrom {
    CommentFromBackend(String),
}
