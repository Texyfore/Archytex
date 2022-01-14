use std::sync::mpsc::{channel, Receiver, Sender};

use viewport::ipc::{IpcEndpoint, IpcMessageFrom, IpcMessageTo};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebIpcChannel {
    front: Option<(Sender<IpcMessageFrom>, Receiver<IpcMessageTo>)>,
    back: Option<(Sender<IpcMessageTo>, Receiver<IpcMessageFrom>)>,
}

#[wasm_bindgen]
impl WebIpcChannel {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (from_tx, from_rx) = channel();
        let (to_tx, to_rx) = channel();

        Self {
            front: Some((from_tx, to_rx)),
            back: Some((to_tx, from_rx)),
        }
    }

    pub fn frontend(&mut self) -> WebIpcFrontend {
        let (sender, receiver) = self.front.take().unwrap();
        WebIpcFrontend { sender, receiver }
    }

    pub fn backend(&mut self) -> WebIpcBackend {
        let (sender, receiver) = self.back.take().unwrap();
        WebIpcBackend { sender, receiver }
    }
}

#[wasm_bindgen]
pub struct WebIpcFrontend {
    sender: Sender<IpcMessageFrom>,
    receiver: Receiver<IpcMessageTo>,
}

#[wasm_bindgen]
impl WebIpcFrontend {
    pub fn send_comment(&self, comment: String) {
        self.sender
            .send(IpcMessageFrom::CommentFromBackend(comment))
            .unwrap();
    }

    pub fn recv_comment(&self) -> Option<String> {
        if let Ok(IpcMessageTo::CommentToFrontend(comment)) = self.receiver.try_recv() {
            Some(comment)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub struct WebIpcBackend {
    sender: Sender<IpcMessageTo>,
    receiver: Receiver<IpcMessageFrom>,
}

impl IpcEndpoint for WebIpcBackend {
    fn send(&self, message: IpcMessageTo) {
        self.sender.send(message).unwrap();
    }

    fn recv(&self) -> Option<IpcMessageFrom> {
        self.receiver.try_recv().ok()
    }
}

#[wasm_bindgen]
pub fn start(ipc_backend: WebIpcBackend) {
    viewport::run(ipc_backend);
}
