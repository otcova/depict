use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (getter, js_namespace = window, js_name = innerWidth)]
    #[doc = "Getter for the `innerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_width() -> JsValue;
    # [wasm_bindgen (getter, js_namespace = window, js_name = innerHeight)]
    #[doc = "Getter for the `innerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    pub fn inner_height() -> JsValue;
}
