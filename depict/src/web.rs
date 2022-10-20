use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "addEventListener")]
    fn js_add_event_listener(listener_type: &str, listener: JsValue);

    #[wasm_bindgen(js_name = document)]
    pub type Document;
    #[wasm_bindgen(getter, static_method_of=Document,js_class="document", js_name="visibilityState")]
    pub fn visibility_state() -> String;
}

pub fn add_event_listener<F: FnMut() + 'static>(listener_type: &str, listener: F) {
    js_add_event_listener(listener_type, Closure::new(listener).into_js_value());
}
