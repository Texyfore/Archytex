use anyhow::Result;
use renderer::Renderer;
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{editor::Editor, input::Input, ipc::IpcHost};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
    editor: Editor,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self> {
        let mut renderer = Renderer::new(window)?;
        let input = Input::default();
        let editor = Editor::new(&renderer);

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
        }

        Ok(Self {
            renderer,
            input,
            editor,
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        self.input.process();
        self.editor.process()?;
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        self.editor.render(&self.renderer)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.editor.window_resized(width, height);
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.keyboard_input(key, state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input.mouse_input(button, state);
    }
}
