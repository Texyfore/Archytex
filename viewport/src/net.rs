use std::{collections::VecDeque, sync::Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

static mut FROM_JS: Option<Mutex<VecDeque<Vec<u8>>>> = None;
static mut TO_JS: Option<Mutex<VecDeque<Vec<u8>>>> = None;

pub fn init() {
    unsafe {
        FROM_JS = Some(Default::default());
        TO_JS = Some(Default::default());
    }
}

pub fn send_packet(packet: Vec<u8>) {
    let mut deque = unsafe { TO_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.push_back(packet);
}

pub fn query_packet() -> Option<Vec<u8>> {
    let mut deque = unsafe { FROM_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.pop_front()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "sendPacket")]
pub fn __send_packet(packet: Vec<u8>) {
    let mut deque = unsafe { FROM_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.push_back(packet);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "queryPacket")]
pub fn __query_packet() -> Option<Vec<u8>> {
    let mut deque = unsafe { TO_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.pop_front()
}
