use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ::js_sys::Object, js_name = document , typescript_type = "Document")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Document` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub type Document;
    # [wasm_bindgen (extends = ::js_sys::Object , js_name = Element , typescript_type = "Element")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Element` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    pub type Element;
    #[wasm_bindgen (static_method_of = Document, js_class = "document" , js_name = getElementById)]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn get_element_by_id(element_id: &str) -> Option<Element>;
}
