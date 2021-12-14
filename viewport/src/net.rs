use wasm_bindgen::prelude::*;
use std::{collections::VecDeque, sync::Mutex};

use crate::info;

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
    info!("Sent packet len: {}", packet.len());
    deque.push_back(packet);
}

pub fn query_packet() -> Option<Vec<u8>> {
    let mut deque = unsafe { FROM_JS.as_mut().unwrap().try_lock().unwrap() };
    deque.pop_front()
}

#[wasm_bindgen(js_name = "sendPacket")]
pub fn __send_packet(packet: Vec<u8>) {
    let mut deque = unsafe {
        FROM_JS
            .as_mut()
            .expect("Sent packet while uninitialized!")
            .try_lock()
            .unwrap()
    };
    deque.push_back(packet);
}

#[wasm_bindgen(js_name = "queryPacket")]
pub fn __query_packet() -> Option<Vec<u8>> {
    unsafe {
        if let Some(deque) = TO_JS.as_mut() {
            let mut deque = deque.try_lock().unwrap();
            let p = deque.pop_front();
            info!("Received packet: {:?}", p);
            p
        } else {
            None
        }
    }
}
