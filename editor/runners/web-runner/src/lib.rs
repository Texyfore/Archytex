use std::sync::mpsc::{channel, Receiver, Sender};

use js_sys::Function;
use viewport::ipc::{IpcHost, IpcMessage};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub struct Channel {
    sender: Option<Sender<IpcMessage>>,
    receiver: Option<Receiver<IpcMessage>>,
}

#[wasm_bindgen]
impl Channel {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: Some(tx),
            receiver: Some(rx),
        }
    }

    #[wasm_bindgen(js_name = "browserEndpoint")]
    pub fn browser_endpoint(&mut self) -> BrowserEndpoint {
        BrowserEndpoint {
            sender: self.sender.take().unwrap(),
        }
    }

    #[wasm_bindgen(js_name = "wasmEndpoint")]
    pub fn wasm_endpoint(&mut self, on_log: Function, on_error: Function) -> WasmEndpoint {
        WasmEndpoint {
            receiver: self.receiver.take().unwrap(),
            on_log,
            on_error,
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct BrowserEndpoint {
    sender: Sender<IpcMessage>,
}

#[wasm_bindgen]
impl BrowserEndpoint {}

#[wasm_bindgen]
pub struct WasmEndpoint {
    receiver: Receiver<IpcMessage>,
    on_log: Function,
    on_error: Function,
}

impl IpcHost for WasmEndpoint {
    fn recv(&self) -> Option<IpcMessage> {
        self.receiver.try_recv().ok()
    }

    fn log(&self, message: String) {
        self.on_log
            .call1(&JsValue::null(), &JsValue::from(message))
            .unwrap();
    }

    fn error(&self, message: String) {
        self.on_error
            .call1(&JsValue::null(), &JsValue::from(message))
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn run(host: WasmEndpoint) {
    viewport::main(host);
}
