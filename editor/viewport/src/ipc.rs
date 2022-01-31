use asset_id::{GizmoID, PropID, TextureID};

pub trait IpcHost {
    fn recv(&self) -> Option<IpcMessage>;
    fn send_editor_mode(&self, mode: i32);
    fn send_camera_speed(&self, speed: i32);
    fn send_grid_step(&self, step: i32);
}

pub enum IpcMessage {
    Resources {
        textures: Vec<(TextureID, Vec<u8>)>,
        props: Vec<(PropID, Vec<u8>)>,
        gizmos: Vec<(GizmoID, Vec<u8>)>,
    },
    Resolution {
        width: u32,
        height: u32,
    },
    EditorMode(EditorMode),
    GridStep(i32),
    CameraSpeed(i32),
    CurrentTexture(TextureID),
    CurrentProp(PropID),
    RequestCameraSpeed,
    RequestGridStep,
}

pub enum EditorMode {
    Solid,
    Face,
    Point,
    Prop,
}
