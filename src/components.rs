#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    #[wasm_bindgen(js_namespace = React)]
    static Fragment: JsValue;
    #[wasm_bindgen(js_namespace = React)]
    static StrictMode: JsValue;
}
