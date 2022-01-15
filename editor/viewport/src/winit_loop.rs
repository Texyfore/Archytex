use anyhow::Result;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct MainLoop {
    event_loop: EventLoop<()>,
    _window: Window,
}

impl MainLoop {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::default().build(&event_loop)?;
        Ok(Self {
            event_loop,
            _window,
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Result<Self> {
        use wasm_bindgen::JsCast;
        use web_sys::HtmlCanvasElement;
        use winit::platform::web::WindowBuilderExtWebSys;

        let event_loop = EventLoop::new();
        let _window = WindowBuilder::default()
            .with_canvas(Some(
                web_sys::window()
                    .ok_or(HtmlElementNotFound)?
                    .document()
                    .ok_or(HtmlElementNotFound)?
                    .get_element_by_id("viewport-canvas")
                    .ok_or(HtmlElementNotFound)?
                    .dyn_into::<HtmlCanvasElement>()
                    .map_err(|_| HtmlElementNotFound)?,
            ))
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            _window,
        })
    }

    pub fn run(self) -> Result<()> {
        self.event_loop.run(|event, _, flow| {
            *flow = ControlFlow::Poll;

            #[allow(clippy::collapsible_match, clippy::single_match)]
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {}
                _ => {}
            }
        });
    }
}

#[cfg(target_arch="wasm32")]
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
#[derive(Error, Debug)]
#[error("Couldn't find a necessary HTML element")]
pub struct HtmlElementNotFound;
