#![allow(clippy::unused_unit)]

use std::sync::mpsc;

use js_sys::{Function, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};

use app::{builtin_resources, Ascn, FromHost, Host, Init, Resource, ResourceKind, ToHost, Winit};
use winit::{event_loop::EventLoop, platform::web::WindowBuilderExtWebSys, window::WindowBuilder};

#[wasm_bindgen]
pub fn run(mut channel: Channel, callback: Callback) {
    console_error_panic_hook::set_once();

    app::run(Init {
        winit: winit(),
        host: Box::new(callback),
        receiver: channel.rx.take().unwrap(),
    });
}

#[wasm_bindgen]
pub struct Channel {
    tx: Option<mpsc::Sender<FromHost>>,
    rx: Option<mpsc::Receiver<FromHost>>,
}

#[wasm_bindgen]
impl Channel {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        for resource in builtin_resources() {
            tx.send(FromHost::LoadResource(resource)).unwrap();
        }

        Self {
            tx: Some(tx),
            rx: Some(rx),
        }
    }

    pub fn sender(&mut self) -> Sender {
        Sender {
            tx: self.tx.take().unwrap(),
        }
    }
}

#[wasm_bindgen]
pub struct Sender {
    tx: mpsc::Sender<FromHost>,
}

#[wasm_bindgen]
impl Sender {
    #[wasm_bindgen(js_name = "setResolution")]
    pub fn set_resolution(&self, width: u32, height: u32) {
        self.tx
            .send(FromHost::Resolution { width, height })
            .unwrap();
    }

    #[wasm_bindgen(js_name = "saveScene")]
    pub fn save_scene(&self, id: i32) {
        self.tx.send(FromHost::SaveScene(id)).unwrap();
    }

    #[wasm_bindgen(js_name = "loadScene")]
    pub fn load_scene(&self, buf: Vec<u8>) {
        self.tx.send(FromHost::LoadScene(buf)).unwrap();
    }

    #[wasm_bindgen(js_name = "setTexture")]
    pub fn set_texture(&self, id: u32) {
        self.tx.send(FromHost::Texture(id)).unwrap();
    }

    #[wasm_bindgen(js_name = "setProp")]
    pub fn set_prop(&self, id: u32) {
        self.tx.send(FromHost::Prop(id)).unwrap();
    }

    pub fn movement(&self, x: f32, y: f32) {
        self.tx.send(FromHost::Movement(x, y)).unwrap();
    }

    #[wasm_bindgen(js_name = "setPointerLock")]
    pub fn set_pointer_lock(&self, lock: bool) {
        self.tx.send(FromHost::LockPointer(lock)).unwrap();
    }

    pub fn button(&self, index: i32) {
        self.tx.send(FromHost::Button(index)).unwrap();
    }

    #[wasm_bindgen(js_name = "loadTexture")]
    pub fn load_texture(&self, id: u32, buf: Vec<u8>) {
        self.tx
            .send(FromHost::LoadResource(Resource {
                id,
                buf,
                kind: ResourceKind::Texture,
            }))
            .unwrap();
    }

    #[wasm_bindgen(js_name = "loadProp")]
    pub fn load_prop(&self, id: u32, buf: Vec<u8>) {
        self.tx
            .send(FromHost::LoadResource(Resource {
                id,
                buf,
                kind: ResourceKind::Prop,
            }))
            .unwrap();
    }
}

#[wasm_bindgen]
pub struct Callback {
    scene_saved: Function,
    button_feedback: Function,
}

#[wasm_bindgen]
impl Callback {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new(scene_saved: Function, button_feedback: Function) -> Self {
        Self {
            scene_saved,
            button_feedback,
        }
    }
}

impl Host for Callback {
    fn callback(&self, data: ToHost) {
        match data {
            ToHost::SceneSaved(id, buf) => {
                self.scene_saved
                    .call2(
                        &JsValue::NULL,
                        &JsValue::from(id),
                        &Uint8Array::from(buf.as_slice()),
                    )
                    .ok();
            }
            ToHost::Button(button) => {
                self.button_feedback
                    .call1(&JsValue::NULL, &JsValue::from(button))
                    .ok();
            }
        }
    }
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_canvas(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("viewport-canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .ok(),
        )
        .build(&event_loop)
        .unwrap();

    Winit { event_loop, window }
}

#[wasm_bindgen]
pub struct Scene {
    inner: Ascn,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: &[u8]) -> Option<Scene> {
        Ascn::new(buf).map(|inner| Self { inner })
    }

    pub fn textures(&self) -> Vec<u32> {
        self.inner.textures()
    }

    pub fn props(&self) -> Vec<u32> {
        self.inner.props()
    }
}
