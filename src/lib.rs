use crate::interface::Interface;
use log::info;
use wasm_bindgen::prelude::*;

mod audio_to_coords;
mod backend;
mod coords_to_audio;
mod get_requests;
mod image_to_coords;
mod interface;
mod process_requests;
mod set_requests;
mod traits;
mod utils;

pub fn to_js<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    info!("WASM tracing initialized")
}

#[wasm_bindgen]
pub struct JsInterface {
    inner: Interface,
}

impl Default for JsInterface {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsInterface {
        JsInterface {
            inner: Interface::default(),
        }
    }
}
