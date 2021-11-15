mod editor;
mod input;
mod log;
mod math;
mod render;

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::{
    editor::Editor,
    input::{InputMapper, Trigger},
    render::Renderer,
};

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
    renderer: Renderer,
    input_mapper: InputMapper,
    editor: Editor<InputMapper, Renderer>,
}

impl MainLoop {
    fn init(window: Window) -> Self {
        let mut renderer = Renderer::new(&window);
        let mut input_mapper = InputMapper::default();
        let editor = Editor::init(&mut input_mapper, &mut renderer);

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
        }

        Self {
            _window: window,
            renderer,
            input_mapper,
            editor,
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

    fn mouse_moved(&mut self, pos: [f32; 2]) {
        self.input_mapper.set_mouse_pos(pos);
    }

    fn scroll_wheel(&mut self, wheel: f32) {
        self.input_mapper.set_scroll_wheel(wheel);
    }

    fn process(&mut self) {
        self.editor.process(&self.input_mapper, &mut self.renderer);
        self.input_mapper.tick();
        self.renderer.render();
    }
}
