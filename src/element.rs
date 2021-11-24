use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    pub type Element;
    #[wasm_bindgen(structural, method, getter = r#type)]
    pub fn kind(this: &Element) -> JsValue;

    #[wasm_bindgen(js_namespace = React, js_name = createElement)]
    pub fn create_element_no_props(element_type: &JsValue) -> Element;

    #[wasm_bindgen(js_namespace = React, js_name = createElement)]
    pub fn create_element_no_children(element_type: &JsValue, props: &JsValue) -> Element;

    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    pub fn create_element(
        element_type: &JsValue,
        props: &JsValue,
        children: &js_sys::Array,
    ) -> Element;

    /// # Safety
    ///
    /// the function in `element_type` closure should live longer enough
    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    pub unsafe fn create_element_fn(
        element_type: &Closure<dyn Fn(js_sys::Object) -> Element>,
        props: &JsValue,
        children: &js_sys::Array,
    ) -> Element;

    /// # Safety
    ///
    /// the function in `element_type` closure should live longer enough
    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    pub unsafe fn create_element_fn_mut(
        element_type: &Closure<dyn FnMut(js_sys::Object) -> Element>,
        props: &JsValue,
        children: &js_sys::Array,
    ) -> Element;
}
