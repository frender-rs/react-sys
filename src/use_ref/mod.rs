use crate::macro_import::wasm_bindgen_react;
use wasm_bindgen::prelude::*;

mod macro_helpers;

mod helpers;
pub use helpers::*;

wasm_bindgen_react! {
    pub type MutableRefObject;
    #[wasm_bindgen(structural, method, getter)]
    pub fn current(this: &MutableRefObject) -> JsValue;
    #[wasm_bindgen(structural, method, setter)]
    pub fn set_current(this: &MutableRefObject, val: JsValue);

    #[wasm_bindgen(js_namespace = React, js_name = useRef)]
    pub fn use_ref(initial_value: &JsValue) -> MutableRefObject;

    /// Closure `get_initial_value` will be called immediately,
    /// thus it is safe to use reference here.
    ///
    /// Closure `get_initial_value` will be called only once or never called
    #[wasm_bindgen(js_namespace = React, js_name = useRef)]
    pub fn use_ref_with(
        get_initial_value: &mut dyn FnMut() -> JsValue,
    ) -> MutableRefObject;
}
