use wasm_bindgen::prelude::*;

crate::macro_import::wasm_bindgen_react! {
    /// `React.useEffect(effect, dependencies)`
    ///
    /// # Safety
    ///
    /// `effect` closure will be passed to js runtime
    /// and called ( or never called ) asynchronously by React.
    ///
    /// As [`wasm_bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/reference/passing-rust-closures-to-js.html) describes,
    ///
    /// > Once a Closure is dropped, it will deallocate its internal memory
    /// > and invalidate the corresponding JavaScript function
    /// > so that any further attempts to invoke it raise an exception.
    ///
    /// Rust compiler can only ensure this closure is not dropped before `use_effect` returned,
    /// so it's up to you to make sure the closure lived longer enough
    /// to be valid in the component life time.
    ///
    /// Thus, this function is marked `unsafe` as a warning.
    #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
    pub unsafe fn use_effect(effect: &Closure<dyn FnMut()>, dependencies: Option<Box<[JsValue]>>);

    #[wasm_bindgen(js_namespace = React, js_name = useEffect)]
    pub unsafe fn use_effect_with_clean(
        effect: &Closure<dyn FnMut() -> JsValue>,
        dependencies: Option<Box<[JsValue]>>,
    );
}
