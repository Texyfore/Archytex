use std::sync::mpsc;

use wasm_bindgen::{prelude::*, JsCast};

use app::{FromHost, Host, Init, Resource, ResourceKind, ToHost, Winit};
use winit::{event_loop::EventLoop, platform::web::WindowBuilderExtWebSys, window::WindowBuilder};

#[wasm_bindgen]
pub fn run(mut channel: Channel, callback: Callback) {
    app::run(Init {
        winit: winit(),
        resources: builtin_resources(),
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
    pub fn set_resolution(&self, width: u32, height: u32) {
        self.tx
            .send(FromHost::Resolution { width, height })
            .unwrap();
    }

    pub fn save_scene(&self) {
        self.tx.send(FromHost::SaveScene).unwrap();
    }

    pub fn load_scene(&self, buf: Vec<u8>) {
        self.tx.send(FromHost::LoadScene(buf)).unwrap();
    }

    pub fn set_texture(&self, id: u32) {
        self.tx.send(FromHost::Texture(id)).unwrap();
    }

    pub fn set_prop(&self, id: u32) {
        self.tx.send(FromHost::Prop(id)).unwrap();
    }
}

#[wasm_bindgen]
pub struct Callback;

#[wasm_bindgen]
impl Callback {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }
}

impl Host for Callback {
    fn callback(&self, _data: ToHost) {
        // todo
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
            buf: include_bytes!("../../../assets/builtin/nodraw.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 1,
            buf: include_bytes!("../../../assets/builtin/ground.png").to_vec(),
            kind: ResourceKind::Texture,
        },
        Resource {
            id: 0,
            buf: include_bytes!("../../../assets/builtin/vertex.agzm").to_vec(),
            kind: ResourceKind::Gizmo,
        },
    ]
}
