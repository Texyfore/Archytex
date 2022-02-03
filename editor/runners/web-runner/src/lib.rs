use std::sync::mpsc::{channel, Receiver, Sender};

use asset_id::{GizmoID, PropID, TextureID};
use js_sys::{Function, Uint8Array};
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
        scene_dump_received: Function,
    ) -> WasmEndpoint {
        WasmEndpoint {
            receiver: self.receiver.take().unwrap(),
            editor_mode_changed,
            camera_speed_changed,
            grid_step_changed,
            scene_dump_received,
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
    #[wasm_bindgen(js_name = "addTexture")]
    pub fn add_texture(&mut self, id: u32, buf: Vec<u8>) {
        self.textures.push((id, buf));
    }

    #[wasm_bindgen(js_name = "addProp")]
    pub fn add_prop(&mut self, id: u32, buf: Vec<u8>) {
        self.props.push((id, buf));
    }

    #[wasm_bindgen(js_name = "addGizmo")]
    pub fn add_gizmo(&mut self, id: u32, buf: Vec<u8>) {
        self.gizmos.push((id, buf));
    }

    #[wasm_bindgen(js_name = "sendResources")]
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

    #[wasm_bindgen(js_name = "setResolution")]
    pub fn set_resolution(&self, width: u32, height: u32) {
        self.sender
            .send(IpcMessage::Resolution { width, height })
            .ok();
    }

    #[wasm_bindgen(js_name = "setEditorMode")]
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

    #[wasm_bindgen(js_name = "setGridStep")]
    pub fn set_grid_step(&self, step: i32) {
        self.sender.send(IpcMessage::GridStep(step)).ok();
    }

    #[wasm_bindgen(js_name = "setCurrentTexture")]
    pub fn set_current_texture(&self, id: u32) {
        self.sender
            .send(IpcMessage::CurrentTexture(TextureID(id)))
            .ok();
    }

    #[wasm_bindgen(js_name = "setCurrentProp")]
    pub fn set_current_prop(&self, id: u32) {
        self.sender.send(IpcMessage::CurrentProp(PropID(id))).ok();
    }

    #[wasm_bindgen(js_name = "requestCameraSpeed")]
    pub fn request_camera_speed(&self) {
        self.sender.send(IpcMessage::RequestCameraSpeed).ok();
    }

    #[wasm_bindgen(js_name = "requestGridStep")]
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
    scene_dump_received: Function,
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

    fn send_scene_dump(&self, buf: &[u8]) {
        let array = Uint8Array::from(buf);
        let array: JsValue = array.into();

        self.scene_dump_received
            .call1(&JsValue::NULL, &array)
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn run(host: WasmEndpoint) {
    console_error_panic_hook::set_once();
    viewport::main(host);
}
