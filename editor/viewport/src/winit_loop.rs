use cgmath::vec2;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{ipc::IpcHost, main_loop::MainLoop};

pub struct WinitLoop {
    event_loop: EventLoop<()>,
    window: Window,
}

impl WinitLoop {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::default().build(&event_loop).unwrap();
        Self { event_loop, window }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        use wasm_bindgen::JsCast;
        use web_sys::HtmlCanvasElement;
        use winit::platform::web::WindowBuilderExtWebSys;

        let event_loop = EventLoop::new();
        let window = WindowBuilder::default()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("viewport-canvas")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap(),
            ))
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    pub fn run<H: IpcHost + 'static>(self, host: H) {
        let mut main_loop = MainLoop::new(&self.window);

        self.event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                    }

                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        main_loop.window_resized(width, height);
                    }

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => {
                        main_loop.keyboard_input(key, state);
                    }

                    WindowEvent::MouseInput { button, state, .. } => {
                        main_loop.mouse_input(button, state);
                    }

                    WindowEvent::CursorMoved {
                        position: PhysicalPosition { x, y },
                        ..
                    } => {
                        main_loop.mouse_movement(vec2(x as f32, y as f32));
                    }

                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(_, y) => main_loop.mouse_wheel_movement(y),
                        MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                            main_loop.mouse_wheel_movement(y.signum() as f32)
                        }
                    },

                    _ => {}
                },

                Event::MainEventsCleared => {
                    main_loop.process(&host);
                    main_loop.render();
                }

                _ => {}
            }
        });
    }
}
