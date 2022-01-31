use std::sync::mpsc::{channel, Receiver, Sender};

use asset_id::{GizmoID, PropID, TextureID};
use js_sys::Function;
use viewport::ipc::{EditorMode, IpcHost, IpcMessage};
use wasm_bindgen::prelude::*;

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
            textures: Vec::new(),
            props: Vec::new(),
            gizmos: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name = "wasmEndpoint")]
    pub fn wasm_endpoint(
        &mut self,
        editor_mode_changed: Function,
        camera_speed_changed: Function,
        grid_step_changed: Function,
    ) -> WasmEndpoint {
        WasmEndpoint {
            receiver: self.receiver.take().unwrap(),
            editor_mode_changed,
            camera_speed_changed,
            grid_step_changed,
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct BrowserEndpoint {
    sender: Sender<IpcMessage>,
    textures: Vec<(u32, Vec<u8>)>,
    props: Vec<(u32, Vec<u8>)>,
    gizmos: Vec<(u32, Vec<u8>)>,
}

#[wasm_bindgen]
impl BrowserEndpoint {
    pub fn add_texture(&mut self, id: u32, buf: Vec<u8>) {
        self.textures.push((id, buf));
    }

    pub fn add_props(&mut self, id: u32, buf: Vec<u8>) {
        self.props.push((id, buf));
    }

    pub fn add_gizmos(&mut self, id: u32, buf: Vec<u8>) {
        self.gizmos.push((id, buf));
    }

    pub fn send_resources(&mut self) {
        self.sender
            .send(IpcMessage::Resources {
                textures: self
                    .textures
                    .drain(..)
                    .map(|(id, buf)| (TextureID(id), buf))
                    .collect(),
                props: self
                    .props
                    .drain(..)
                    .map(|(id, buf)| (PropID(id), buf))
                    .collect(),
                gizmos: self
                    .gizmos
                    .drain(..)
                    .map(|(id, buf)| (GizmoID(id), buf))
                    .collect(),
            })
            .ok();
    }

    pub fn set_editor_mode(&self, mode: i32) {
        self.sender
            .send(IpcMessage::EditorMode(match mode {
                0 => EditorMode::Solid,
                1 => EditorMode::Face,
                2 => EditorMode::Point,
                3 => EditorMode::Prop,
                _ => panic!(),
            }))
            .ok();
    }

    pub fn set_grid_step(&self, step: i32) {
        self.sender.send(IpcMessage::GridStep(step)).ok();
    }

    pub fn set_current_texture(&self, id: u32) {
        self.sender
            .send(IpcMessage::CurrentTexture(TextureID(id)))
            .ok();
    }

    pub fn set_current_prop(&self, id: u32) {
        self.sender.send(IpcMessage::CurrentProp(PropID(id))).ok();
    }

    pub fn request_camera_speed(&self) {
        self.sender.send(IpcMessage::RequestCameraSpeed).ok();
    }

    pub fn request_grid_step(&self) {
        self.sender.send(IpcMessage::RequestGridStep).ok();
    }
}

#[wasm_bindgen]
pub struct WasmEndpoint {
    receiver: Receiver<IpcMessage>,
    editor_mode_changed: Function,
    camera_speed_changed: Function,
    grid_step_changed: Function,
}

impl IpcHost for WasmEndpoint {
    fn recv(&self) -> Option<IpcMessage> {
        self.receiver.try_recv().ok()
    }
    
    fn send_editor_mode(&self, mode: i32) {
        self.editor_mode_changed
            .call1(&JsValue::NULL, &JsValue::from(mode))
            .unwrap();
    }

    fn send_camera_speed(&self, speed: i32) {
        self.camera_speed_changed
            .call1(&JsValue::NULL, &JsValue::from(speed))
            .unwrap();
    }

    fn send_grid_step(&self, step: i32) {
        self.grid_step_changed
            .call1(&JsValue::NULL, &JsValue::from(step))
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn run(host: WasmEndpoint) {
    console_error_panic_hook::set_once();
    viewport::main(host);
}
