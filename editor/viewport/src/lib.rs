pub mod ipc;

mod editor;
mod input;
mod main_loop;
mod math;
mod winit_loop;

use self::{ipc::IpcHost, winit_loop::WinitLoop};

pub fn main<H: IpcHost + 'static>(host: H) {
    let winit_loop = WinitLoop::new();
    winit_loop.run(host);
}
