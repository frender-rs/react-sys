use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    pub type Element;
    #[wasm_bindgen(structural, method, getter = r#type)]
    pub fn kind(this: &Element) -> JsValue;

    #[wasm_bindgen(js_namespace = React, js_name = createElement)]
    pub fn create_element_no_props(element_type: &JsValue) -> Element;

    #[wasm_bindgen(js_namespace = React, js_name = createElement)]
    pub fn create_element_with_props(element_type: &JsValue, props: JsValue) -> Element;

    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    pub fn create_element(
        element_type: &JsValue,
        props: JsValue,
        children: js_sys::Array,
    ) -> Element;
}
