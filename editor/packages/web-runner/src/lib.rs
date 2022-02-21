use std::sync::mpsc;

use app::{FromHost, Host, Init, Resource, ToHost, Winit};
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub fn run(mut channel: Channel, callback: Callback) {
    app::run(Init {
        winit: winit(),
        resources: builtin_resources(),
        host: Box::new(callback),
        receiver: channel.rx.take().unwrap(),
    });
}

pub struct Channel {
    tx: Option<mpsc::Sender<FromHost>>,
    rx: Option<mpsc::Receiver<FromHost>>,
}

impl Channel {
    #[allow(clippy::new_without_default)]
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

pub struct Sender {
    tx: mpsc::Sender<FromHost>,
}

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

pub struct Callback;

impl Host for Callback {
    fn callback(&self, data: ToHost) {
        // todo
    }
}

fn winit() -> Winit {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::default().build(&event_loop).unwrap();
    Winit { event_loop, window }
}

fn builtin_resources() -> Vec<Resource> {
    vec![]
}
