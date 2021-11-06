use wasm_bindgen::prelude::*;

#[cfg(feature = "import-react")]
macro_rules! wasm_bindgen_react_dom {
    ($($b:item)+) => {
        #[wasm_bindgen]
        extern "C" {
            $($b)+
        }
    };
}

#[cfg(not(feature = "import-react"))]
macro_rules! wasm_bindgen_react_dom {
    ($($b:item)+) => {
        #[wasm_bindgen(inline_js = r#"export * as ReactDOM from "react-dom";"#)]
        extern "C" {
            $($b)+
        }
    };
}

wasm_bindgen_react_dom! {
    #[wasm_bindgen(js_namespace = ReactDOM)]
    fn render(react_element: &crate::Element, dom_element: &web_sys::Element);
}
