#![allow(dead_code)]

pub const POINT_SELECT_RADIUS: f32 = 50.0;
pub const NEW_BRUSH_MIN_SCREEN_DISTANCE: f32 = 10.0;

pub const FACE_HIGHLIGHT_COLOR: [f32; 4] = [0.2, 0.8, 1.0, 1.0];
pub const VERTEX_HIGHLIGHT_COLOR: [f32; 4] = [0.04, 0.36, 0.85, 1.0];

pub const MAX_POINTS: usize = 16384;
pub const MAX_FACES: usize = 16384;
pub const MAX_SOLIDS: usize = 16384;

pub const GRID_MIN: i32 = -3;
pub const GRID_MAX: i32 = 2;