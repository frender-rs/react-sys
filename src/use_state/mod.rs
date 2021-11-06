mod macro_helpers;

mod helpers;
pub use helpers::*;

use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    #[wasm_bindgen(js_namespace = React, js_name = useState)]
    pub fn use_state(initial_state: JsValue) -> Box<[JsValue]>;

    #[wasm_bindgen(js_namespace = React, js_name = useState)]
    pub fn use_state_with(initial_state: &mut dyn FnMut() -> JsValue) -> Box<[JsValue]>;
}
