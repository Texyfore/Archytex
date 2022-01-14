pub mod ipc;

use self::ipc::{IpcEndpoint, IpcMessageFrom, IpcMessageTo};

pub fn run<I: IpcEndpoint>(ipc_endpoint: I) {
    if let Some(IpcMessageFrom::CommentFromBackend(comment)) = ipc_endpoint.recv() {
        ipc_endpoint.send(IpcMessageTo::CommentToFrontend(format!(
            "Got your comment: `{}`",
            comment
        )));
    }
}
