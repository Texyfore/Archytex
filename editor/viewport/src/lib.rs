pub mod ipc;

mod winit_loop;

use anyhow::Result;

use self::{ipc::IpcHost, winit_loop::MainLoop};

pub fn main<H: IpcHost>(host: H) {
    match run() {
        Ok(_) => unreachable!(),
        Err(err) => host.fatal_error(format!("{}", err)),
    }
}

fn run() -> Result<()> {
    let main_loop = MainLoop::new()?;
    main_loop.run()?;
    unreachable!()
}
