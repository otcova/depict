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

// pub fn time<F: FnOnce()>(f: F) -> f32 {
//     let start: f64 = current_ms();
//     f();
//     (current_ms() - start) as f32
// }

pub struct Chrono {
    start_seconds: f32,
    seconds: f32,
}

impl Default for Chrono {
    fn default() -> Self {
        Self {
            start_seconds: seconds(),
            seconds: 0.,
        }
    }
}

impl Chrono {
    pub fn update(&mut self) {
        self.seconds = seconds() - self.start_seconds;
    }
    pub fn seconds(&self) -> f32 {
        self.seconds
    }
}