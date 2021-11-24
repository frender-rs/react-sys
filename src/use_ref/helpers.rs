use wasm_bindgen::prelude::*;

use super::macro_helpers::define_react_use_ref_helpers;

define_react_use_ref_helpers! {
    {
        value: usize,
        object: MutableRefObjectUsize,
        use_ref_usize,
        use_ref_usize_with,
    },
    {
        value: Option<usize>,
        object: MutableRefObjectOptionalUsize,
        use_ref_optional_usize,
        use_ref_optional_usize_with,
    },
    {
        value: bool,
        object: MutableRefObjectBool,
        use_ref_bool,
        use_ref_bool_with,
    },
    {
        value: web_sys::HtmlElement,
        object: MutableRefObjectHtmlElement,
        use_ref_html_element,
        use_ref_html_element_with,
    },
    {
        value: Option<web_sys::HtmlElement>,
        object: MutableRefObjectOptionalHtmlElement,
        use_ref_optional_html_element,
        use_ref_optional_html_element_with,
    },
}
