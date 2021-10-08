use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
}

#[macro_export]
macro_rules! console {
    ($($t:tt)*) => {
        $crate::log::log(&format_args!($($t)*).to_string())
    };
}