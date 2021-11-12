mod input;
mod log;
mod render;

use render::Renderer;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::input::{InputMapper, Trigger};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::default().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    insert_canvas(&window);

    let mut main_loop = MainLoop::init(window);

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
                _ => {}
            },
            Event::MainEventsCleared => {
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
    window: Window,
    renderer: Renderer,
    input_mapper: InputMapper,
}

impl MainLoop {
    fn init(window: Window) -> Self {
        let mut renderer = Renderer::new(&window);
        let input_mapper = InputMapper::new(&[]);

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
        }

        Self {
            window,
            renderer,
            input_mapper,
        }
    }

    fn window_resized(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    fn keyboard_input(&mut self, code: VirtualKeyCode, state: ElementState) {
        self.input_mapper.set_trigger(Trigger::Key(code), state);
    }

    fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input_mapper
            .set_trigger(Trigger::Button(button), state);
    }

    fn process(&mut self) {
        self.input_mapper.tick();
        self.renderer.render();
    }
}
