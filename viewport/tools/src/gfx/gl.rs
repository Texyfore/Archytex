use super::Color;
use crate::web_util;
use glow::*;

pub struct WebGL {
    ctx: Context,
}

impl Default for WebGL {
    fn default() -> Self {
        Self {
            ctx: Context::from_webgl2_context(web_util::get_webgl_context()),
        }
    }
}

impl WebGL {
    pub fn enable_depth_test(&self) {
        unsafe { self.ctx.enable(DEPTH_TEST) };
    }

    pub fn set_clear_color(&self, color: Color) {
        unsafe { self.ctx.clear_color(color.r, color.g, color.b, color.a) };
    }

    pub fn clear(&self) {
        unsafe { self.ctx.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT) };
    }

    pub fn set_viewport_size(&self, width: i32, height: i32) {
        unsafe { self.ctx.viewport(0, 0, width, height) };
    }
}
