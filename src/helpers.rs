use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_helpers! {
    #[wasm_bindgen(js_name = create_fragment)]
    pub fn create_fragment_no_props() -> crate::Element;

    #[wasm_bindgen(js_name = create_fragment)]
    pub fn create_fragment_no_children(props: &JsValue) -> crate::Element;

    #[wasm_bindgen(variadic)]
    pub fn create_fragment(
        props: &JsValue,
        children: &js_sys::Array,
    ) -> crate::Element;
}
