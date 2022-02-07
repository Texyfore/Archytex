mod graphics;
mod logic;

use logic::Logic;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use self::{
    graphics::{Canvas, Renderer},
    logic::Context,
};

pub fn run(winit: Winit, _callbacks: Callbacks) {
    let mut renderer = Renderer::new(&winit.window);
    let mut logic = Logic::init(Context {
        renderer: &renderer,
    });

    winit.event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    renderer.resize(width, height);
                    logic.resized(width, height);
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
                    logic.key(code, state);
                }
                WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y },
                    ..
                } => {
                    logic.movement(x as f32, y as f32);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let delta = match delta {
                        MouseScrollDelta::LineDelta(_, delta) => delta,
                        MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y as f32,
                    };
                    logic.scroll(delta);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    logic.button(button, state);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                logic.process(Context {
                    renderer: &renderer,
                });

                let mut canvas = Canvas;
                logic.render(&mut canvas);
                renderer.render(canvas);
            }
            _ => (),
        }
    });
}

pub struct Winit {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

pub struct Callbacks {
    pub save: Box<dyn Fn(&[u8])>,
}
