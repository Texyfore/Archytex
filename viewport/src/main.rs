mod log;
mod render;

use render::Renderer;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::default().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    insert_canvas(&window);

    let renderer = Renderer::new(&window);

    {
        let (width, height) = window.inner_size().into();
        renderer.resize(width, height);
    }

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    renderer.resize(width, height);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                renderer.render();
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
