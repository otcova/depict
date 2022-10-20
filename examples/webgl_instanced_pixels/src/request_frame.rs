use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "requestAnimationFrame")]
    fn js_request_animation_frame(closure: &JsValue);
}

pub fn start_animation_loop<F: FnMut() + 'static>(mut f: F) {
    let closure = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    *closure.borrow_mut() = Some(
        Closure::new(move || {
            f();
            js_request_animation_frame(closure_clone.borrow().as_ref().unwrap());
        })
        .into_js_value(),
    );

    js_request_animation_frame(closure.borrow().as_ref().unwrap());
}
