mod graphics;
mod logic;

use logic::Logic;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use self::graphics::{Canvas, Renderer};

pub fn run(init: Init) {
    let winit = init.winit;
    let save_handler = init.save_handler;

    let mut renderer = Renderer::new(&winit.window);
    let mut logic = Logic::init(logic::Context {
        renderer: &renderer,
        save_handler: save_handler.as_ref(),
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
                logic.process(logic::Context {
                    renderer: &renderer,
                    save_handler: save_handler.as_ref(),
                });

                let mut canvas = Canvas;
                logic.render(&mut canvas);
                renderer.render(canvas);
            }
            _ => (),
        }
    });
}

pub struct Init {
    pub winit: Winit,
    pub save_handler: Box<dyn OnSave>,
    pub resources: Vec<Resource>,
}

pub struct Winit {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

pub trait OnSave {
    fn on_save(&self, buf: &[u8]);
}

pub struct Resource {
    pub bytes: Vec<u8>,
    pub kind: ResourceKind,
}

pub enum ResourceKind {
    Texture,
    Prop,
    Gizmo,
}
