use std::collections::HashMap;

use cgmath::Vector2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

pub trait Input {
    fn define_actions(&mut self, actions: &[(&str, Trigger)]);
    fn is_active(&self, action: &str) -> bool;
    fn is_active_once(&self, action: &str) -> bool;
    fn mouse_delta(&self) -> Vector2<f32>;
    fn scroll_wheel(&self) -> f32;
    fn mouse_pos(&self) -> Vector2<f32>;
}

#[derive(Default)]
pub struct InputMapper {
    actions: HashMap<String, Action>,
    mouse_pos_before: [f32; 2],
    mouse_pos: [f32; 2],
    scroll_wheel: f32,
}

impl InputMapper {
    pub fn set_trigger(&mut self, trigger: Trigger, state: ElementState) {
        for (_, mut action) in &mut self.actions {
            if action.trigger == trigger {
                match state {
                    ElementState::Pressed => {
                        action.active = true;
                        action.active_once = true;
                    }
                    ElementState::Released => {
                        action.active = false;
                        action.active_once = false;
                    }
                }
            }
        }
    }

    pub fn set_mouse_pos(&mut self, pos: [f32; 2]) {
        self.mouse_pos = pos;
    }

    pub fn set_scroll_wheel(&mut self, wheel: f32) {
        self.scroll_wheel = wheel;
    }

    pub fn tick(&mut self) {
        for (_, mut action) in &mut self.actions {
            action.active_once = false;
        }
        self.mouse_pos_before = self.mouse_pos;
        self.scroll_wheel = 0.0;
    }
}

impl Input for InputMapper {
    fn define_actions(&mut self, actions: &[(&str, Trigger)]) {
        self.actions = actions
            .iter()
            .map(|(name, trigger)| {
                (
                    name.to_string(),
                    Action {
                        trigger: trigger.clone(),
                        active: false,
                        active_once: false,
                    },
                )
            })
            .collect();
    }

    fn is_active(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            action.active
        } else {
            false
        }
    }

    fn is_active_once(&self, action: &str) -> bool {
        if let Some(action) = self.actions.get(action) {
            action.active_once
        } else {
            false
        }
    }

    fn mouse_delta(&self) -> Vector2<f32> {
        let a: Vector2<_> = self.mouse_pos.into();
        let b: Vector2<_> = self.mouse_pos_before.into();
        b - a
    }

    fn scroll_wheel(&self) -> f32 {
        self.scroll_wheel
    }

    fn mouse_pos(&self) -> Vector2<f32> {
        self.mouse_pos.into()
    }
}

#[derive(PartialEq, Clone)]
pub enum Trigger {
    Key(VirtualKeyCode),
    Button(MouseButton),
}

struct Action {
    trigger: Trigger,
    active: bool,
    active_once: bool,
}
