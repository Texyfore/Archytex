mod button;
mod data;
mod graphics;
mod logic;
mod math;

use std::sync::mpsc::Receiver;

use asset::{scene::Scene, Gizmo, GizmoID, Prop, PropID, Texture, TextureID};
use data::PropInfoContainer;
use instant::Instant;
use logic::{ElementKind, Logic};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use self::graphics::Canvas;

pub fn run(init: Init) {
    let window = init.winit.window;
    let event_loop = init.winit.event_loop;
    let host = init.host;
    let from_host = init.receiver;

    let (mut renderer, graphics) = graphics::init(&window);
    let mut prop_info = PropInfoContainer::default();

    {
        let resources = init.resources;
        for resource in resources {
            match resource.kind {
                ResourceKind::Texture => {
                    let id = TextureID(resource.id);
                    let texture = Texture::new(&resource.buf);
                    renderer.add_texture(id, texture);
                }
                ResourceKind::Prop => {
                    let id = PropID(resource.id);
                    let prop = Prop::decode(&resource.buf).unwrap();
                    prop_info.insert(id, &prop);
                    renderer.add_prop(id, prop);
                }
                ResourceKind::Gizmo => {
                    let id = GizmoID(resource.id);
                    let gizmo = Gizmo::decode(&resource.buf).unwrap();
                    renderer.add_gizmo(id, gizmo);
                }
            }
        }
    }

    let mut logic = Logic::init(logic::Context {
        host: host.as_ref(),
        graphics: &graphics,
        prop_infos: &prop_info,
        delta: 0.0,
    });

    {
        let (width, height) = window.inner_size().into();
        renderer.resize(width, height);
        logic.resized(width, height);
    }

    let mut before = Instant::now();
    let mut lock_pointer = false;

    event_loop.run(move |event, _, flow| {
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
                    if !lock_pointer {
                        logic.movement(x as f32, y as f32);
                    }
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
                let after = Instant::now();
                let delta = (after - before).as_secs_f32();
                before = after;

                while let Ok(signal) = from_host.try_recv() {
                    match signal {
                        FromHost::Resolution { width, height } => {
                            window.set_inner_size(PhysicalSize { width, height })
                        }
                        FromHost::SaveScene(id) => {
                            logic.save_scene(
                                logic::Context {
                                    host: host.as_ref(),
                                    graphics: &graphics,
                                    prop_infos: &prop_info,
                                    delta,
                                },
                                id,
                            );
                        }
                        FromHost::LoadScene(buf) => {
                            let scene = Scene::decode(&buf).unwrap();
                            logic.load_scene(
                                logic::Context {
                                    host: host.as_ref(),
                                    graphics: &graphics,
                                    prop_infos: &prop_info,
                                    delta,
                                },
                                &scene,
                            );
                        }
                        FromHost::Texture(id) => {
                            logic.set_texture(TextureID(id));
                        }
                        FromHost::Prop(id) => {
                            logic.set_prop(PropID(id));
                        }
                        FromHost::Button(button) => match button {
                            button::PROP => {
                                logic.set_editor_mode(
                                    logic::Context {
                                        host: host.as_ref(),
                                        graphics: &graphics,
                                        prop_infos: &prop_info,
                                        delta,
                                    },
                                    ElementKind::Prop,
                                );
                            }
                            button::SOLID => {
                                logic.set_editor_mode(
                                    logic::Context {
                                        host: host.as_ref(),
                                        graphics: &graphics,
                                        prop_infos: &prop_info,
                                        delta,
                                    },
                                    ElementKind::Solid,
                                );
                            }
                            button::FACE => {
                                logic.set_editor_mode(
                                    logic::Context {
                                        host: host.as_ref(),
                                        graphics: &graphics,
                                        prop_infos: &prop_info,
                                        delta,
                                    },
                                    ElementKind::Face,
                                );
                            }
                            button::POINT => {
                                logic.set_editor_mode(
                                    logic::Context {
                                        host: host.as_ref(),
                                        graphics: &graphics,
                                        prop_infos: &prop_info,
                                        delta,
                                    },
                                    ElementKind::Point,
                                );
                            }
                            button::MOVE => todo!(),
                            button::ROTATE => todo!(),
                            _ => (),
                        },
                        FromHost::Movement(x, y) => {
                            logic.movement_override(x, y);
                        }
                        FromHost::LockPointer(lock) => {
                            lock_pointer = lock;
                        }
                    }
                }

                logic.process(logic::Context {
                    host: host.as_ref(),
                    graphics: &graphics,
                    prop_infos: &prop_info,
                    delta,
                });

                let mut canvas = Canvas::default();
                logic.render(&mut canvas);
                renderer.render(canvas);
            }
            _ => (),
        }
    });
}

pub struct Init {
    pub winit: Winit,
    pub resources: Vec<Resource>,
    pub host: Box<dyn Host>,
    pub receiver: Receiver<FromHost>,
}

pub struct Winit {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

pub struct Resource {
    pub id: u32,
    pub buf: Vec<u8>,
    pub kind: ResourceKind,
}

pub enum ResourceKind {
    Texture,
    Prop,
    Gizmo,
}

pub trait Host {
    fn callback(&self, data: ToHost);
}

pub enum ToHost {
    SceneSaved(i32, Vec<u8>),
    Button(i32),
}

pub enum FromHost {
    Resolution { width: u32, height: u32 },
    SaveScene(i32),
    LoadScene(Vec<u8>),
    Prop(u32),
    Texture(u32),
    Button(i32),
    Movement(f32, f32),
    LockPointer(bool),
}
