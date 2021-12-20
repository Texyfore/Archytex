use std::{collections::VecDeque, sync::Mutex};
use wasm_bindgen::prelude::*;

static mut FROM_JS: Option<Mutex<VecDeque<Message>>> = None;
static mut TO_JS: Option<Mutex<VecDeque<String>>> = None;

pub fn init() {
    unsafe {
        FROM_JS = Some(Default::default());
        TO_JS = Some(Default::default());
    }
}

pub fn send_packet(json: String) {
    let mut deque = unsafe { TO_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.push_back(json);
}

pub fn query_packet() -> Option<Message> {
    let mut deque = unsafe { FROM_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.pop_front()
}

pub fn push_from_js(message: Message) {
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

#[wasm_bindgen(js_name = "sendTextureData")]
pub fn __send_texture_data(id: u32, data: Vec<u8>) {
    push_from_js(Message::TextureData { id, data })
}

#[wasm_bindgen(js_name = "finishTexture")]
pub fn __finish_texture(id: u32) {
    push_from_js(Message::FinishTexture { id })
}

pub enum Message {
    SetResolution { width: u32, height: u32 },
    TextureData { id: u32, data: Vec<u8> },
    FinishTexture { id: u32 },
}
