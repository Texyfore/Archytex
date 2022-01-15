pub mod ipc;

mod winit_loop;
mod main_loop;

use self::{ipc::IpcHost, winit_loop::WinitLoop};

pub fn main<H: IpcHost + 'static>(host: H) {
    let winit_loop = match WinitLoop::new() {
        Ok(ok) => ok,
        Err(err) => {
            host.error(format!("{}", err));
            return;
        },
    };

    winit_loop.run(host);
}