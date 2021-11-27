mod editor;
mod input;
mod log;
mod math;
mod msg;
mod render;

use instant::Instant;

#[cfg(target_arch = "wasm32")]
use std::sync::mpsc::{channel, Sender};

use cgmath::{Matrix4, SquareMatrix};
use render::{Scene, SceneRenderer, SpritePass, TextureBank};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::render::WorldPass;

use self::{
    editor::Editor,
    input::{InputMapper, Trigger},
};

#[cfg(target_arch = "wasm32")]
use self::msg::Message;

macro_rules! message {
    ($msg:expr) => {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            $crate::handleMessage($msg);
        }
    };
}

macro_rules! textures {
    ($bank:ident $(,$id:literal => $path:literal)*) => {
        $(
            $bank.insert(
                $id,
                &image::load_from_memory(include_bytes!($path)).unwrap(),
            );
        )*
    };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(raw_module = "../glue.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn handleMessage(msg: &str);
}

#[cfg(target_arch = "wasm32")]
static mut MSG_IN: Option<Sender<Message>> = None;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::default().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    insert_canvas(&window);

    let mut main_loop = {
        let (width, height) = window.inner_size().into();
        let mut main_loop = MainLoop::init(window);
        main_loop.window_resized(width, height);
        main_loop
    };

    #[cfg(target_arch = "wasm32")]
    let msg_rx = {
        let (tx, rx) = channel();
        unsafe { MSG_IN = Some(tx) };
        rx
    };

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    main_loop.window_resized(width, height);
                }
                WindowEvent::CloseRequested => {
                    *flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(code),
                            state,
                            ..
                        },
                    ..
                } => {
                    main_loop.keyboard_input(code, state);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    main_loop.mouse_input(button, state);
                }
                WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y },
                    ..
                } => {
                    main_loop.mouse_moved([x as f32, y as f32]);
                }
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(_, y) => main_loop.scroll_wheel(y),
                    MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                        main_loop.scroll_wheel(y.signum() as f32)
                    }
                },
                _ => {}
            },
            Event::MainEventsCleared => {
                #[cfg(target_arch = "wasm32")]
                while let Ok(msg) = msg_rx.try_recv() {
                    main_loop.message_received(msg);
                }

                main_loop.process();
            }
            _ => {}
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn insert_canvas(window: &Window) {
    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&window.canvas())
        .unwrap();
}

struct MainLoop {
    _window: Window,
    before: Instant,
    renderer: SceneRenderer,
    texture_bank: TextureBank,
    input_mapper: InputMapper,
    editor: Editor,
}

impl MainLoop {
    fn init(window: Window) -> Self {
        let gfx_init = render::init(&window);
        let renderer = gfx_init.create_scene_renderer();
        let mut texture_bank = gfx_init.create_texture_bank();
        let solid_factory = gfx_init.create_solid_factory();
        let line_factory = gfx_init.create_line_factory();

        textures!(
            texture_bank,
            0 => "editor/vertex.png",
            10 => "editor/nodraw.png"
        );

        let mut input_mapper = InputMapper::default();
        let editor = Editor::init(solid_factory, line_factory, &mut input_mapper);

        Self {
            _window: window,
            before: Instant::now(),
            renderer,
            texture_bank,
            input_mapper,
            editor,
        }
    }

    fn window_resized(&mut self, width: u32, height: u32) {
        self.renderer.resize_viewport(width, height);
        self.editor.window_resized(width, height);
    }

    fn keyboard_input(&mut self, code: VirtualKeyCode, state: ElementState) {
        self.input_mapper.set_trigger(Trigger::Key(code), state);
    }

    fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input_mapper
            .set_trigger(Trigger::Button(button), state);
    }

    fn mouse_moved(&mut self, pos: [f32; 2]) {
        self.input_mapper.set_mouse_pos(pos);
    }

    fn scroll_wheel(&mut self, wheel: f32) {
        self.input_mapper.set_scroll_wheel(wheel);
    }

    #[cfg(target_arch = "wasm32")]
    fn message_received(&mut self, msg: Message) {
        match msg {
            Message::AddTexture { id, data } => {
                if let Ok(image) = image::load_from_memory(&data) {
                    self.texture_bank.insert(id, &image)
                }else {
                    error!("Received malformed texture");
                }
            },
        }
    }

    fn process(&mut self) {
        let after = Instant::now();
        let elapsed = (after - self.before).as_secs_f32();
        self.before = after;

        let mut scene = Scene {
            texture_bank: &self.texture_bank,
            world_pass: WorldPass {
                camera_matrix: Matrix4::identity(),
                solid_batches: Default::default(),
                line_batches: Default::default(),
            },
            sprite_pass: SpritePass {
                camera_matrix: Matrix4::identity(),
                sprites: Default::default(),
            },
        };

        self.editor.process(elapsed, &self.input_mapper, &self.texture_bank);

        self.input_mapper.tick();
        self.editor.render(&mut scene);
        self.renderer.render(scene);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "addTexture")]
pub fn add_texture(id: usize, data: Vec<u8>) {
    let sender = unsafe { MSG_IN.as_mut().unwrap() };
    sender.send(Message::AddTexture {id, data}).unwrap();
}
