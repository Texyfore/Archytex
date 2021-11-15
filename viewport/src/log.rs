use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(string: &str);

    #[wasm_bindgen(js_namespace=console)]
    pub fn warn(string: &str);

    #[wasm_bindgen(js_namespace=console)]
    pub fn error(string: &str);
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $(,$arg:expr)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::log::log(&format!($fmt $(,$arg)*));
        #[cfg(not(target_arch="wasm32"))]
        println!($fmt $(,$arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $(,$arg:expr)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::log::warn(&format!($fmt $(,$arg)*));
        #[cfg(not(target_arch="wasm32"))]
        println!($fmt $(,$arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $(,$arg:expr)*) => {
        #[cfg(target_arch="wasm32")]
        $crate::log::error(&format!($fmt $(,$arg)*));
        #[cfg(not(target_arch="wasm32"))]
        println!($fmt $(,$arg)*);
    };
}
