use std::{collections::VecDeque, sync::Mutex};
use wasm_bindgen::prelude::*;

static mut FROM_JS: Option<Mutex<VecDeque<Message>>> = None;
static mut TO_JS: Option<Mutex<VecDeque<String>>> = None;
static mut SAVED_SCENE: Option<Mutex<Option<Vec<u8>>>> = None;

pub fn init() {
    unsafe {
        FROM_JS = Some(Default::default());
        TO_JS = Some(Default::default());
        SAVED_SCENE = Some(Default::default());
    }
    // Initialization done, make it known to the outside world
    send_packet(r#"{ "message": "init" }"#.to_owned());
}

pub fn send_packet(json: String) {
    let mut deque = unsafe { TO_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.push_back(json);
}

pub fn query_packet() -> Option<Message> {
    let mut deque = unsafe { FROM_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.pop_front()
}

pub fn set_saved_scene(data: Vec<u8>) {
    let mut result = unsafe { SAVED_SCENE.as_mut().unwrap().try_lock().unwrap() };
    *result = Some(data);
}

fn push_from_js(message: Message) {
    let mut deque = unsafe {
        FROM_JS
            .as_mut()
            .expect("Sent packet while uninitialized!")
            .try_lock()
            .unwrap()
    };
    deque.push_back(message);
}

#[wasm_bindgen(js_name = "queryMessage")]
pub fn __query_message() -> Option<String> {
    unsafe {
        if let Some(deque) = TO_JS.as_mut() {
            let mut deque = deque.try_lock().unwrap();
            deque.pop_front()
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
    push_from_js(Message::TextureData { id, data })
}

#[wasm_bindgen(js_name = "loadTextures")]
pub fn __load_textures() {
    push_from_js(Message::LoadTextures)
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

#[wasm_bindgen(js_name = "saveScene")]
pub fn __save_scene() {
    push_from_js(Message::SaveScene);
}

#[wasm_bindgen(js_name = "getSavedScene")]
pub fn __get_saved_scene() -> Option<Vec<u8>> {
    let mut result = unsafe { SAVED_SCENE.as_mut().unwrap().try_lock().unwrap() };
    result.take()
}

pub enum Message {
    SetResolution { width: u32, height: u32 },
    TextureData { id: u32, data: Vec<u8> },
    LoadTextures,
    SetEditorMode(i32),
    SetSolidEditorMode(i32),
    SetGizmo(i32),
    SelectTexture(u32),
    SelectProp(u32),
    SaveScene,
}
