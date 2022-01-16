pub mod ipc;

mod input;
mod main_loop;
mod winit_loop;

use self::{ipc::IpcHost, winit_loop::WinitLoop};

pub fn main<H: IpcHost + 'static>(host: H) {
    let winit_loop = match WinitLoop::new() {
        Ok(ok) => ok,
        Err(err) => {
            host.error(format!(
                "Error: {}\n\nCaused by:\n    {}",
                err,
                err.root_cause()
            ));
            return;
        }
    };

    winit_loop.run(host);
}
