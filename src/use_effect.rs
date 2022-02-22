use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    /// `React.useEffect(effect, dependencies)`
    ///
    /// # Safety
    ///
    /// `effect` should be a js function.
    ///
    /// If `effect` is the value returned from `Closure::once_into_js()`.
    /// You need to make sure `effect` changes if and only if
    /// `dependencies` change.
    /// If `dependencies` change but `effect` doesn't change,
    /// `effect` will be called more than once.
    /// If `effect` but `dependencies` doesn't change,
    /// `effect` will never be called causing memory leak.
    ///
    #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
    pub unsafe fn use_effect(effect: JsValue, dependencies: js_sys::Array);

    /// `React.useEffect(effect)`
    ///
    /// # Safety
    ///
    /// `effect` should be a js function.
    ///
    /// It can the value returned from `Closure::once_into_js()`.
    ///
    #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
    pub unsafe fn use_effect_on_each_render(effect: JsValue);
}
