pub mod event;

use crate::gfx::Graphics;
use event::Event;
use std::collections::VecDeque;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::{
    dpi::{PhysicalSize, Size},
    event::{Event as WinitEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowBuilderExtWebSys,
    window::WindowBuilder,
};

pub struct App {
    graphics: Graphics,
    event_queue: VecDeque<Event>,
}

impl Default for App {
    fn default() -> Self {
        let graphics = Graphics::new(&get_canvas());
        let event_queue = VecDeque::new();

        Self {
            graphics,
            event_queue,
        }
    }
}

impl App {
    pub fn run<M: MainLoop>(mut self, mut main_loop: M) {
        console_error_panic_hook::set_once();

        let event_loop = EventLoop::new();

        WindowBuilder::new()
            .with_canvas(Some(get_canvas()))
            .with_inner_size(Size::Physical(PhysicalSize::new(640, 480)))
            .build(&event_loop)
            .expect("Failed to create window");

        self.event_queue.push_back(Event::Initialized);

        event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Wait;

            match event {
                WinitEvent::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        self.graphics.resize_viewport(width as i32, height as i32);
                    }
                    _ => {}
                },
                WinitEvent::MainEventsCleared => {
                    self.graphics.begin();
                    main_loop.process(&mut self);
                }
                _ => {}
            }
        });
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }
}

pub trait MainLoop: 'static {
    fn process(&mut self, app: &mut App);
}

fn get_canvas() -> HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    document
        .get_element_by_id("viewport")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}
