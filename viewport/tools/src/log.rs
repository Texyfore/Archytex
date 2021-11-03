use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn warm(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(msg: &str);
}


#[macro_export]
macro_rules! console {
    ($($t:tt)*) => {
        $crate::log::log(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => {
        $crate::log::warn(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => {
        $crate::log::error(&format_args!($($t)*).to_string())
    };
}
