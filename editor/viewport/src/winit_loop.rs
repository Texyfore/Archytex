use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{ipc::IpcHost, main_loop::MainLoop};

macro_rules! check {
    ($h:ident, $f:ident, $e:expr) => {
        match $e {
            Ok(_) => {}
            Err(err) => {
                $h.error(format!(
                    "Error: {}\n\nCaused by:\n    {}",
                    err,
                    err.root_cause()
                ));
                *$f = ControlFlow::Exit;
            }
        }
    };
}

pub struct WinitLoop {
    event_loop: EventLoop<()>,
    window: Window,
}

impl WinitLoop {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self> {
        use anyhow::Context;

        let event_loop = EventLoop::new();
        let window = WindowBuilder::default()
            .build(&event_loop)
            .context("couldn't create window")?;

        Ok(Self { event_loop, window })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Result<Self> {
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
            .context("couldn't create window")?;

        Ok(Self { event_loop, window })
    }

    pub fn run<H: IpcHost + 'static>(self, host: H) {
        let mut main_loop = match MainLoop::new(&self.window) {
            Ok(ok) => ok,
            Err(err) => {
                host.error(format!(
                    "Error: {}\n\nCaused by:\n    {}",
                    err,
                    err.root_cause()
                ));
                return;
            }
        };

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

                    _ => {}
                },

                Event::MainEventsCleared => {
                    check!(host, flow, main_loop.process(&host));
                    check!(host, flow, main_loop.render());
                }

                _ => {}
            }
        });
    }
}
