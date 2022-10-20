use crate::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = performance)]
    type Performance;
    #[wasm_bindgen(static_method_of = Performance, js_class = "performance")]
    fn now() -> f64;
}

pub fn seconds() -> f32 {
    (Performance::now() / 1000.) as f32
}

#[derive(Copy, Clone)]
pub struct Chrono {
    start_seconds: f32,
}

impl Chrono {
    pub fn start() -> Self {
        Self {
            start_seconds: seconds(),
        }
    }
    pub fn seconds(&self) -> f32 {
        seconds() - self.start_seconds
    }
}
