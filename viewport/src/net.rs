use concurrent_queue::ConcurrentQueue;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static mut FROM_JS: Option<ConcurrentQueue<Message>> = None;
static mut TO_JS: Option<ConcurrentQueue<String>> = None;
static mut SAVED_SCENE: Option<Mutex<Option<Vec<u8>>>> = None;

pub fn init() {
    unsafe {
        FROM_JS = Some(ConcurrentQueue::bounded(16));
        TO_JS = Some(ConcurrentQueue::bounded(16));
        SAVED_SCENE = Some(Default::default());
    }
    // Initialization done, make it known to the outside world
    send_packet(r#"{ "message": "init" }"#.to_owned());
}

pub fn send_packet(json: String) {
    let deque = unsafe { TO_JS.as_mut().unwrap() };
    deque.push(json).ok();
}

pub fn query_packet() -> Option<Message> {
    let deque = unsafe { FROM_JS.as_mut().unwrap() };
    deque.pop().ok()
}

pub fn set_saved_scene(data: Vec<u8>) {
    let mut result = unsafe { SAVED_SCENE.as_mut().unwrap().try_lock().unwrap() };
    *result = Some(data);
}

fn push_from_js(message: Message) {
    let deque = unsafe { FROM_JS.as_mut().expect("Sent packet while uninitialized!") };
    deque.push(message).ok();
}

#[wasm_bindgen(js_name = "queryMessage")]
pub fn __query_message() -> Option<String> {
    unsafe {
        if let Some(deque) = TO_JS.as_mut() {
            deque.pop().ok()
        } else {
            None
        }
    }
}

#[wasm_bindgen(js_name = "setResolution")]
pub fn __set_resolution(width: u32, height: u32) {
    push_from_js(Message::SetResolution { width, height });
}

#[wasm_bindgen(js_name = "textureData")]
pub fn __texture_data(id: u32, data: Vec<u8>) {
    push_from_js(Message::TextureData { id, data });
}

#[wasm_bindgen(js_name = "loadTextures")]
pub fn __load_textures() {
    push_from_js(Message::LoadTextures);
}

#[wasm_bindgen(js_name = "propData")]
pub fn __prop_data(id: u32, data: Vec<u8>) {
    push_from_js(Message::PropData { id, data });
}

#[wasm_bindgen(js_name = "loadProps")]
pub fn __load_props() {
    push_from_js(Message::LoadProps);
}

#[wasm_bindgen(js_name = "setEditorMode")]
pub fn __set_editor_mode(mode: i32) {
    push_from_js(Message::SetEditorMode(mode));
}

#[wasm_bindgen(js_name = "setSolidEditorMode")]
pub fn __set_solid_editor_mode(mode: i32) {
    push_from_js(Message::SetSolidEditorMode(mode));
}

#[wasm_bindgen(js_name = "setGizmo")]
pub fn __set_gizmo(gizmo: i32) {
    push_from_js(Message::SetGizmo(gizmo));
}

#[wasm_bindgen(js_name = "selectTexture")]
pub fn __select_texture(texture: u32) {
    push_from_js(Message::SelectTexture(texture));
}

#[wasm_bindgen(js_name = "selectProp")]
pub fn __select_prop(prop: u32) {
    push_from_js(Message::SelectProp(prop));
}

#[wasm_bindgen(js_name = "setCameraSpeed")]
pub fn __set_camera_speed(speed: f32) {
    push_from_js(Message::SetCameraSpeed(speed));
}

#[wasm_bindgen(js_name = "saveScene")]
pub fn __save_scene() {
    push_from_js(Message::SaveScene);
}

#[wasm_bindgen(js_name = "getSavedScene")]
pub fn __get_saved_scene() -> Option<Vec<u8>> {
    let mut result = unsafe { SAVED_SCENE.as_mut().unwrap().try_lock().unwrap() };
    result.take()
}

#[wasm_bindgen(js_name = "loadScene")]
pub fn __load_scene(scene: Vec<u8>) {
    if let Some(scene) = mdl::Scene::decode(&scene) {
        push_from_js(Message::LoadScene(scene));
    }
}

#[wasm_bindgen(js_name = "setGridSize")]
pub fn __set_grid_size(size: i32) {
    push_from_js(Message::SetGridSize(size));
}

pub enum Message {
    SetResolution { width: u32, height: u32 },
    TextureData { id: u32, data: Vec<u8> },
    PropData { id: u32, data: Vec<u8> },
    LoadTextures,
    LoadProps,
    SetEditorMode(i32),
    SetSolidEditorMode(i32),
    SetGizmo(i32),
    SelectTexture(u32),
    SelectProp(u32),
    SetCameraSpeed(f32),
    SaveScene,
    LoadScene(mdl::Scene),
    SetGridSize(i32),
}
