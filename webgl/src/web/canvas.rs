use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Element, extends = ::js_sys::Object , js_name = HTMLCanvasElement , typescript_type = "HTMLCanvasElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCanvasElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub type HtmlCanvasElement;
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLCanvasElement" , js_name = width)]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn width(this: &HtmlCanvasElement) -> u32;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLCanvasElement" , js_name = width)]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn set_width(this: &HtmlCanvasElement, value: u32);
    # [wasm_bindgen (structural , method , getter , js_class = "HTMLCanvasElement" , js_name = height)]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn height(this: &HtmlCanvasElement) -> u32;
    # [wasm_bindgen (structural , method , setter , js_class = "HTMLCanvasElement" , js_name = height)]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn set_height(this: &HtmlCanvasElement, value: u32);
    # [wasm_bindgen (catch , method , structural , js_class = "HTMLCanvasElement" , js_name = getContext)]
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    pub fn get_context(
        this: &HtmlCanvasElement,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, JsValue>;
}
