mod inner_logic;
mod input;

use crate::input::ElementKind;
use inner_logic::InnerLogic;
use input::InputMapper;
use tools::app::{
    event::Event,
    input::{ButtonKind, KeyKind},
    App, MainLoop,
};

pub struct Viewport {
    input_mapper: InputMapper,
    inner_logic: Option<InnerLogic>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            input_mapper: Default::default(),
            inner_logic: None,
        }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {
                    self.input_mapper
                        .register_action("forward", vec![ElementKind::Key(KeyKind::W)]);
                    self.input_mapper
                        .register_action("backward", vec![ElementKind::Key(KeyKind::S)]);
                    self.input_mapper
                        .register_action("left", vec![ElementKind::Key(KeyKind::A)]);
                    self.input_mapper
                        .register_action("right", vec![ElementKind::Key(KeyKind::D)]);
                    self.input_mapper
                        .register_action("up", vec![ElementKind::Key(KeyKind::E)]);
                    self.input_mapper
                        .register_action("down", vec![ElementKind::Key(KeyKind::Q)]);
                    self.input_mapper
                        .register_action("look", vec![ElementKind::Button(ButtonKind::Right)]);

                    self.inner_logic = Some(InnerLogic::new(app.graphics()));
                }
                Event::Resized(width, height) => {
                    if let Some(logic) = &mut self.inner_logic {
                        logic.resized(width, height);
                    }
                }
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        if let Some(logic) = &mut self.inner_logic {
            logic.process(&self.input_mapper, app);
        }

        self.input_mapper.clear();
    }
}
