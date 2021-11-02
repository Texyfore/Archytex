pub mod event;
pub mod input;

use crate::{gfx::Graphics, web_util};
use event::{Event, RawInputKind};
use std::collections::VecDeque;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Size},
    event::{Event as WinitEvent, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowBuilderExtWebSys,
    window::{Window, WindowBuilder},
};

const WINDOW_SIZE: (u32, u32) = (640, 480);

pub struct App {
    window: Option<Window>,
    graphics: Graphics,
    event_queue: VecDeque<Event>,
    cursor_visible: bool,
}

impl Default for App {
    fn default() -> Self {
        let graphics = Graphics::default();
        let event_queue = VecDeque::new();

        Self {
            window: None,
            graphics,
            event_queue,
            cursor_visible: true,
        }
    }
}

impl App {
    pub fn run<M: MainLoop>(mut self, mut main_loop: M) {
        console_error_panic_hook::set_once();

        let event_loop = EventLoop::new();

        self.window = Some(
            WindowBuilder::new()
                .with_canvas(Some(web_util::get_canvas()))
                .with_inner_size(Size::Physical(PhysicalSize::new(
                    WINDOW_SIZE.0,
                    WINDOW_SIZE.1,
                )))
                .build(&event_loop)
                .expect("Failed to create window"),
        );

        self.event_queue.push_back(Event::Initialized);

        self.graphics
            .resize_viewport(WINDOW_SIZE.0 as i32, WINDOW_SIZE.1 as i32);
        self.event_queue
            .push_back(Event::Resized(WINDOW_SIZE.0, WINDOW_SIZE.1));

        event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::Poll;

            match event {
                WinitEvent::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        self.graphics.resize_viewport(width as i32, height as i32);
                        self.event_queue.push_back(Event::Resized(width, height));
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
                        self.event_queue
                            .push_back(Event::RawInput(RawInputKind::Key(
                                state.into(),
                                key.into(),
                            )));
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        self.event_queue
                            .push_back(Event::RawInput(RawInputKind::Button(
                                state.into(),
                                button.into(),
                            )))
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        self.event_queue
                            .push_back(Event::RawInput(RawInputKind::Movement(
                                position.x as f32,
                                position.y as f32,
                            )))
                    }

                    WindowEvent::MouseWheel { delta, .. } => {
                        self.event_queue
                            .push_back(Event::RawInput(RawInputKind::Wheel(match delta {
                                MouseScrollDelta::LineDelta(y, ..) => y.signum(),
                                MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                                    y.signum() as f32
                                }
                            })));
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

    pub fn graphics(&self) -> &Graphics {
        &self.graphics
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        if self.cursor_visible != visible {
            if let Some(window) = &self.window {
                window.set_cursor_visible(visible);
            }
        }
        self.cursor_visible = visible;
    }
}

pub trait MainLoop: 'static {
    fn process(&mut self, app: &mut App);
}
