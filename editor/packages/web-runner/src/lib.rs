#![allow(clippy::unused_unit)]

use std::sync::mpsc;

use js_sys::{Function, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};

use app::{FromHost, Host, Init, Resource, ResourceKind, ToHost, Winit};
use winit::{event_loop::EventLoop, platform::web::WindowBuilderExtWebSys, window::WindowBuilder};

#[wasm_bindgen]
pub fn run(mut channel: Channel, callback: Callback, resources: Resources) {
    console_error_panic_hook::set_once();

    let mut imported = resources.vec;
    let mut resources = builtin_resources();
    resources.append(&mut imported);

    app::run(Init {
        winit: winit(),
        resources,
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
    pub fn save_scene(&self) {
        self.tx.send(FromHost::SaveScene).unwrap();
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
            ToHost::SceneSaved(buf) => {
                self.scene_saved
                    .call1(&JsValue::NULL, &Uint8Array::from(buf.as_slice()))
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

#[wasm_bindgen]
pub struct Resources {
    vec: Vec<Resource>,
}

#[wasm_bindgen]
impl Resources {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    #[wasm_bindgen(js_name = "addTexture")]
    pub fn add_texture(&mut self, id: u32, buf: Vec<u8>) {
        self.vec.push(Resource {
            id,
            buf,
            kind: ResourceKind::Texture,
        });
    }

    #[wasm_bindgen(js_name = "addProp")]
    pub fn add_prop(&mut self, id: u32, buf: Vec<u8>) {
        self.vec.push(Resource {
            id,
            buf,
            kind: ResourceKind::Prop,
        });
    }

    #[wasm_bindgen(js_name = "addGizmo")]
    pub fn add_gizmo(&mut self, id: u32, buf: Vec<u8>) {
        self.vec.push(Resource {
            id,
            buf,
            kind: ResourceKind::Gizmo,
        });
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

fn builtin_resources() -> Vec<Resource> {
    vec![
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/nodraw.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../../../assets/ground.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/vertex.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../../../assets/arrow.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 2,
            buf: include_bytes!("../../../assets/plane.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
        Resource {
            id: 3,
            buf: include_bytes!("../../../assets/arc.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
    ]
}
