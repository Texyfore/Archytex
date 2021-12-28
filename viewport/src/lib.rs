mod editor;
mod input;
mod log;
mod math;
mod net;
mod render;
mod ring_vec;

use crate::{editor::EditMode, render::WorldPass};
use cgmath::{Matrix4, SquareMatrix};
use instant::Instant;
use render::{Scene, SceneRenderer, SpritePass, TextureBank};
use wasm_bindgen::{prelude::*, JsCast};
use winit::platform::web::WindowBuilderExtWebSys;
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
    net::Message,
};

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();

    net::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::default()
        .with_canvas(Some(
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("viewport-canvas")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
        .build(&event_loop)
        .unwrap();

    let mut main_loop = {
        let (width, height) = window.inner_size().into();
        let mut main_loop = MainLoop::init(window);
        main_loop.window_resized(width, height);
        main_loop
    };

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

struct MainLoop {
    window: Window,
    before: Instant,
    renderer: SceneRenderer,
    texture_bank: TextureBank,
    input_mapper: InputMapper,
    editor: Editor,
}

impl MainLoop {
    fn init(window: Window) -> Self {
        let gfx_init = render::init(&window);
        let renderer = gfx_init.create_scene_renderer();
        let texture_bank = gfx_init.create_texture_bank();
        let solid_factory = gfx_init.create_solid_factory();
        let line_factory = gfx_init.create_line_factory();

        let mut input_mapper = InputMapper::default();
        let editor = Editor::init(solid_factory, line_factory, &mut input_mapper);

        Self {
            window,
            before: Instant::now(),
            renderer,
            texture_bank,
            input_mapper,
            editor,
        }
    }

    fn window_resized(&mut self, width: u32, height: u32) {
        self.renderer.resize_viewport(width, height);
        self.editor.window_resized(width, height);
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
        let after = Instant::now();
        let elapsed = (after - self.before).as_secs_f32();
        self.before = after;

        while let Some(message) = net::query_packet() {
            match message {
                Message::SetResolution { width, height } => {
                    self.window.set_inner_size(PhysicalSize { width, height });
                    info!("Resolution changed to [{}x{}]", width, height);
                }
                Message::TextureData { id, data } => {
                    self.texture_bank.insert_data(id, data);
                    info!("Uploaded texture {}", id);
                }
                Message::LoadTextures => {
                    self.texture_bank.finish();
                    info!("All textures loaded");
                }
                Message::SetEditorMode(mode) => {
                    match mode {
                        0 => self.editor.mode = EditMode::Solid,
                        1 => self.editor.mode = EditMode::Prop,
                        _ => {}
                    }
                    info!("Editor mode changed to: {:?}", mode);
                }
                Message::SetSolidEditorMode(mode) => {
                    self.editor.set_solid_editor_mode(mode);
                }
                Message::SetGizmo(gizmo) => {
                    info!("Gizmo will be set to: {}", gizmo);
                }
                Message::SelectTexture(texture) => {
                    info!("A texture was selected: {}", texture);
                }
                Message::SelectProp(prop) => {
                    info!("A prop was selected: {}", prop);
                }
                Message::SaveScene => {
                    self.editor.save_scene(&self.texture_bank);
                    info!("Scene saved");
                }
            }
        }

        let mut scene = Scene {
            texture_bank: &self.texture_bank,
            world_pass: WorldPass {
                camera_matrix: Matrix4::identity(),
                solid_batches: Default::default(),
                line_batches: Default::default(),
            },
            sprite_pass: SpritePass {
                camera_matrix: Matrix4::identity(),
                sprites: Default::default(),
            },
        };

        self.editor
            .process(elapsed, &self.input_mapper, &self.texture_bank);

        self.input_mapper.tick();
        self.editor.render(&mut scene);
        self.renderer.render(scene);
    }
}
